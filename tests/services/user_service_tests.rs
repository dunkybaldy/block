#[cfg(test)]
pub mod user_service_tests {
    use rstest::rstest;
    
    use block::service::user_service::{UserService};
    use block::structs::user::User;

    #[rstest]
    fn test_equal_points_chooses() {
        
    }

    #[rstest]
    fn test_add_new_user() {
        let mut us: UserService = UserService::new();

        us.new_user("123", "Donk", "Bonk");

        let users = us.get_users();
        assert_eq!(1, users.len());

        let user = us.get_user("123");
        if let Some(u) = user {
            assert_eq!("123", u.id);
            assert_eq!("Donk", u.firstname);
            assert_eq!("Bonk", u.lastname);
            assert_eq!(0, u.total_points);
        } else {
            panic!("user should be there")
        }
    }

    #[rstest]
    fn test_user_not_found() {
        let us = UserService::new();
        let user = us.get_user("123");

        assert_eq!(true, user.is_none());
    }
}