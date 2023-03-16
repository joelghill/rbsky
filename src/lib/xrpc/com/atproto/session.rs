//! Module related to session-management in ATP.
use crate::xrpc::Error;
use serde::{Deserialize, Serialize};

/// Stuct respresenting an AT Protocol Session
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionState {
    did: String,
    handle: String,
    access_jwt: String,
    refresh_jwt: String,
}

pub trait SessionManager {
    /// Implementation of com.atproto.session.create method
    /// Creates an authentication session. 
    /// #### Arguments
    ///
    /// * `handle` - A string slice containging the user's handle (email, username, did)
    /// * `password` - A string slice containing the user's password
    fn create(&self, handle: &str, password: &str) -> Result<SessionState, Error>;
}
