use std::{
    any::TypeId,
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

type PrependEnvsFn = fn();

// Thread-safe registry using Mutex
pub struct ConfigRegistry {
    configs: HashMap<TypeId, PrependEnvsFn>,
}

impl ConfigRegistry {
    fn new() -> Self {
        ConfigRegistry {
            configs: HashMap::new(),
        }
    }

    // Register a config type
    pub fn register<T: 'static>(&mut self, prepend_envs: PrependEnvsFn) {
        self.configs.insert(TypeId::of::<T>(), prepend_envs);
    }

    // Apply all registered prefixes
    pub fn apply_all_prefixes(&self) {
        for prepend_fn in self.configs.values() {
            prepend_fn();
        }
    }
}

// Global registry instance using thread-safe OnceLock<Mutex<>>
static REGISTRY: OnceLock<Mutex<ConfigRegistry>> = OnceLock::new();

// Function to access the registry
pub fn global_registry() -> &'static Mutex<ConfigRegistry> {
    REGISTRY.get_or_init(|| Mutex::new(ConfigRegistry::new()))
}

// Trait for configs that can register themselves
pub trait RegisterableConfig: 'static {
    fn register_self();
}

// Concrete type for inventory collection
pub struct ConfigRegistrationItem {
    pub register_fn: fn(),
}

// Collect all registered configs
inventory::collect!(ConfigRegistrationItem);

// Function to initialize all configs
pub fn init_all_configs() {
    for item in inventory::iter::<ConfigRegistrationItem> {
        (item.register_fn)();
    }
}
