use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password(String);

impl Drop for Password {
    fn drop(&mut self) {
        unsafe {
            self.0.as_bytes_mut().fill(0);
        }
    }
}

impl From<String> for Password {
    fn from(s: String) -> Self {
        Password(s)
    }
}

impl AsRef<String> for Password {
    fn as_ref(&self) -> &String {
        &self.0
    }
}
