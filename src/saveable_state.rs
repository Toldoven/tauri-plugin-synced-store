use std::{path::{PathBuf, Path}, ops::Not};

use serde::{Serialize, Deserialize};

use tokio::fs::{write, read};

use anyhow::Result;

use crate::utils::create_dir_all_without_file_name::create_dir_all_without_file_name;

pub struct SaveableToml<T>
where T: Default + Serialize + for<'a> Deserialize<'a>
{
    pub path: PathBuf,
    pub state: T
}

impl<T> SaveableToml<T>
where T: Default + Serialize + for<'a> Deserialize<'a>
{
    pub fn new(
        path: impl AsRef<Path>,
    ) -> Self {
        Self {
            path: PathBuf::from(path.as_ref()),
            state: T::default(),
        }
    }

    pub async fn save(&self) -> Result<()> {
        let path = &self.path;
        
        let string = toml::ser::to_string(&self.state)?;

        create_dir_all_without_file_name(path).await?;

        write(path, string).await?;

        Ok(())
    }

    async fn create_default(
        path: impl AsRef<Path>,
    ) -> Result<()> {

        Self::new(path).save().await?;

        Ok(())
    }

    pub async fn load_path(
        path: impl AsRef<Path>,
    ) -> Result<Self> {

        let path = path.as_ref();

        if path.exists().not() {
            Self::create_default(path).await?;
        }

        let bytes = read(&path).await?;

        let value = match toml::de::from_slice::<T>(&bytes) {
            Ok(session) => session,
            Err(_) => {
                Self::create_default(path).await?;
                toml::de::from_slice::<T>(&bytes)?
            },
        };

        let state = Self {
            path: PathBuf::from(path),
            state: value,
        };

        Ok(state)
    }
}