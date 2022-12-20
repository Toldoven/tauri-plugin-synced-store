use std::{borrow::Borrow, sync::{Arc}};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tokio::{sync::{Mutex}};

pub struct SyncedState<T> {
    pub(crate) key: String,
    pub(crate) state: Arc<Mutex<T>>,
    pub(crate) handle: AppHandle
}

impl<T> SyncedState<T>
where T: Default + Serialize + for<'a> Deserialize<'a> + Clone
{
    pub async fn init(
        key: impl Into<String>,
        handle: impl Borrow<AppHandle>
    ) -> Self {

        let handle = handle.borrow();
        let key: String = key.into();

        let state = T::default();

        Self {
            key,
            state: Arc::new(Mutex::new(
                state
            )),
            handle: handle.clone(),
        }
    }

    pub fn init_sync(
        key: impl Into<String>,
        handle: impl Borrow<AppHandle>
    ) -> Self {
        tokio::task::block_in_place(|| {
            tauri::async_runtime::block_on(Self::init(key, handle))
        })
    }

    fn emit_update(&self, payload: T) {
        let key = &self.key;
        let handle = &self.handle;
        let event = format!("synced-state://{key}-update");

        handle
            .emit_all(event.as_str(), payload)
            .ok();
    }

    pub async fn mutate(
        &self,
        mut function: impl FnMut(&mut T)
    ) {
        let mut state = self.state.lock().await;

        function(&mut state);

        self.emit_update(state.to_owned());
    }

    pub async fn get(&self) -> T {
        let lock = self.state.lock().await;
        lock.clone()
    }

    pub async fn set(&self, new_value: T) {
        self.mutate(|value| {
            *value = new_value.clone();
        }).await;
    }
}