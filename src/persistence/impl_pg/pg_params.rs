use crate::utility::get_setting;

lazy_static! {
    pub static ref PG_PARAMS: String = format!(
        "postgres://postgres@{}:{}/backend-test",
        get_setting("BACKEND_POSTGRES_IP"),
        get_setting("POSTGRES_PORT")
    );
}
