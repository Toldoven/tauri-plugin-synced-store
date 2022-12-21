pub(crate) mod saveable_state;
pub mod synced_state;
pub(crate) mod utils;
pub mod inits;
pub mod synced_state_toml;

use std::path::Path;

use inits::{StateManage, StateInit, StateSave, StateTomlInit};
use serde::{Serialize, Deserialize};
pub use synced_state::Synced;
pub use synced_state_toml::SyncedToml;
use tauri::{plugin::{self, TauriPlugin}, RunEvent, Wry, State};

pub type SyncState<'a, T> = State<'a, Synced<T>>;
pub type SyncStateToml<'a, T> = State<'a, SyncedToml<T>>;


pub struct PluginBuilder {
    states_manage: Vec<Box<dyn StateManage + Sync + Send>>,
    states_save: Vec<Box<dyn StateSave + Sync + Send>>
}

impl PluginBuilder {

    pub fn new() -> Self {
        Self {
            states_manage: Vec::new(),
            states_save: Vec::new(),
        }
    }

    pub fn manage<T>(mut self, key: impl Into<String>) -> Self
    where T: Default + Serialize + for<'a> Deserialize<'a> + Clone + Sync + Send + 'static
    {   
        let state = StateInit::<T>::new(key);
        self.states_manage.push(
            Box::new(state)
        );
        self
    }

    pub fn manage_toml<T>(
        mut self,
        key: impl Into<String>,
        path: impl AsRef<Path>,
    ) -> Self
    where T: Default + Serialize + for<'a> Deserialize<'a> + Clone + Sync + Send + 'static
    {   
        let state = StateTomlInit::<T>::new(key, path);
        self.states_manage.push(
            Box::new(state.clone())
        );
        self.states_save.push(
            Box::new(state)
        );
        self
    }

    pub fn build(self) -> TauriPlugin<Wry> {
        plugin::Builder::new("synced_state")
            .setup(move |handle| {

                self.states_manage.iter().for_each(|state| {
                    state.manage(handle);
                });

                Ok(())
            })
            .on_event(move |handle, event| {

                let RunEvent::Exit = event else { return };
                
                // let states_save = self.states_save.cln

                self.states_save.iter().for_each(|state| {
                    let result = state.save(handle);

                    if let Err(error) = result {
                        eprintln!("Error while saving state: {error}")
                    }
                });

            })
            .build()
    }
}