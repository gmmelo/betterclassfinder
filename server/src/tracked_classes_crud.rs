use rusqlite::{params, Connection, Result};

// Adds new class to list of tracked classes
fn create_tracked_class(db: &Connection, class_id: i64) -> Result<()> {
    db.execute(
        "INSERT OR IGNORE INTO classes (id) VALUES (?1)",
        params![class_id]
    )?;

    Ok(())
}

fn delete_tracked_class(db: &Connection, class_id: i64) -> Result<()> {
    db.execute(
        "DELETE FROM classes WHERE id = ?1",
        params![class_id]
    )?;

    Ok(())
}

// Adds tracking link for user and class
pub fn create_tracker(db: &Connection, user_id: i64, class_id: i64) -> Result<()> {
    // Creates class in tracked list if not yet tracked
    create_tracked_class(db, class_id)?;

    db.execute(
        "INSERT OR IGNORE INTO trackers (class_id, user_id) VALUES (?1, ?2)",
        params![class_id, user_id]
    )?;

    Ok(())
}

pub fn get_trackers_by_class(db: &Connection, class_id: i64) -> Result<Vec<i64>> {
    let mut stmt = db.prepare("SELECT user_id FROM trackers WHERE class_id = ?1")?;
    let users = stmt.query_map(params![class_id], |row| row.get(0))?;
    users.collect()
}

pub fn delete_tracker(db: &Connection, user_id: i64, class_id: i64) -> Result<()> {
    db.execute(
        "DELETE FROM trackers (class_id, user_id) VALUES (?1, ?2)",
        params![class_id, user_id]
    )?;

    let mut stmt = db.prepare(
        "SELECT EXISTS(SELECT : FROM trackers WHERE class_id = ?1"
    )?;
    let still_tracking_class: bool = stmt.query_row(params![class_id], |row| row.get(0))?;

    if !still_tracking_class {
        delete_tracked_class(db, class_id);
    }

    Ok(())
}
