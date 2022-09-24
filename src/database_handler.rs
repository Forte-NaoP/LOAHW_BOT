use rusqlite::{Connection, Result, params, Error};

use super::user_info::UserInfo;

pub fn initialize(conn: &Connection) -> Result<()> {
    conn.execute("CREATE TABLE IF NOT EXISTS user (
        name text primary key,
        character text
    )", params![])?;
    Ok(())
}

pub fn user_register(conn: &Connection, user_data: &UserInfo) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO user (name, character) 
        VALUES (?1, ?2)",
        params![user_data.user_name(), user_data.to_json()],
    )?;
    Ok(())
}

pub fn user_delete(conn: &Connection, user_name: &String) -> Result<(), rusqlite::Error>{
    conn.execute("DELETE FROM user WHERE name = (?1)", params![user_name])?;
    Ok(())
}

pub fn user_update(conn: &Connection, new_data: &UserInfo) -> Result<(), rusqlite::Error> {
    
    let current_data:Result<String> = conn.query_row(
        "SELECT character FROM user WHERE name = (?1)",
        params![new_data.user_name()],
        |row| row.get(0));
    
    let mut current_data: UserInfo = serde_json::from_str(current_data.unwrap().as_str()).unwrap();
    let mut current_character = current_data.user_character_mut();

    for (name, lv) in new_data.user_character() {
        current_character.insert(name.to_string(), *lv);
    }

    conn.execute("UPDATE user SET character = (?1) WHERE name = (?2)", params![current_data.to_json(), new_data.user_name()])?;

    Ok(())
}