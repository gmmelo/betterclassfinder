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

fn update_user_password(db: &Connection, user_id: i64, new_password: &str) -> Result<()> {
    db.execute(
        "UPDATE users SET encrypted_password = ?1 WHERE id = ?2",
        params![new_password, user_id] 
    )?;

    Ok(());
}

fn delete_user(db: &Connection, user_id: i64) -> Result<> {
    db.execute(
        "DELETE FROM users WHERE id = ?1",
        params![user_id]
    )?;

    Ok(());
}