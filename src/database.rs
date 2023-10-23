use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

extern crate tokio;



#[derive(Debug, PartialEq)]
enum AsyncDatabaseError {
    KeyNotFound,
    #[allow(dead_code)]
    OtherError,
}

#[derive(Debug, Clone)]
pub(crate) struct AsyncDatabase {
    inner: Arc<Mutex<BTreeMap<i128, String>>>,
}

#[allow(non_snake_case)]
impl AsyncDatabase {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    async fn get(&self, key: i128) -> Result<Option<String>, AsyncDatabaseError> {
        let lock = self.inner.lock().unwrap();
        match lock.get(&key) {
            Some(value) => Ok(Some(value.clone())),
            None => Err(AsyncDatabaseError::KeyNotFound),
        }
    }

    #[allow(dead_code)]
    async fn set(&self, key: i128, value: String) {
        let mut lock = self.inner.lock().unwrap();
        lock.insert(key, value);
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use tokio::test;

    #[tokio::test]
    async fn given_nonexistent_key_get_value_returns_error() {
        let database = AsyncDatabase::new();

        // Get the value for a non-existent key.
        let result = database.get(123).await;

        // Assert that the value is an error.
        let error = result.unwrap_err();
        assert_eq!(error, AsyncDatabaseError::KeyNotFound);
    }

    #[tokio::test]
    async fn test_get() {
        let database = AsyncDatabase::new();
        let num = 1i128;
        let val = "value1".to_string();

        // Set a value in the database.
        database.set(1i128, "value1".to_string()).await;

        // Get the value from the database.
        let value =  database.get(1i128).await;

        // Assert that the value is correct.
        assert_eq!(value.unwrap(), Some(val));
    }
}
