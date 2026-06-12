use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use thiserror::Error;
use tokio::sync::Mutex;
use wasmtime::{
    Config, Engine, Store,
    component::{Component, Linker, ResourceTable},
};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

use crate::bindings::{
    Plugin,
    exports::dxps::service_plugins::plugin_api::{
        PluginMetadata as WitPluginMetadata, Request, Response,
    },
};

#[derive(Clone)]
pub struct PluginManager {
    inner: Arc<Mutex<PluginCatalog>>,
}

struct PluginCatalog {
    engine: Engine,
    records: BTreeMap<String, PluginRecord>,
}

struct PluginRecord {
    id: String,
    wasm_path: PathBuf,
    config: serde_json::Value,
    state: PluginState,
}

enum PluginState {
    Registered,
    Loaded(Box<LoadedPlugin>),
}

struct LoadedPlugin {
    store: Store<HostState>,
    bindings: Plugin,
    metadata: WitPluginMetadata,
}

struct HostState {
    table: ResourceTable,
    wasi: WasiCtx,
}

impl HostState {
    fn new() -> Self {
        Self {
            table: ResourceTable::new(),
            wasi: WasiCtxBuilder::new().build(),
        }
    }
}

impl WasiView for HostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginSummary {
    pub id: String,
    pub wasm_path: PathBuf,
    pub enabled: bool,
    pub config: serde_json::Value,
    pub metadata: Option<PluginMetadata>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterPlugin {
    pub id: String,
    pub wasm_path: PathBuf,
    #[serde(default)]
    pub config: serde_json::Value,
    #[serde(default)]
    pub load: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePluginConfig {
    #[serde(default)]
    pub config: serde_json::Value,
    #[serde(default)]
    pub reload: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DispatchRequest {
    #[serde(default = "default_method")]
    pub method: String,
    pub path: String,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DispatchResponse {
    pub status: u16,
    pub body: String,
}

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("plugin `{0}` already exists")]
    AlreadyExists(String),
    #[error("plugin `{0}` was not found")]
    NotFound(String),
    #[error("plugin `{0}` is already loaded")]
    AlreadyLoaded(String),
    #[error("plugin `{0}` is not loaded")]
    NotLoaded(String),
    #[error("plugin id must not be empty")]
    EmptyId,
    #[error("plugin component path does not exist: {0}")]
    MissingComponent(PathBuf),
    #[error("plugin `{id}` failed: {source}")]
    Runtime {
        id: String,
        #[source]
        source: anyhow::Error,
    },
    #[error("plugin `{id}` rejected configuration: {message}")]
    ConfigRejected { id: String, message: String },
    #[error("plugin `{id}` rejected request: {message}")]
    RequestRejected { id: String, message: String },
}

impl PluginManager {
    pub fn new() -> Result<Self, PluginError> {
        let mut config = Config::new();
        config.wasm_component_model(true);

        let engine = Engine::new(&config).map_err(|source| PluginError::Runtime {
            id: "engine".to_owned(),
            source: source.into(),
        })?;

        Ok(Self {
            inner: Arc::new(Mutex::new(PluginCatalog {
                engine,
                records: BTreeMap::new(),
            })),
        })
    }

    pub async fn list(&self) -> Vec<PluginSummary> {
        let catalog = self.inner.lock().await;
        catalog
            .records
            .values()
            .map(PluginRecord::summary)
            .collect()
    }

    pub async fn get(&self, id: &str) -> Result<PluginSummary, PluginError> {
        let catalog = self.inner.lock().await;
        let record = catalog
            .records
            .get(id)
            .ok_or_else(|| PluginError::NotFound(id.to_owned()))?;
        Ok(record.summary())
    }

    pub async fn register(&self, input: RegisterPlugin) -> Result<PluginSummary, PluginError> {
        validate_id(&input.id)?;
        validate_component_path(&input.wasm_path)?;

        let mut catalog = self.inner.lock().await;
        if catalog.records.contains_key(&input.id) {
            return Err(PluginError::AlreadyExists(input.id));
        }

        let mut record = PluginRecord {
            id: input.id.clone(),
            wasm_path: input.wasm_path,
            config: input.config,
            state: PluginState::Registered,
        };

        if input.load {
            let loaded = load_record(&catalog.engine, &record)?;
            record.state = PluginState::Loaded(Box::new(loaded));
        }

        let summary = record.summary();
        catalog.records.insert(input.id, record);
        Ok(summary)
    }

    pub async fn remove(&self, id: &str) -> Result<PluginSummary, PluginError> {
        let mut catalog = self.inner.lock().await;
        let record = catalog
            .records
            .remove(id)
            .ok_or_else(|| PluginError::NotFound(id.to_owned()))?;
        Ok(record.summary())
    }

    pub async fn load(&self, id: &str) -> Result<PluginSummary, PluginError> {
        let mut catalog = self.inner.lock().await;
        let engine = catalog.engine.clone();
        let record = catalog
            .records
            .get_mut(id)
            .ok_or_else(|| PluginError::NotFound(id.to_owned()))?;

        if matches!(record.state, PluginState::Loaded(_)) {
            return Err(PluginError::AlreadyLoaded(id.to_owned()));
        }

        let loaded = load_record(&engine, record)?;
        record.state = PluginState::Loaded(Box::new(loaded));
        Ok(record.summary())
    }

    pub async fn unload(&self, id: &str) -> Result<PluginSummary, PluginError> {
        let mut catalog = self.inner.lock().await;
        let record = catalog
            .records
            .get_mut(id)
            .ok_or_else(|| PluginError::NotFound(id.to_owned()))?;

        if !matches!(record.state, PluginState::Loaded(_)) {
            return Err(PluginError::NotLoaded(id.to_owned()));
        }

        record.state = PluginState::Registered;
        Ok(record.summary())
    }

    pub async fn update_config(
        &self,
        id: &str,
        input: UpdatePluginConfig,
    ) -> Result<PluginSummary, PluginError> {
        let mut catalog = self.inner.lock().await;
        let engine = catalog.engine.clone();
        let record = catalog
            .records
            .get_mut(id)
            .ok_or_else(|| PluginError::NotFound(id.to_owned()))?;

        record.config = input.config;
        match &mut record.state {
            PluginState::Registered => {
                if input.reload {
                    let loaded = load_record(&engine, record)?;
                    record.state = PluginState::Loaded(Box::new(loaded));
                }
            }
            PluginState::Loaded(_) => {
                if input.reload {
                    let loaded = load_record(&engine, record)?;
                    record.state = PluginState::Loaded(Box::new(loaded));
                } else {
                    configure_loaded(record)?;
                }
            }
        }

        Ok(record.summary())
    }

    pub async fn dispatch(
        &self,
        id: &str,
        input: DispatchRequest,
    ) -> Result<DispatchResponse, PluginError> {
        let mut catalog = self.inner.lock().await;
        let record = catalog
            .records
            .get_mut(id)
            .ok_or_else(|| PluginError::NotFound(id.to_owned()))?;

        let PluginState::Loaded(loaded) = &mut record.state else {
            return Err(PluginError::NotLoaded(id.to_owned()));
        };

        let req = Request {
            method: input.method,
            path: input.path,
            body: input.body,
        };

        let response = loaded
            .bindings
            .dxps_service_plugins_plugin_api()
            .call_handle(&mut loaded.store, &req)
            .map_err(|source| PluginError::Runtime {
                id: id.to_owned(),
                source: source.into(),
            })?
            .map_err(|message| PluginError::RequestRejected {
                id: id.to_owned(),
                message,
            })?;

        Ok(response.into())
    }
}

fn load_record(engine: &Engine, record: &PluginRecord) -> Result<LoadedPlugin, PluginError> {
    validate_component_path(&record.wasm_path)?;

    let component =
        Component::from_file(engine, &record.wasm_path).map_err(|source| PluginError::Runtime {
            id: record.id.clone(),
            source: source.into(),
        })?;

    let mut linker = Linker::new(engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker).map_err(|source| PluginError::Runtime {
        id: record.id.clone(),
        source: source.into(),
    })?;

    let mut store = Store::new(engine, HostState::new());
    let bindings = Plugin::instantiate(&mut store, &component, &linker).map_err(|source| {
        PluginError::Runtime {
            id: record.id.clone(),
            source: source.into(),
        }
    })?;

    let metadata = bindings
        .dxps_service_plugins_plugin_api()
        .call_metadata(&mut store)
        .map_err(|source| PluginError::Runtime {
            id: record.id.clone(),
            source: source.into(),
        })?;

    let mut loaded = LoadedPlugin {
        store,
        bindings,
        metadata,
    };

    configure_loaded_state(&record.id, &record.config, &mut loaded)?;
    Ok(loaded)
}

fn configure_loaded(record: &mut PluginRecord) -> Result<(), PluginError> {
    let PluginState::Loaded(loaded) = &mut record.state else {
        return Err(PluginError::NotLoaded(record.id.clone()));
    };

    configure_loaded_state(&record.id, &record.config, loaded)
}

fn configure_loaded_state(
    id: &str,
    config: &serde_json::Value,
    loaded: &mut LoadedPlugin,
) -> Result<(), PluginError> {
    let config_json = serde_json::to_string(config).map_err(|source| PluginError::Runtime {
        id: id.to_owned(),
        source: source.into(),
    })?;

    loaded
        .bindings
        .dxps_service_plugins_plugin_api()
        .call_configure(&mut loaded.store, &config_json)
        .map_err(|source| PluginError::Runtime {
            id: id.to_owned(),
            source: source.into(),
        })?
        .map_err(|message| PluginError::ConfigRejected {
            id: id.to_owned(),
            message,
        })
}

fn validate_id(id: &str) -> Result<(), PluginError> {
    if id.trim().is_empty() {
        return Err(PluginError::EmptyId);
    }
    Ok(())
}

fn validate_component_path(path: &Path) -> Result<(), PluginError> {
    if !path.exists() {
        return Err(PluginError::MissingComponent(path.to_path_buf()));
    }
    Ok(())
}

fn default_method() -> String {
    "GET".to_owned()
}

impl PluginRecord {
    fn summary(&self) -> PluginSummary {
        let (enabled, metadata) = match &self.state {
            PluginState::Registered => (false, None),
            PluginState::Loaded(loaded) => (true, Some(loaded.metadata.clone().into())),
        };

        PluginSummary {
            id: self.id.clone(),
            wasm_path: self.wasm_path.clone(),
            enabled,
            config: self.config.clone(),
            metadata,
        }
    }
}

impl From<WitPluginMetadata> for PluginMetadata {
    fn from(value: WitPluginMetadata) -> Self {
        Self {
            name: value.name,
            version: value.version,
            description: value.description,
        }
    }
}

impl From<Response> for DispatchResponse {
    fn from(value: Response) -> Self {
        Self {
            status: value.status,
            body: value.body,
        }
    }
}
