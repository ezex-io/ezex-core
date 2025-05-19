/// # Configuration Registration Macro
///
/// This macro registers a configuration type with the global config registry.
/// It enables automatic environment variable prefixing for the registered type.
///
/// ## Usage
///
/// ```rust
/// use common::register_config;
///
/// #[derive(Debug, Clone, Args, EnvPrefix)]
/// #[prefix = "MY_APP"]
/// pub struct Config {
///     #[arg(long, env = "SERVER_PORT")]
///     pub port: u16,
/// }
///
/// // Register this config type
/// register_config!(Config);
/// ```
///
/// ## How It Works
///
/// The macro:
/// 1. Implements the `RegisterableConfig` trait for your config type
/// 2. Creates a function to register the config with the global registry
/// 3. Submits the registration function to the inventory system
///
/// ## Benefits
///
/// - Automatic environment variable prefixing
/// - No need to manually call `prepend_envs()` for each config type
/// - Centralized application of prefixes
///
/// ## Requirements
///
/// The config type must:
/// - Derive the `EnvPrefix` macro
/// - Have a `prepend_envs()` method (automatically provided by `EnvPrefix`)
#[macro_export]
macro_rules! register_config {
    ($config_type:ty) => {
        const _: () = {
            // Function to register this config
            fn register_config_fn() {
                <$config_type as $crate::config_registry::RegisterableConfig>::register_self();
            }

            // Implement the trait
            impl $crate::config_registry::RegisterableConfig for $config_type {
                fn register_self() {
                    let mut registry = $crate::config_registry::global_registry().lock().unwrap();
                    registry.register::<Self>(Self::prepend_envs);
                }
            }

            // Submit to inventory
            inventory::submit! {
                $crate::config_registry::ConfigRegistrationItem {
                    register_fn: register_config_fn
                }
            }
        };
    };
}
