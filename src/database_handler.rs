use rusqlite::{Connection, Result};

use super::user_info::user_info;

pub fn user_register(conn: &Connection, user_data: user_info) -> Result<()> {
    Ok(())
}

pub fn user_delete(conn: &Connection, user_name: String) {

}

pub fn user_update(conn: &Connection, user_data: user_info) {

}