use postgres::{Connection, TlsMode};
use std::{thread, time};

use crate::persistence::DaoError;

pub fn try_connect(pg_params: &str, max_tries: u64) -> Result<Connection, DaoError> {
	const SLEEP_FACTOR: u64 = 1000;
	debug!("Connecting to db with '{}'...", pg_params);
    for i in 0..max_tries + 1 {
        match Connection::connect(pg_params, TlsMode::None) {
            Ok(c) => return Ok(c),
            Err(e) => {
				if i == max_tries {
					error!("DB connection failed, reached max_tries = {}", max_tries);
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
