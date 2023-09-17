pub mod login;

#[cfg(test)]
mod tests {
    use crate::auth::login::launch_login_page;

    #[test]
    fn login_test() {
        assert_ne!(launch_login_page("test", "test", "test"), true);
    }
}
