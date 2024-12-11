# rschat2

![rschat2_logo](./rs2_logo)


rschat2 is a terminal-based chat application written in Rust, designed for secure and private communication. It features end-to-end encryption, ensuring that only intended recipients can read the messages. Users can engage in private messages and create private channels, all within a text-based user interface (TUI).

## Features

1. **Text-Based User Interface (TUI)**
   - Interact with the chat application directly from the terminal, providing a lightweight and efficient user experience.

2. **End-to-End Encryption**
   - All messages are encrypted on the sender's side and decrypted only by the recipient, ensuring that even the server cannot read the data.

3. **Private Messages**
   - Send direct messages to individual users securely.

4. **Private Channels**
   - Create and join private channels for group discussions with enhanced privacy.

5. **User Authentication**
   - Authenticate with a username and password to access the chat application.

6. **Encrypted Server Configuration**
   - Server IPs are saved to a file and encrypted with the user's password, ensuring that only the program can access them.

7. **Debug Mode**
   - Enter `/debug` into the message input to display logs of recent activities.
   - The debug log is scrollable and includes a text bar at the bottom for commands like `/exit`.

## Installation and Usage

### Prerequisites

- Ensure that you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Steps to Run:

1. **Clone this repository:**
   ```bash
   git clone https://github.com/your-repo/rschat2.git
   ```

2. **Navigate to the project directory:**
   ```bash
   cd rschat2
   ```

3. **Build the application:**
   ```bash
   cargo build --release
   ```

4. **Run the application:**
   ```bash
   cargo run --release
   ```

## Planned Features

1. **Message Persistence**
   - Implement message history, allowing users to review past messages when they reconnect to the chat.