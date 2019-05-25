#[macro_use]
extern crate log;

extern crate test_backend;

use test_backend::persistence::pg_params::PG_PARAMS_TEST;
use test_backend::persistence::{ UserDao, UserDaoPg };
use test_backend::dto::User;
use test_backend::utility::init_logger;

fn create_dao() -> UserDaoPg {
    match UserDaoPg::new(PG_PARAMS_TEST) {
        Ok(dao) => dao,
        Err(e) => {
            error!("{}", e);
            panic!();
        }
    }
}


#[test]
fn add_user() {
    init_logger();

    let dao = create_dao();

    let mut user = User::default();
    user.set_name("Hans");
    user.set_email("hans@sers.com");

    match dao.username_exists(user.get_name()) {
        Ok(username_exists) => assert_eq!(false, username_exists),
        Err(e) => {
            error!("{}", e);
            panic!();
        }
    }
    match dao.email_exists(user.get_email()) {
        Ok(email_exists) => assert_eq!(false, email_exists),
        Err(e) => {
            error!("{}", e);
            panic!();
        }
    }

    match dao.add_user(user) {
        Ok(u) => {
            assert_eq!("Hans", u.get_name());
            assert_eq!("hans@sers.com", u.get_email());
        },
        Err(e) => {
            error!("{}", e);
            panic!();
        }
    }
}