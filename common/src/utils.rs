use anyhow;
use std::panic;
use std::process;

pub fn value_or_error(value: &serde_json::Value, key: &str) -> anyhow::Result<serde_json::Value> {
    match value.get(key) {
        Some(value) => Ok(value.to_owned()),
        None => anyhow::bail!("Unable to find '{}' in '{}'", key, value),
    }
}

pub fn exit_on_panic() {
    // take_hook() returns the default hook in case when a custom one is not set
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        log::error!("Exiting On : {:#?}", panic_info);
        orig_hook(panic_info);
        process::exit(1);
    }));
}

pub fn error_to_tonic_status(e: anyhow::Error) -> tonic::Status {
    tonic::Status::internal(e.to_string())
}

pub fn coin_to_chain_id(coin: &str) ->Option<String>  {
    todo!()
}
