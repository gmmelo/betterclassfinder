use rusqlite::{params, Connection, Result};

fn create_user(db: &Connection, usr: &str, pwd: &str) -> Result<i64> {
    db.execute(
        "INSERT INTO users (name, encrypted_password) VALUES (?1, ?2)",
        params![usr, pwd]
    )?;

    Ok();
}

fn get_user(db: &Connection, user_id: i64) -> Result<Option<User>> {
    let stmt = db.prepare(
        "SELECT id, name, encrypted_password FROM users WHERE id = ?1"
    )?;

    let rows = stmt.query(params![user_id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(User {
            id: row.get(0)?,
            name: row.get(1)?,
            encrypted_password: row.get(2)?
        }));
    } else {
        Ok(None);
    }
}