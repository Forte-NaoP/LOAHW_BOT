use std::fmt::format;

use tokio_rusqlite::Connection as Connection;
use rusqlite::{Result, params, Error};

use super::user_info::UserInfo;

pub async fn initialize(conn: &Connection) -> Result<()> {
    conn.call(|conn| {
        conn.execute("CREATE TABLE IF NOT EXISTS user (
            name text primary key,
            character text
        )", params![]).unwrap();
    }).await;

    Ok(())
}

pub async fn user_register(conn: &Connection, user_data: UserInfo) -> Result<()> {
    conn.call(move |conn| {
        conn.execute(
            "INSERT INTO user (name, character) 
            VALUES (?1, ?2)",
            params![user_data.user_name(), user_data.to_json()],
        ).unwrap();
    }).await;
    Ok(())
}

pub async fn user_delete(conn: &Connection, user_name: String) -> Result<()>{
    conn.call(move|conn| {
        conn.execute("DELETE FROM user WHERE name = (?1)", params![user_name]).unwrap();
    }).await;
    Ok(())
}

pub async fn user_update(conn: &Connection, new_data: UserInfo) -> Result<()> {
    conn.call(move |conn| {
        let current_data = get_user_data(conn, new_data.user_name());
        let mut current_data: UserInfo = serde_json::from_str(current_data.unwrap().as_str()).unwrap();
        let current_character = current_data.user_character_mut();
    
        for (name, lv) in new_data.user_character() {
            current_character.insert(name.to_string(), lv.clone());
        }
    
        conn.execute("UPDATE user SET character = (?1) WHERE name = (?2)", params![current_data.to_json(), new_data.user_name()]).unwrap();
    }).await;
    Ok(())
}

pub async fn character_delete(conn: &Connection, user_name:String, character_name: String) -> Result<()> {
    conn.call(move |conn| {
        let current_data = get_user_data(conn, &user_name);
        
        let mut current_data: UserInfo = serde_json::from_str(current_data.unwrap().as_str()).unwrap();
        let current_character = current_data.user_character_mut();
    
        current_character.remove(&character_name);
    
        conn.execute("UPDATE user SET character = (?1) WHERE name = (?2)", params![current_data.to_json(), user_name]).unwrap();
    }).await;

    Ok(())
}

pub async fn user_query(conn: &Connection, user_name:String) -> Result<UserInfo> {

    let character = conn.call(move |conn| {
        let mut stmt = conn.prepare("SELECT character FROM user WHERE name = (?1)")?;
        let character = stmt
            .query_row(params![user_name], |row| {
                Ok(row.get::<usize, String>(0)?)
            })?;
        let character: UserInfo = serde_json::from_str(character.as_str()).unwrap();
        Ok::<_, rusqlite::Error>(character)
    }).await?;

    Ok(character)
}

pub async fn reset_by_date(conn: &Connection) -> Result<()> {


    Ok(())
}

pub fn get_user_data(conn: &mut rusqlite::Connection, user_name: &str) -> Result<String> {
    conn.query_row(
        "SELECT character FROM user WHERE name = (?1)",
        params![user_name],
        |row| row.get(0)
    )
}

