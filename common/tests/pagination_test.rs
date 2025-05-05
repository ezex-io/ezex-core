use diesel::{Connection, PgConnection, RunQueryDsl, sql_query};
use dotenv::dotenv;
use std::env;

fn connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connnecting to databse")
}

#[test]
fn test_load_and_count_pages() {
    let mut conn = connection();

    sql_query("CREATE TEMPORARY TABLE users (id serial PRIMARY KEY, name VARCHAR(255))")
        .execute(&mut conn)
        .unwrap();
    sql_query(
        "INSERT INTO users (name) VALUES ('Ali'), ('Hossein'), ('Naghi'), ('Akbar'), ('Hasan')",
    )
    .execute(&mut conn)
    .unwrap();

    let base_query = users::table.select(users::name);
    let (results, total_pages) = base_query
        .paginate(1)
        .per_page(2)
        .load_and_count_pages::<String>(&mut conn)
        .unwrap();

    assert_eq!(results.len(), 2); // First page => 2 users
    assert_eq!(total_pages, 2); // 3 users / 2 per page => 2 pages
}
