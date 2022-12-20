use std::{marker::PhantomData, path::{PathBuf, Path}};

use serde::{Serialize, Deserialize};
use tauri::{Manager, AppHandle};

use crate::{synced_state::{SyncedState}, synced_state_toml::SyncedStateToml};

use anyhow::Result;

pub(crate) trait StateManage {
    fn manage(&self, app: &AppHandle);
}

pub(crate) trait StateSave {
    fn save(&self, handle: &AppHandle) -> Result<()>;
}

pub(crate) struct StateInit<T>
where T: Default + Serialize + for<'a> Deserialize<'a> + Clone + Sync + Send + 'static
{
    key: String,
    phantom: PhantomData<T>
}

impl<T> StateInit<T>
where T: Default + Serialize + for<'a> Deserialize<'a> + Clone + Sync + Send + 'static
{
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            phantom: PhantomData
        }
    }
}

impl<T> StateManage for StateInit<T>
where T: Default + Serialize + for<'a> Deserialize<'a> + Clone + Sync + Send + 'static
{
    fn manage(&self, handle: &AppHandle) {
        let state = SyncedState::<T>::init_sync(&self.key, handle);
        handle.manage(state);
    }
}

#[derive(Clone)]
pub struct StateTomlInit<T>
where T: Default + Serialize + for<'a> Deserialize<'a> + Clone + Sync + Send + 'static
{
    key: String,
    path: PathBuf,
    phantom: PhantomData<T>
}

impl<T> StateTomlInit<T>
where T: Default + Serialize + for<'a> Deserialize<'a> + Clone + Sync + Send + 'static
{
    pub fn new(
        key: impl Into<String>,
        path: impl AsRef<Path>
    ) -> Self {
        let path = path.as_ref();
        Self {
            key: key.into(),
            path: PathBuf::from(path),
            phantom: PhantomData
        }
    }
}

impl<T> StateManage for StateTomlInit<T>
where T: Default + Serialize + for<'a> Deserialize<'a> + Clone + Sync + Send + 'static
{
    fn manage(&self, handle: &AppHandle) {
        let state = SyncedStateToml::<T>::init_sync(
            &self.key,
            &self.path,
            handle
        );
        handle.manage(state);
    }
}

impl<T> StateSave for StateTomlInit<T>
where T: Default + Serialize + for<'a> Deserialize<'a> + Clone + Sync + Send + 'static
{
    fn save(&self, handle: &AppHandle) -> Result<()> {
        let state = handle.state::<SyncedStateToml<T>>();
        state.save_sync()
    }
}