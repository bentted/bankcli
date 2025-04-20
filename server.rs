use rusqlite::{params, Connection, Result};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// Initialize the database and create the accounts table
fn initialize_database() -> Result<Connection> {
    let conn = Connection::open("banking_system.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            pin INTEGER NOT NULL,
            balance INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

// Handle client requests
fn handle_client(mut stream: TcpStream, conn: &Connection) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                break;
            }
            Ok(_) => {
                let request = String::from_utf8_lossy(&buffer);
                let response = process_request(request.trim(), conn);
                stream.write(response.as_bytes()).expect("Failed to write to stream");
            }
            Err(e) => {
                eprintln!("Failed to read from client: {}", e);
                break;
            }
        }
    }
}

// Process client requests
fn process_request(request: &str, conn: &Connection) -> String {
    let parts: Vec<&str> = request.split_whitespace().collect();
    if parts.is_empty() {
        return "Invalid request format.\n".to_string();
    }

    match parts[0] {
        "DEPOSIT" => {
            if parts.len() != 3 {
                return "Usage: DEPOSIT <account_name> <amount>\n".to_string();
            }
            let name = parts[1];
            let amount: i64 = match parts[2].parse() {
                Ok(a) => a,
                Err(_) => return "Invalid amount format.\n".to_string(),
            };
            if let Some(current_balance) = get_balance_from_db(conn, name) {
                let new_balance = current_balance + amount;
                update_balance_in_db(conn, name, new_balance);
                format!("Deposit successful. New balance: {}\n", new_balance)
            } else {
                "Account not found.\n".to_string()
            }
        }
        "WITHDRAW" => {
            if parts.len() != 3 {
                return "Usage: WITHDRAW <account_name> <amount>\n".to_string();
            }
            let name = parts[1];
            let amount: i64 = match parts[2].parse() {
                Ok(a) => a,
                Err(_) => return "Invalid amount format.\n".to_string(),
            };
            if let Some(current_balance) = get_balance_from_db(conn, name) {
                if amount <= current_balance {
                    let new_balance = current_balance - amount;
                    update_balance_in_db(conn, name, new_balance);
                    format!("Withdrawal successful. Remaining balance: {}\n", new_balance)
                } else {
                    "Insufficient funds.\n".to_string()
                }
            } else {
                "Account not found.\n".to_string()
            }
        }
        _ => "Unknown command.\n".to_string(),
    }
}

// Update balance in the database
fn update_balance_in_db(conn: &Connection, name: &str, new_balance: i64) {
    conn.execute(
        "UPDATE accounts SET balance = ?1 WHERE name = ?2",
        params![new_balance, name],
    )
    .expect("Failed to update balance");
}

// Get balance from the database
fn get_balance_from_db(conn: &Connection, name: &str) -> Option<i64> {
    let mut stmt = conn
        .prepare("SELECT balance FROM accounts WHERE name = ?1")
        .expect("Failed to prepare statement");
    stmt.query_row(params![name], |row| row.get(0)).ok()
}

// Start the server
pub fn start_server() {
    let conn = initialize_database().expect("Failed to initialize database");
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");

    println!("Server is running on 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected.");
                let conn = conn.clone();
                std::thread::spawn(move || handle_client(stream, &conn));
            }
            Err(e) => eprintln!("Failed to accept client: {}", e),
        }
    }
}