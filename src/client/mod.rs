use self::scopes::ScopeManager;

pub mod scopes;

/// Main class for handling interaction with the ESI API.
pub struct EsiClient {
    client_id: &'static str,
    pub scopes: ScopeManager,
}

impl EsiClient {
    /// Creates a new EsiClient object.
    ///
    /// # Parameters
    ///
    /// * `client_id` - your program's ESI client ID from the CCP ESI developer webpage
    pub fn new(client_id: &'static str) -> EsiClient {
        EsiClient {
            client_id: client_id,
            scopes: ScopeManager::new(),
        }
    }
    /// Returns the client ID associated with the EsiClient.
    pub fn get_client_id(self) -> &'static str {
        self.client_id
    }
}

#[cfg(test)]
mod tests {
    use super::EsiClient;

    #[test]
    fn new_client() {
        let client = EsiClient::new("testclient");
        assert_eq!(client.scopes.get_scope_string(), "");
        assert_eq!(client.get_client_id(), "testclient");
    }
}
