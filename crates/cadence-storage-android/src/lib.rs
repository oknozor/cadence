use dioxus::prelude::debug;
use dioxus_sdk::storage::StorageBacking;
use once_cell::sync::Lazy;
use redb::{ReadableTable, TableDefinition};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::sync::Arc;
use thiserror::Error;

const COLLECTION: &str = "cadence";

static DATABASE: Lazy<LocalStorage> =
    Lazy::new(|| init_database().expect("Failed to initialize mobile database"));

#[derive(Debug, Error)]
enum StorageError {
    #[error("Jni error {0}")]
    Jni(#[from] jni::errors::Error),
    #[error("database error {0}")]
    Db(#[from] redb::Error),
    #[error("database error {0}")]
    DbCommit(#[from] redb::CommitError),
    #[error("database error {0}")]
    DbTransaction(#[from] redb::TransactionError),
    #[error("database error {0}")]
    DbTable(#[from] redb::TableError),
    #[error("database error {0}")]
    DbStorage(#[from] redb::StorageError),
}

#[derive(Clone, Debug)]
pub struct LocalStorage {
    inner: Arc<redb::Database>,
}

impl StorageBacking for LocalStorage {
    type Key = String;

    fn get<T: DeserializeOwned + Clone + 'static>(key: &Self::Key) -> Option<T> {
        DATABASE.get(key)
    }
    fn set<T: Serialize + Send + Sync + Clone + 'static>(key: Self::Key, value: &T) {
        DATABASE.insert(&key, value).expect("failed to insert data");
    }
}

fn init_database() -> Result<LocalStorage, StorageError> {
    #[cfg(target_os = "android")]
    let path = {
        let path = internal_storage_dir();
        path.join("cadence-db")
    };

    #[cfg(not(target_os = "android"))]
    let path = {
        let path = dirs::data_dir().expect("Failed to get data directory");
        path.join("cadence-db")
    };

    debug!("Opening database {:?}", path);

    let database = match redb::Database::open(&path) {
        Ok(db) => db,
        Err(_err) => redb::Database::create(path).expect("failed to create database"),
    };

    Ok(LocalStorage {
        inner: Arc::new(database),
    })
}

impl LocalStorage {
    fn insert<T>(&self, key: &str, value: T) -> Result<(), StorageError>
    where
        T: Sized + Serialize,
    {
        let json = serde_json::to_string(&value).expect("Serialization error");
        let db = self.inner.clone();
        let write_tnx = db.begin_write()?;
        {
            let definition = TableDefinition::<&str, &str>::new(COLLECTION);
            let mut table = write_tnx.open_table(definition)?;
            table.insert(key, json.as_str())?;
        }
        write_tnx.commit()?;
        Ok(())
    }

    fn get<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned,
    {
        let definition = TableDefinition::<&str, &str>::new(COLLECTION);
        let db = self.inner.clone();
        let Ok(read_txn) = db.begin_write() else {
            return None;
        };

        let table = read_txn
            .open_table(definition)
            .expect("failed to open database");

        table
            .get(key)
            .ok()
            .flatten()
            .map(|data| serde_json::from_str(data.value()))
            .and_then(Result::ok)
    }
}

#[cfg(target_os = "android")]
pub fn internal_storage_dir() -> std::path::PathBuf {
    use jni::JNIEnv;
    use jni::objects::{JObject, JString};
    use std::path::PathBuf;

    let (tx, rx) = std::sync::mpsc::channel();

    fn run(env: &mut JNIEnv<'_>, activity: &JObject<'_>) -> Result<PathBuf, StorageError> {
        let files_dir = env
            .call_method(activity, "getFilesDir", "()Ljava/io/File;", &[])?
            .l()?;
        let files_dir: JString<'_> = env
            .call_method(files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])?
            .l()?
            .into();
        let files_dir: String = env.get_string(&files_dir)?.into();
        Ok(PathBuf::from(files_dir))
    }

    dioxus::mobile::wry::prelude::dispatch(move |env, activity, _webview| {
        tx.send(run(env, activity).unwrap()).unwrap()
    });

    rx.recv().unwrap()
}
