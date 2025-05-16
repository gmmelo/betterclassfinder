use std::error::Error;
// use reqwest::blocking::Client;
use rusqlite::{params, Connection, Result};

fn main() -> Result<(), Box<dyn Error>> {
    let _url = "https://eadvs-cscc-catalog-api.apps.asu.edu/catalog-microservices/api/v1/search/classes?&refine=Y&campusOrOnlineSelection=A&catalogNbr=230&honors=F&promod=F&searchType=all&subject=CSE&term=2257";

    // refine = Y, 
    // campusOrOnlineSelection = A
    // catalogNbr = [CLASS NUMBER]
    // honors = F, T
    // promod = F, T
    // searchType = all
    // subject = [CLASS SUBJECT]
    // term = [TERM NUMBER]


/*     let json: serde_json::Value = Client::new()
        .get(url)
        .header("authorization", "Bearer null")
        .send()?
        .json()?;

    println!("{:#?}", json); */

    let db = Connection::open("tracked.sqlite3")?;


    // List of users
    db.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            encrypted_password TEXT
        )",
        []
    )?;

    // List of classes being watched by at least one user
    db.execute(
        "CREATE TABLE IF NOT EXISTS classes (
            id INTEGER PRIMARY KEY
        )",
        []
    )?;

    // List of each individual class being tracked by each user
    db.execute(
        "CREATE TABLE IF NOT EXISTS trackers (
            class_id TEXT NOT NULL,
            user_id INTEGER NOT NULL,
            PRIMARY KEY (class_id, user_id),
            FOREIGN KEY (class_id) REFERENCES classes(id),
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
        []
    )?;

    Ok(())
}
