use rusqlite::{Connection, Result};

mod user_info;
mod database_handler;

fn main() -> Result<()>{
    let conn = Connection::open("practice.db")?;

    Ok(())
}
