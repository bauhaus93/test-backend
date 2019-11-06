use postgres::{Connection, TlsMode};
use postgres::tls;
use std::{thread, time};

use crate::persistence::DaoError;

pub fn try_connect(pg_params: &str) -> Result<Connection, DaoError> {
    const MAX_TRIES: u64 = 5;
	const SLEEP_FACTOR: u64 = 1000;
    let negotiator = NativeTls::new()?;
	debug!("Connecting to db with '{}'", pg_params);
    for i in 0..MAX_TRIES + 1 {
        match Connection::connect(pg_params, TlsMode::Require(&negotiator)) {
            Ok(c) => return Ok(c),
            Err(e) => {
				if i == MAX_TRIES {
					error!("DB connection failed, reached MAX_TRIES = {}", MAX_TRIES);
					return Err(DaoError::from(e));
				} else {
					warn!("DB Connection failed, retrying in {} ms", SLEEP_FACTOR * (i + 1));
					thread::sleep(time::Duration::from_millis(SLEEP_FACTOR * (i + 1)));
				}
		}
        }
    }
	unreachable!()
}
