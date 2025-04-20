use std::io::{self, Write};
use std::net::TcpStream;

fn main() {
    println!("Welcome to the Banking System Client!");
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Connected to the server!");

            loop {
                println!("Please choose an option:");
                println!("1. Deposit");
                println!("2. Withdrawal");
                println!("3. Exit");

                let mut choice = String::new();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let choice = choice.trim();

                match choice {
                    "1" => {
                        handle_deposit(&mut stream);
                    }
                    "2" => {
                        handle_withdrawal(&mut stream);
                    }
                    "3" => {
                        println!("Exiting the client. Goodbye!");
                        break;
                    }
                    _ => println!("Invalid option. Please try again."),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to the server: {}", e);
        }
    }
}

// Handle deposit operation
fn handle_deposit(stream: &mut TcpStream) {
    let mut account_name = String::new();
    println!("Enter your account name:");
    io::stdin()
        .read_line(&mut account_name)
        .expect("Failed to read input");
    let account_name = account_name.trim();

    let mut amount = String::new();
    println!("Enter the amount to deposit:");
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read input");
    let amount = amount.trim();

    let request = format!("DEPOSIT {} {}\n", account_name, amount);
    if let Err(e) = stream.write_all(request.as_bytes()) {
        eprintln!("Failed to send request: {}", e);
        return;
    }

    let mut response = String::new();
    if let Err(e) = io::BufReader::new(stream).read_line(&mut response) {
        eprintln!("Failed to read response: {}", e);
        return;
    }

    println!("Server response: {}", response.trim());
}

// Handle withdrawal operation
fn handle_withdrawal(stream: &mut TcpStream) {
    let mut account_name = String::new();
    println!("Enter your account name:");
    io::stdin()
        .read_line(&mut account_name)
        .expect("Failed to read input");
    let account_name = account_name.trim();

    let mut amount = String::new();
    println!("Enter the amount to withdraw:");
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read input");
    let amount = amount.trim();

    let request = format!("WITHDRAW {} {}\n", account_name, amount);
    if let Err(e) = stream.write_all(request.as_bytes()) {
        eprintln!("Failed to send request: {}", e);
        return;
    }

    let mut response = String::new();
    if let Err(e) = io::BufReader::new(stream).read_line(&mut response) {
        eprintln!("Failed to read response: {}", e);
        return;
    }

    println!("Server response: {}", response.trim());
}