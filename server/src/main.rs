mod users_crud;
mod tracked_classes_crud;
use std::error::Error;
use std::io::{self, Write};
// use reqwest::blocking::Client;
use rusqlite::{Connection, Result};
use tracked_classes_crud::get_trackers_by_class;

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



    // Database Testing
    loop {
        println!();
        println!("(1) Add user");
        println!("(2) Add tracker");
        println!("(3) List users by class");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unreadable line");

        let case: &str = input.trim();
        println!();
        match case {
            "1" => {
                // Get Name
                println!("Name:");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name: &str = name.trim();

                // Get Password
                println!("Encrypted Password:");
                io::stdout().flush().unwrap();
                let mut password = String::new();
                io::stdin().read_line(&mut password).unwrap();
                let password: &str = password.trim();
                
                // Create new user in database
                match users_crud::create_user(&db, name, password) {
                    Ok(id) => println!("Added {} ({})", name, id),
                    Err(e) => eprintln!("Error adding {}", e)
                }
            }

            "2" => {
                // Get class id
                println!("Class ID:");
                io::stdout().flush().unwrap();
                let mut class_input = String::new();
                io::stdin().read_line(&mut class_input).unwrap();
                let class_input = class_input.trim();

                // Get user id
                println!("User ID:");
                io::stdout().flush().unwrap();
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input).unwrap();
                let user_input= user_input.trim();

                let class_id: i64 = class_input.parse().expect("Invalid Class ID");
                let user_id: i64 = user_input.parse().expect("Invalid User ID");
                match tracked_classes_crud::create_tracker(&db, user_id, class_id) {
                    Ok(_) => println!("{} is tracking {}", user_id, class_id),
                    Err(e) => eprintln!("Error: {}", e)
                }
            }

            "3" => {
                // Get class id
                println!("Class ID:");
                io::stdout().flush().unwrap();
                let mut class_input = String::new();
                io::stdin().read_line(&mut class_input).unwrap();
                let class_input = class_input.trim();

                let class_id: i64 = class_input.parse().expect("Invalid Class ID");
                match get_trackers_by_class(&db, class_id) {
                    Ok(trackers) => for user_id in trackers {
                        println!("{} is tracking {}", user_id, class_id);
                    },
                    Err(e) => eprintln!("Error: {}", e)
                }
            }

            _ => {
                println!("Invalid");
                continue;
            }
        }
    }
}
