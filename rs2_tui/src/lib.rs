use pancurses::{initscr, endwin, Window, Input, COLOR_PAIR, resize_term, COLOR_RED, init_pair, start_color};
use rs2_auth::{db_exists, store_user, validate_user, init_db};
use log::{info, warn, error};

pub fn start_tui() {
    // Initialize pancurses
    info!("Starting TUI");
    let window = initscr();
    window.keypad(true);

    let fixed_rows = 24; // Fixed height
    let fixed_cols = 80; // Fixed width
    resize_term(fixed_rows, fixed_cols);

    
    start_color();
    init_pair(1, COLOR_RED, 0); // Define color pair for red text



    // Check if the database file exists
    if !db_exists() {
        warn!("Missing database - creating new user");
        handle_create_user(&window); // Load Create User screen if DB doesn't exist
    }
    
    

    loop {
        window.clear();
        window.refresh();

        // Load Login screen
        if handle_login(&window) {
            info!("User logged in");
            handle_connect(&window);
            pancurses::napms(15000); 
            break; // Exit loop if login is successful
        } else {
            warn!("Invalid login!");
            // Display incorrect login message
            let (max_y, _) = window.get_max_yx();
            window.attron(COLOR_PAIR(1)); // Enable red text
            window.mvaddstr(max_y - 2, 2, "Incorrect Username or Password.");
            window.attroff(COLOR_PAIR(1)); // Disable red text
            window.refresh();
            pancurses::napms(1500); // Pause for 1.5 seconds
        }
    }

    // Cleanup
    endwin();
}


fn handle_login(window: &Window) -> bool {
    let (username, password) = lock_screen(window);
    // Read and validate against the database
    let conn = init_db().unwrap();
    validate_user(&conn, &username, &password).unwrap()
}

fn handle_create_user(window: &Window) {
    let conn = init_db().unwrap();
    let mut username = String::new();
    let mut password1 = String::new();
    let mut password2 = String::new();

    loop {
        window.clear();
        window.mvaddstr(0, 0, "Create User");
        window.mvaddstr(2, 0, "Enter username:");
        window.refresh();
        username = get_input(window);

        window.clear();
        window.mvaddstr(0, 0, "Create User");
        window.mvaddstr(2, 0, "Enter password:");
        window.refresh();
        password1 = get_input_masked(window);

        window.clear();
        window.mvaddstr(0, 0, "Create User");
        window.mvaddstr(2, 0, "Re-enter password:");
        window.refresh();
        password2 = get_input_masked(window);

        if password1 == password2 {
            break;
        } else {
            let (max_y, _) = window.get_max_yx();
            window.attron(COLOR_PAIR(1)); // Enable red text
            window.mvaddstr(max_y - 2, 2, "Passwords do not match. Try again.");
            window.attroff(COLOR_PAIR(1)); // Disable red text
            window.refresh();
            pancurses::napms(1500); // Pause for 1.5 seconds
        }
    }

    // Hash and store the user
    store_user(&conn, &username, &password1).unwrap();

    window.clear();
    window.mvaddstr(0, 0, "User created successfully!");
    window.refresh();
    pancurses::napms(1500); // Pause for 1.5 seconds
}

fn lock_screen(window: &Window) -> (String, String) {
    let mut username = String::new();
    let mut password = String::new();

    window.clear();
    window.mvaddstr(0, 0, "Login");
    window.mvaddstr(2, 0, "Username:");
    window.mvaddstr(4, 0, "Password:");
    window.refresh();

    // Input username
    window.mv(2, 10);
    username = get_input(window);

    // Input password (masked)
    window.mv(4, 10);
    password = get_input_masked(window);

    (username, password)
}

/// Displays a screen to connect to a server by IP and port
pub fn handle_connect(window: &Window) {
    let mut ip = String::new();
    let mut port = String::new();

    // Clear and refresh the window
    window.clear();
    window.mvaddstr(0, 0, "Connect to Server");
    window.mvaddstr(2, 0, "Enter IP address:");
    window.refresh();

    // Get IP address input
    window.mv(2, 20);
    ip = get_input(window);
    

    // Ask for port
    window.mvaddstr(4, 0, "Enter port:");
    window.mv(4, 20);
    port = get_input(window);

    info!("connection {ip}:{port}");

    // Display connection details (for debugging purposes)
    window.mvaddstr(6, 0, &format!("Connecting to {}:{}...", ip, port));
    window.refresh();
}



/// Helper function to get user input (plain text)
fn get_input(window: &Window) -> String {
    let mut input = String::new();
    loop {
        match window.getch() {
            Some(Input::Character('\n')) => break, // Enter key to finish input
            Some(Input::Character(c)) => {
                if input.len() < 128 {
                    input.push(c);
                }
            }
            Some(Input::KeyBackspace) => {
                if !input.is_empty() {
                    input.pop();
                    let (y, x) = window.get_cur_yx();
                    if x > 0 {
                        window.mv(y, x - 1);
                        window.delch();
                    }
                }
            }
            _ => {}
        }
        window.refresh();
    }
    input
}

/// Helper function to get masked user input (e.g., passwords)
fn get_input_masked(window: &Window) -> String {
    let mut input = String::new();
    loop {
        match window.getch() {
            Some(Input::Character('\n')) => break, // Enter key to finish input
            Some(Input::Character(c)) => {
                if input.len() < 128 {
                    input.push(c);
                }
            }
            Some(Input::KeyBackspace) => {
                if !input.is_empty() {
                    input.pop();
                    let (y, x) = window.get_cur_yx();
                    if x > 0 {
                        window.mv(y, x - 1);
                        window.delch();
                    }
                }
            }
            _ => {}
        }
        window.refresh();
    }
    input
}

