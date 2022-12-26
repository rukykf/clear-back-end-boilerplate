use anyhow::Result;

use super::{Pool, PooledConnection};

/// API context type. This is a public type, since
/// is needs to be accessed and constructed in the
/// graphql api filters.
#[derive(Clone)]
pub struct Context {
    pl: Pool,
}

impl Context {
    pub fn new(pl: Pool) -> Self {
        Self { pl }
    }
    /// Replaces pl.get()
    pub async fn conn(&self) -> Result<PooledConnection> {
        Ok(self.pl.acquire().await?)
    }

    pub fn say_hello(&self) -> String {
        String::from("Hello World")
    }
}
