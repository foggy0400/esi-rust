use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

#[derive(PartialEq, Clone, Copy)]
/// Enum representing possible ESI scopes
pub enum ScopeList {
    ReadStructures,
    SearchStructures,
    StructureMarkets,
}

impl Display for ScopeList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeList::ReadStructures => write!(f, "esi-universe.read_structures.v1"),
            ScopeList::SearchStructures => write!(f, "esi-search.search_structures.v1"),
            ScopeList::StructureMarkets => write!(f, "esi-markets.structure_markets.v1"),
        }
    }
}

impl Ord for ScopeList {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl PartialOrd for ScopeList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for ScopeList {}

pub struct ScopeManager {
    scopes: Vec<ScopeList>,
    scope_str: String,
}

impl ScopeManager {
    pub fn new() -> ScopeManager {
        ScopeManager {
            scopes: vec![],
            scope_str: String::new(),
        }
    }
    pub fn get_scope_string(&self) -> &str {
        &self.scope_str
    }
    fn update_scope_string(&mut self) {
        let mut fmt_string = String::from("");
        for scope in &self.scopes {
            fmt_string.push_str(&scope.to_string());
            fmt_string.push_str(" ");
        }
        self.scope_str = String::from(fmt_string.trim())
    }
    pub fn add_scope(&mut self, scope: ScopeList) -> Result<usize, &str> {
        match self.scopes.binary_search(&scope) {
            Ok(_) => Err("Scope already in array"),
            Err(idx) => {
                self.scopes.insert(idx, scope);
                self.update_scope_string();
                Ok(idx)
            }
        }
    }
    fn add_scope_noup(&mut self, scope: ScopeList) -> Result<usize, &str> {
        match self.scopes.binary_search(&scope) {
            Ok(_) => Err("Scope already in array"),
            Err(idx) => {
                self.scopes.insert(idx, scope);
                Ok(idx)
            }
        }
    }
    pub fn add_scopes(&mut self, scopes: Vec<ScopeList>) -> (Vec<ScopeList>, Vec<ScopeList>) {
        let mut suc_v = vec![];
        let mut err_v = vec![];
        for scope in scopes {
            match self.add_scope_noup(scope.clone()) {
                Err(_) => err_v.push(scope),
                Ok(_) => suc_v.push(scope),
            }
        }
        self.update_scope_string();
        (suc_v, err_v)
    }
    fn remove_scope_noup(&mut self, scope: ScopeList) -> Result<ScopeList, &str> {
        match self.scopes.binary_search(&scope) {
            Ok(idx) => Ok(self.scopes.remove(idx)),
            Err(_) => Err("Scope not in array"),
        }
    }
    pub fn remove_scope(&mut self, scope: ScopeList) -> Result<ScopeList, &str> {
        match self.scopes.binary_search(&scope) {
            Ok(idx) => {
                self.update_scope_string();
                Ok(self.scopes.remove(idx))
            }
            Err(_) => Err("Scope not in array"),
        }
    }
    pub fn remove_scopes(&mut self, scopes: Vec<ScopeList>) -> (Vec<ScopeList>, Vec<ScopeList>) {
        let mut suc_v = vec![];
        let mut err_v = vec![];
        for scope in scopes {
            match self.remove_scope_noup(scope.clone()) {
                Err(_) => err_v.push(scope),
                Ok(_) => suc_v.push(scope),
            }
        }
        self.update_scope_string();
        (suc_v, err_v)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::scopes::{ScopeList, ScopeManager};

    #[test]
    fn scope_string() {
        let mut client = ScopeManager {
            scopes: vec![ScopeList::ReadStructures],
            scope_str: String::new(),
        };
        client.update_scope_string();
        assert_eq!(
            client.get_scope_string(),
            ScopeList::ReadStructures.to_string()
        );
    }
    #[test]
    fn scope_string_multiple_elements() {
        let mut client = ScopeManager {
            scopes: vec![ScopeList::SearchStructures, ScopeList::ReadStructures],
            scope_str: String::new(),
        };
        client.update_scope_string();
        assert_eq!(
            client.get_scope_string(),
            format!(
                "{} {}",
                ScopeList::SearchStructures,
                ScopeList::ReadStructures
            )
        );
    }
    #[test]
    fn scope_string_empty() {
        let client = ScopeManager {
            scopes: vec![],
            scope_str: String::new(),
        };
        assert_eq!(client.get_scope_string(), "");
    }
    #[test]
    fn append_scopes() {
        let mut client = ScopeManager::new();
        assert_eq!(client.get_scope_string(), "");
        let _ = client.add_scope(ScopeList::SearchStructures);
        assert_eq!(client.get_scope_string(), "esi-search.search_structures.v1");
    }
    #[test]
    fn append_duplicate_scopes() {
        let mut client = ScopeManager::new();
        assert_eq!(client.get_scope_string(), "");
        let _ = client.add_scope(ScopeList::SearchStructures);
        assert_eq!(client.get_scope_string(), "esi-search.search_structures.v1");
        let _ = client.add_scope(ScopeList::ReadStructures);
        assert_eq!(
            client.get_scope_string(),
            "esi-search.search_structures.v1 esi-universe.read_structures.v1"
        );
        let _ = client.add_scope(ScopeList::SearchStructures);
        assert_eq!(
            client.get_scope_string(),
            "esi-search.search_structures.v1 esi-universe.read_structures.v1"
        );
    }
}
