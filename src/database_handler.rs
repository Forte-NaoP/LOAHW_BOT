use serde::Serialize;
use tokio_rusqlite::Connection as Connection;
use rusqlite::{Result, params};
use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc, Weekday, Date, NaiveDateTime, Duration};
use chrono_tz::Asia::Seoul;
use chrono_tz::Tz;

use super::user_info::UserInfo;

pub async fn initialize(conn: &Connection) -> Result<()> {
    conn.call(|conn| {
        let mut create_table = conn.prepare("CREATE TABLE IF NOT EXISTS ?1 (
            ?2 text primary key,
            ?3 text")
            .unwrap();
        create_table.execute(params!["user", "name", "character"]).unwrap();
        create_table.execute(params!["timeline", "tense", "rfc_str"]).unwrap();
        conn.execute(
            "REPLACE INTO timeline (tense, rfc_str) VALUES (?1, ?2))", 
            params!["last_access", Utc::now().with_timezone(&Seoul).to_rfc2822()]
        ).unwrap();

        conn.execute(
            "INSERT OR IGNORE INTO timeline (tense, rfc_str) VALUES (?1, ?2))", 
            params!["last_wed", (get_last_wed() + Duration::days(7)).to_rfc2822()]
        ).unwrap();
    }).await;

    reset_weekly(conn).await.unwrap();

    Ok(())
}

async fn reset_weekly(conn: &Connection) -> Result<()> {

    let (last_access, last_wed) = 
        conn.call(|conn| {
            let mut stmt = conn.prepare("SELECT rfc_str FROM timeline WHERE tense = ?1").unwrap();
            let last_access = stmt.query_row(params!["last_access"], |r| {
                Ok(DateTime::parse_from_rfc2822(r.get::<usize, String>(0).unwrap().as_str()).unwrap().with_timezone(&Seoul))
            }).unwrap();
            let last_wed = stmt.query_row(params!["last_wed"], |r| {
                Ok(DateTime::parse_from_rfc2822(r.get::<usize, String>(0).unwrap().as_str()).unwrap().with_timezone(&Seoul))
            }).unwrap();

            (last_access, last_wed)
        }).await;

    if last_access >= last_wed {
        reset_manually_all(conn).await.unwrap();
        conn.call(move |conn| {
            conn.execute("UPDATE timeline SET rfc_str = ?1 WHERE tense = ?2", params![(last_wed + Duration::days(7)).to_rfc2822(), "last_wed"]).unwrap();
        }).await;
    }
    
    Ok(())
}

async fn reset_manually_all(conn: &Connection) -> Result<()> {
    let mut users = conn.call(move |conn| {
        let mut stmt = conn.prepare("SELECT character FROM user").unwrap();
        let rows = stmt
            .query_map(
                params![], 
                |row| {
                    Ok(row.get::<usize, String>(0)?)
                }
            ).unwrap();

        let mut users: Vec<UserInfo> = Vec::new();
        for user in rows {
            users.push(serde_json::from_str(user?.as_str()).unwrap());
        }

        Ok::<_, rusqlite::Error>(users)
    }).await?;

    for user in users.iter_mut() {
        for (_, chars) in user.user_character_mut().iter_mut() {
            chars.reset_done_hw();
        }
    }

    conn.call(move |conn| {
        let mut stmt = conn.prepare("UPDATE user SET character = (?1) WHERE name = (?2)").unwrap();
        for user in users.iter() {
            stmt.execute(params![user.to_json(), user.user_name()]).unwrap();
        }
    }).await;

    Ok(())
}

pub async fn reset_manually_one(conn: &Connection, user: &String) -> Result<()> {
    let mut user_info = conn.call(move |conn| {
        let mut stmt = conn.prepare("SELECT character FROM user").unwrap();
        let user_info = stmt
            .query_row(
                params![], 
                |row| {
                    Ok(row.get::<usize, String>(0)?)
                }
            ).unwrap();

        Ok::<UserInfo, rusqlite::Error>(serde_json::from_str(user_info.as_str()).unwrap())
    }).await?;

    for (_, chars) in user_info.user_character_mut().iter_mut() {
        chars.reset_done_hw();
    }

    conn.call(move |conn| {
        let mut stmt = conn.prepare("UPDATE user SET character = (?1) WHERE name = (?2)").unwrap();
        stmt.execute(params![user_info.to_json(), user_info.user_name()]).unwrap();
    }).await;

    Ok(())
}


fn get_last_wed() -> DateTime<Tz> {
    let now: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
    let now_num = now.weekday().num_days_from_sunday();
    let wed_num = Weekday::Wed.num_days_from_sunday();
    
    let mut diff = now_num.abs_diff(wed_num);
    if wed_num > now_num {
        diff += 3;
    }

    Utc.ymd(now.year(), now.month(), now.day()-diff).and_hms(6, 0, 0).with_timezone(&Seoul)
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

pub fn get_user_data(conn: &mut rusqlite::Connection, user_name: &str) -> Result<String> {
    conn.query_row(
        "SELECT character FROM user WHERE name = (?1)",
        params![user_name],
        |row| row.get(0)
    )
}

