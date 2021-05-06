use std::collections::HashMap;

use log::error;

use crate::response::status_code::StatusCode;

#[derive(Debug)]
pub enum CallbackError {
    AlreadyRegistered,
    NoCallbackForPath,
}

pub struct CallbackHandler {
    callbacks: HashMap<String, Box<dyn Fn() -> StatusCode + Sync + Send>>,
}

impl Default for CallbackHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl CallbackHandler {
    pub fn new() -> CallbackHandler {
        CallbackHandler { callbacks: Default::default() }
    }

    pub fn register(&mut self, path: &str, func: Box<dyn Fn() -> StatusCode + Sync + Send>) -> Result<(), CallbackError> {
        if self.callbacks.contains_key(path) {
            return Err(CallbackError::AlreadyRegistered);
        }

        self.callbacks.insert(String::from(path), func);

        Ok(())
    }

    pub fn unregister(&mut self, path: &str) -> Result<(), CallbackError> {
        match self.callbacks.remove(path) {
            Some(_func) => Ok(()),
            None => Err(CallbackError::NoCallbackForPath)
        }
    }


    pub fn handle(&self, path: &str) -> Result<StatusCode, CallbackError> {
        match self.callbacks.get(path) {
            None => {
                error!("No callback found for path {}", path);
                Err(CallbackError::NoCallbackForPath)
            }
            Some(func) => Ok(func())
        }
    }
}

