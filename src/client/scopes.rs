#[derive(PartialEq)]
enum ScopeList {
    ReadStructures,
    SearchStructures,
    StructureMarkets,
}

pub struct EsiScopes {
    scopes: Vec<ScopeList>,
    code: String,
    active: bool,
}

impl EsiScopes {
    fn new() -> EsiScopes {
        EsiScopes {
            scopes: vec![],
            code: String::new(),
            active: false,
        }
    }
    fn update_code(&mut self) {
        self.code = EsiScopes::get_scope_string(&self.scopes);
    }
    fn get_scope_string(scopes: &Vec<ScopeList>) -> String {
        let mut scope_str: String = String::from("");
        for scope in scopes {
            match scope {
                ScopeList::ReadStructures => scope_str.push_str("esi-universe.read_structures.v1"),
                ScopeList::SearchStructures => {
                    scope_str.push_str("esi-search.search_structures.v1")
                }
                ScopeList::StructureMarkets => {
                    scope_str.push_str("esi-markets.structure_markets.v1")
                }
            }
            scope_str.push_str(" ")
        }
        String::from(scope_str.trim())
    }
    fn add_scope(&mut self, scope: ScopeList) {
        if !self.scopes.contains(&scope) {
            self.scopes.push(scope);
        }
        self.update_code();
    }
    fn add_scopes(&mut self, scopes: Vec<ScopeList>) {
        for scope in scopes {
            if !self.scopes.contains(&scope) {
                self.scopes.push(scope);
            }
        }
        self.update_code();
    }
}

#[cfg(test)]
mod tests {
    use crate::client::scopes::{EsiScopes, ScopeList};
    #[test]
    fn scope_string() {
        let scope = vec![ScopeList::ReadStructures];
        assert_eq!(
            EsiScopes::get_scope_string(&scope),
            "esi-universe.read_structures.v1"
        );
    }
    #[test]
    fn scope_string_multiple_elements() {
        let scope = vec![ScopeList::ReadStructures, ScopeList::SearchStructures];
        assert_eq!(
            EsiScopes::get_scope_string(&scope),
            "esi-universe.read_structures.v1 esi-search.search_structures.v1"
        );
    }
    #[test]
    fn scope_string_empty() {
        let scope = vec![];
        assert_eq!(EsiScopes::get_scope_string(&scope), "");
    }
    #[test]
    fn new_scopes() {
        let mut scopes = EsiScopes::new();
        assert_eq!(scopes.active, false);
        assert_eq!(scopes.code, "");
        scopes.update_code();
        assert_eq!(scopes.code, "");
    }
    #[test]
    fn append_scopes() {
        let mut scopes = EsiScopes::new();
        assert_eq!(scopes.code, "");
        scopes.add_scope(ScopeList::SearchStructures);
        assert_eq!(scopes.code, "esi-search.search_structures.v1");
    }
    #[test]
    fn append_duplicate_scopes() {
        let mut scopes = EsiScopes::new();
        assert_eq!(scopes.code, "");
        scopes.add_scope(ScopeList::SearchStructures);
        assert_eq!(scopes.code, "esi-search.search_structures.v1");
        scopes.add_scope(ScopeList::ReadStructures);
        assert_eq!(
            scopes.code,
            "esi-search.search_structures.v1 esi-universe.read_structures.v1"
        );
        scopes.add_scope(ScopeList::SearchStructures);
        assert_eq!(
            scopes.code,
            "esi-search.search_structures.v1 esi-universe.read_structures.v1"
        );
    }
}
