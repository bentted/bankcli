

---

# BankCLI

**BankCLI** is a command-line banking application written in **Rust**. This application provides features like user registration, login, and account verification. It is designed to demonstrate secure and efficient banking operations in a simple CLI-based interface.

---

## Features

### 🔒 Secure User Management
- User registration with validation.
- User login with password authentication.
- Account verification to ensure data integrity.

### 💾 Persistent Data Storage
- Stores user data securely for future reference and transactions.

### ⚡ Fast and Lightweight
- Written in Rust, ensuring high performance and low memory usage.

### 🛠 Modular Design
- Divided into client and server modules for better maintainability and scalability.

---

## Repository Structure

| File/Folder      | Description                                                                 |
|------------------|-----------------------------------------------------------------------------|
| `Cargo.toml`     | Contains metadata and dependencies for the Rust project.                   |
| `client.rs`      | Handles client-side operations, such as user interactions and input.       |
| `server.rs`      | Manages server-side logic, such as data validation and database handling.   |
| `main.rs`        | The entry point of the application, orchestrating client and server logic. |

---

## How It Works

### 1. User Registration
- Users can register by providing a username and password.
- Input validation ensures only valid data is accepted.

### 2. Login and Authentication
- Existing users can log in to access their account.
- Passwords are securely handled to protect user privacy.

### 3. Account Verification
- The system verifies user accounts to prevent unauthorized access.
- Ensures only registered users can interact with the application.

### 4. Modular Architecture
- **Client Module**: Handles user input, CLI interactions, and communication with the server.
- **Server Module**: Processes requests, validates data, and manages storage.

---

## Getting Started

### Prerequisites
- Rust programming language installed. If not, install it from [Rust's official website](https://www.rust-lang.org/).

### Setup
1. Clone the repository:
   ```bash
   git clone https://github.com/bentted/bankcli.git
   cd bankcli
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the application:
   ```bash
   cargo run
   ```

---

## Usage

Once the application is running, you can:
1. Register a new user:
   ```bash
   Register a new account by following the CLI prompts.
   ```
2. Login to an existing account:
   ```bash
   Provide your username and password to access your account.
   ```
3. Perform account operations:
   - View account details.
   - Perform transactions (if implemented).
   - Verify the account.

---

## Dependencies

The project uses the following dependencies:
- Rust Standard Library (for core functionalities).
- [Add any additional dependencies if used.]

---

## Contribution

Contributions are welcome! If you’d like to contribute:
1. Fork the repository.
2. Create a new branch for your feature.
3. Submit a pull request explaining your changes.

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

## Acknowledgments

- Built with ❤️ using the Rust programming language.
- Inspired by the need for secure and efficient CLI-based applications.

---
