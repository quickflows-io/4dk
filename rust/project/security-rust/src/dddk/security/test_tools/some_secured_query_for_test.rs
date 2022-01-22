#[cfg(test)]
pub mod some_secured_query_for_test {
    use std::any::Any;
    use dddk_core::dddk::query::query::Query;
    use dddk_macro::Query;
    use crate::dddk::security::query::secured_query::SecuredQuery;

    #[derive(Query)]
    pub struct AQuery {}

    impl AQuery {
        pub fn new() -> AQuery {
            AQuery {}
        }
    }

    pub fn get_a_query_secured(roles: Vec<String>) -> SecuredQuery {
        SecuredQuery::new(Box::new(AQuery::new()), roles)
    }

    #[derive(Query)]
    pub struct AnotherQuery {}

    impl AnotherQuery {
        pub fn new() -> AnotherQuery {
            AnotherQuery {}
        }
    }
}