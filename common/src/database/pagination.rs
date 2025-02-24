use diesel::dsl;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::OrderDsl;
use diesel::query_dsl::LoadQuery;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::sql_types::BigInt;
use diesel::{Queryable, QueryableByName};
use indexmap::IndexMap;
use uuid::Uuid;
pub type AppResult<T> = diesel::result::QueryResult<T>;

#[derive(QueryableByName, Queryable, Debug)]
pub struct WithCount<T> {
    #[diesel(embed)]
    pub record: T,
    #[sql_type = "::diesel::sql_types::BigInt"]
    pub total: i64,
}

pub trait WithCountExtension<T> {
    fn records_and_total(self) -> (Vec<T>, i64);
}

impl<T> WithCountExtension<T> for Vec<WithCount<T>> {
    fn records_and_total(self) -> (Vec<T>, i64) {
        let cnt = self.get(0).map(|row| row.total).unwrap_or(0);
        let vec = self.into_iter().map(|row| row.record).collect();
        (vec, cnt)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PaginationOptions<'a> {
    after_id: Option<&'a String>,
    after_created_at: Option<&'a String>,
    pub page_size: u32,
}

impl<'a> PaginationOptions<'a> {
    pub fn new(params: &'a IndexMap<String, String>) -> AppResult<Self> {
        const DEFAULT_PAGE_SIZE: u32 = 10;
        const MAX_PAGE_SIZE: u32 = 100;

        let page_size = params
            .get("page_size")
            .map(|s| s.parse().unwrap_or(DEFAULT_PAGE_SIZE))
            .unwrap_or(DEFAULT_PAGE_SIZE);

        if page_size > MAX_PAGE_SIZE {
            return Err(diesel::result::Error::QueryBuilderError(
                format!("cannot request more than {} items", MAX_PAGE_SIZE,).into(),
            ));
        }

        Ok(Self {
            after_id: params.get("after_id"),
            after_created_at: params.get("after_created_at"),
            page_size,
        })
    }

    pub fn after(&self) -> (Option<&String>, Option<&String>) {
        (self.after_id, self.after_created_at)
    }
}

pub trait Paginate: Sized {
    fn paginate<O>(
        self,
        cursor: O,
        params: &IndexMap<String, String>,
    ) -> AppResult<PaginatedQuery<dsl::Order<Self, O>, O>>
    where
        Self: OrderDsl<O>,
        O: Clone + Expression,
    {
        let options = PaginationOptions::new(params)?;
        log::trace!("Query Options set to :{:?}", options);
        Ok(PaginatedQuery {
            query: self.order(cursor.clone()),
            order_clause: cursor,
            options,
        })
    }
}

impl<T> Paginate for T {}

#[derive(Debug)]
pub struct Paginated<'a, T> {
    records_and_total: Vec<WithCount<T>>,
    options: PaginationOptions<'a>,
}

impl<'a, T> Paginated<'a, T> {
    pub fn total(&self) -> Option<i64> {
        Some(
            self.records_and_total
                .get(0)
                .map(|row| row.total)
                .unwrap_or_default(),
        )
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.records_and_total.iter().map(|row| &row.record)
    }
}

impl<'a, T: 'static> IntoIterator for Paginated<'a, T> {
    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.records_and_total.into_iter().map(|row| row.record))
    }
}

#[derive(Debug)]
pub struct PaginatedQuery<'a, T, O> {
    query: T,
    order_clause: O,
    options: PaginationOptions<'a>,
}

impl<'a, T, O> PaginatedQuery<'a, T, O> {
    pub fn load<U>(
        self,
        conn: &'a PooledConnection<ConnectionManager<PgConnection>>,
    ) -> QueryResult<Paginated<U>>
    where
        Self: LoadQuery<PooledConnection<ConnectionManager<PgConnection>>, WithCount<U>>,
    {
        let options = self.options;
        let records_and_total = self.internal_load(conn)?;

        Ok(Paginated {
            records_and_total,
            options,
        })
    }
}

impl<'a, T, O> QueryId for PaginatedQuery<'a, T, O> {
    const HAS_STATIC_QUERY_ID: bool = false;
    type QueryId = ();
}

impl<'a, T: Query, O> Query for PaginatedQuery<'a, T, O> {
    type SqlType = (T::SqlType, BigInt);
}

impl<'a, T, O, DB> RunQueryDsl<DB> for PaginatedQuery<'a, T, O> {}

impl<'a, T, O> QueryFragment<Pg> for PaginatedQuery<'a, T, O>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()>{
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t ");
        if let (Some(after_id), Some(after_created_at)) = self.options.after() {
            let after_id: Uuid = after_id.parse().map_err(|e| {
                log::error!("{:#?}", e);
                diesel::result::Error::QueryBuilderError("Couldn't parse page.".into())
            })?;

            out.push_sql(" WHERE (created_at, id) >= (");

            out.push_bind_param::<diesel::sql_types::Timestamptz, _>(
                &after_created_at
                    .parse::<chrono::NaiveDateTime>()
                    .map_err(|e| {
                        log::error!("{:#?}", e);
                        diesel::result::Error::QueryBuilderError("Couldn't parse page.".into())
                    })?,
            )?;
            out.push_sql(", ");
            out.push_bind_param::<diesel::sql_types::Uuid, _>(&after_id)?;

            out.push_sql(")");
        }
        out.push_sql(" LIMIT ");
        out.push_bind_param::<BigInt, _>(&(i64::from(self.options.page_size) + 1))?;
        Ok(())
    }
}
