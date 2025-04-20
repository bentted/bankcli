use rusqlite::{params, Connection, Result};
use std::sync::Mutex;

#[derive(Debug)]
enum Account {
    Balance(()),
}

lazy_static::lazy_static! {
    static ref ACCOUNT_BALANCE: Mutex<Account> = Mutex::new(Account::Balance(())); // Initial balance
}

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

// Verify if an account is unique before creating it
fn is_unique_account(conn: &Connection, name: &str) -> bool {
    let mut stmt = conn
        .prepare("SELECT COUNT(*) FROM accounts WHERE name = ?1")
        .expect("Failed to prepare statement");
    let count: i64 = stmt
        .query_row(params![name], |row| row.get(0))
        .unwrap_or(0);
    count == 0
}

// Verify account name and PIN
fn verify_account(conn: &Connection, name: &str, pin: i64) -> bool {
    let mut stmt = conn
        .prepare("SELECT COUNT(*) FROM accounts WHERE name = ?1 AND pin = ?2")
        .expect("Failed to prepare statement");
    let count: i64 = stmt
        .query_row(params![name, pin], |row| row.get(0))
        .unwrap_or(0);
    count > 0
}

// Create a new account
fn create_account(conn: &Connection, name: &str, pin: i64, initial_balance: i64) {
    if is_unique_account(conn, name) {
        conn.execute(
            "INSERT INTO accounts (name, pin, balance) VALUES (?1, ?2, ?3)",
            params![name, pin, initial_balance],
        )
        .expect("Failed to create account");
        println!("Account created successfully!");
    } else {
        println!("Account with this name already exists!");
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

// Deposit function
fn deposit(conn: &Connection, name: &str) {
    if let Some(current_balance) = get_balance_from_db(conn, name) {
        let mut input = String::new();
        println!("Enter the amount to deposit:");
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i64 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number format. Deposit canceled.");
                return;
            }
        };

        let new_balance = current_balance + num;
        update_balance_in_db(conn, name, new_balance);

        println!("Deposit successful. New balance: {}", new_balance);
    } else {
        println!("Account not found. Deposit canceled.");
    }
}

// Withdrawal function
fn withdrawal(conn: &Connection, name: &str) {
    if let Some(current_balance) = get_balance_from_db(conn, name) {
        let mut input = String::new();
        println!("Enter the amount to withdraw:");
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i64 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number format. Withdrawal canceled.");
                return;
            }
        };

        if num <= current_balance {
            let new_balance = current_balance - num;
            update_balance_in_db(conn, name, new_balance);
            println!("Withdrawal successful. Remaining balance: {}", new_balance);
        } else {
            println!("Insufficient funds. Current balance: {}", current_balance);
        }
    } else {
        println!("Account not found. Withdrawal canceled.");
    }
}

fn main() {
    let conn = match initialize_database() {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Failed to initialize database: {}", err);
            return;
        }
    };

    loop {
        println!("Welcome to the banking system!");
        println!("Are you a current user or would you like to register?");
        println!("1. Current User");
        println!("2. Register New Account");
        println!("3. Exit");

        let mut initial_choice = String::new();
        std::io::stdin()
            .read_line(&mut initial_choice)
            .expect("Failed to read input");
        let initial_choice = initial_choice.trim();

        match initial_choice {
            "1" => {
                println!("Enter your account name:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).expect("Failed to read input");
                let name = name.trim();

                println!("Enter your PIN:");
                let mut pin = String::new();
                std::io::stdin().read_line(&mut pin).expect("Failed to read input");
                let pin: i64 = match pin.trim().parse() {
                    Ok(p) => p,
                    Err(_) => {
                        println!("Invalid PIN format. Verification failed.");
                        continue;
                    }
                };

                if verify_account(&conn, name, pin) {
                    println!("Account verified successfully!");
                    // Proceed to the main banking menu
                    loop {
                        println!("Welcome, {}!", name);
                        println!("Please choose an option:");
                        println!("1. Deposit");
                        println!("2. Withdrawal");
                        println!("3. Exit");

                        let mut choice = String::new();
                        std::io::stdin()
                            .read_line(&mut choice)
                            .expect("Failed to read input");
                        let choice = choice.trim();

                        match choice {
                            "1" => deposit(&conn, name),
                            "2" => withdrawal(&conn, name),
                            "3" => {
                                println!("Exiting the system. Goodbye!");
                                break;
                            }
                            _ => println!("Invalid option. Please try again."),
                        }
                    }
                } else {
                    println!("Invalid account name or PIN. Returning to the main menu.");
                }
            }
            "2" => {
                println!("Enter account name:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).expect("Failed to read input");
                let name = name.trim();

                println!("Enter a 4-digit PIN:");
                let mut pin = String::new();
                std::io::stdin().read_line(&mut pin).expect("Failed to read input");
                let pin: i64 = match pin.trim().parse() {
                    Ok(p) => p,
                    Err(_) => {
                        println!("Invalid PIN format. Account creation canceled.");
                        continue;
                    }
                };

                create_account(&conn, name, pin, 0);
            }
            "3" => {
                println!("Exiting the system. Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}