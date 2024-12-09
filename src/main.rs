use std::collections::HashMap;
use std::io;

const INVALID: &str = "Invalid input";
const OPTIONS: [&str; 7] = ["1", "2", "3", "4", "5", "6", "7"];

enum Status {
    UsernameNotFound,
    IncorrectPassword,
    LoginSuccess,
    Error
}

enum MenuOption {
    CreateUser,
    Login,
    ChangePassword,
    DeleteUser,
    Logout,
    ListUsers,
    Exit,
}

fn get_menu_option(choice: &str) -> Option<MenuOption> {
    match choice.trim() {
        "1" => Some(MenuOption::Login),
        "2" => Some(MenuOption::CreateUser),
        "3" => Some(MenuOption::ChangePassword),
        "4" => Some(MenuOption::DeleteUser),
        "5" => Some(MenuOption::Logout),
        "6" => Some(MenuOption::ListUsers),
        "7" => Some(MenuOption::Exit),
        _ => None,
    }
}

fn create_account(user_map: &mut HashMap<String, String>) -> (String, bool) {

    let mut username = String::new();
    let mut password = String::new();
    let mut password2 = String::new();

    'account: loop {
        username.clear();
        println!("Enter username");
        io::stdin().read_line(&mut username).expect("Failed to read line");
        print!("{esc}c", esc = 27 as char);
        username = username.trim().to_string();

        if user_map.contains_key(&username) {
            println!("Username already exists");
            println!("Try a different username");
            if user_decision() {
                continue 'account;
            } else {
                return (username, false);
            }
        }

        'password: loop {
            password.clear();
            password2.clear();
            println!("Enter password");
            io::stdin().read_line(&mut password).expect("Failed to read line");
            print!("{esc}c", esc = 27 as char);
            println!("Re-enter password");
            io::stdin().read_line(&mut password2).expect("Failed to read line");
            print!("{esc}c", esc = 27 as char);

            if password.trim() != password2.trim() {
                println!("Passwords do not match");
                continue 'password;
            } else {
                println!("User created {username}:{password}");
                user_map.insert(username.clone(), password.trim().to_string());
                return (username, true);
            }
        }
    }
}

fn validate_account(user_map: &mut HashMap<String, String>, username: &String, password: &String) -> Status {
    if password == INVALID || username == INVALID {
        // println!("Something went wrong!");
        Status::Error
    } else if !user_map.contains_key(username) {
        // println!("Username not found, consider creating an account!");
        return Status::UsernameNotFound;
    } else if user_map.get(username).expect("Unexpected! User was just deleted!") == password {
        // println!("Successfully logged in!");
        Status::LoginSuccess
    } else {
        // println!("Password is incorrect!");
        Status::IncorrectPassword
    }
}

fn ask_for_string(msg: &str) -> String {
    let mut variable: String = String::new();

    println!("{}", msg);
    variable.clear(); // Clear buffer as readline appends to a string!
    match io::stdin().read_line(&mut variable) {
        Ok(_) => {
            variable = variable.trim().to_string();
        }
        Err(_) => variable = INVALID.to_string()
    }
    print!("{esc}c", esc = 27 as char);

    variable
}

fn user_decision() -> bool {

    // Continue loop
    loop {
        // Clear buffer as readline appends to a string!
        let answer = ask_for_string("Continue? [Y/n]");
        match answer.as_str() {
            INVALID => {
                println!("Error reading input");
                print!("{esc}c", esc = 27 as char);
                return false; // Exit the outer loop on error
            }
            _ => {
                let input = answer.trim().to_lowercase();
                match input.as_str() {
                    "y" | "yes" => return true,
                    "n" | "no" => return false,
                    _ => {
                        println!("Invalid input. Please enter 'Y' or 'n'!, You gave {}", input.as_str());
                        continue
                    }
                }
            }
        }
    }
}

fn display_menu() {
    println!("Please choose an option:");
    println!("1. LogIn");
    println!("2. SignUp");
    println!("3. Change Password");
    println!("4. Delete User");
    println!("5. LogOut");
    println!("6. List Users");
    println!("7. Exit");
}

fn list_users(user_map: &HashMap<String, String>) {
    println!("{} Users in Database:", user_map.len());
    for (username, password) in user_map.iter() {
        println!("{}:{}", username, password);
    }
}

fn main() {
    print!("{esc}c", esc = 27 as char);

    let mut option;
    let mut current_user: String = String::from(INVALID);
    let mut user_map: HashMap<String, String> = HashMap::new();

    'program: loop {
        'option: loop {
            display_menu();
            option = ask_for_string("");
            if OPTIONS.contains(&option.as_str()) {
                break 'option;
            }
        }

        match get_menu_option(option.as_str()) {
            Some(MenuOption::Login) => {
                'credentials: loop {
                    let username = ask_for_string("Enter username:");
                    if current_user == username {
                        print!("{esc}c", esc = 27 as char);
                        println!("Already logged in to this account!");
                        continue 'program;
                    }
                    let password = ask_for_string("Enter password:");

                    match validate_account(&mut user_map, &username, &password) {
                        Status::Error => {
                            print!("{esc}c", esc = 27 as char);
                            print!("Exited with unexpected option!");
                            break 'program;
                        }
                        Status::LoginSuccess => {
                            print!("{esc}c", esc = 27 as char);
                            println!("Successfully logged in!");
                            current_user = username;
                            continue 'program;
                        }
                        _ => {
                            print!("{esc}c", esc = 27 as char);
                            println!("Incorrect username or password!");
                            println!("Maybe Retry");
                            if user_decision() {
                                continue 'credentials;
                            } else {
                                continue 'program;
                            }
                        }
                    }
                }
            }
            Some(MenuOption::CreateUser) => {
                let created: bool;
                let temp;
                (temp, created) = create_account(&mut user_map);
                if created {
                    current_user = temp;
                }
            }
            Some(MenuOption::ChangePassword) => {
                let mut password: String = String::from(INVALID);
                let mut password2: String = String::from(INVALID);
                match current_user.as_str() {
                    INVALID => {
                        println!("Login first!");
                        continue 'program;
                    }
                    _ => {
                        print!("{esc}c", esc = 27 as char);
                        'password: loop {
                            password.clear();
                            password2.clear();
                            println!("Enter password");
                            io::stdin().read_line(&mut password).expect("Failed to read line");
                            print!("{esc}c", esc = 27 as char);
                            println!("Re-enter password");
                            io::stdin().read_line(&mut password2).expect("Failed to read line");
                            print!("{esc}c", esc = 27 as char);

                            if password.trim() != password2.trim() {
                                println!("Passwords do not match");
                                continue 'password;
                            } else {
                                user_map.insert(current_user.clone(), password.trim().to_string());
                                println!("Updated Password");
                                continue 'program;
                            }
                        }
                    }
                }
            }
            Some(MenuOption::DeleteUser) => {
                if !user_decision() {
                    continue 'program;
                }
                match current_user.as_str() {
                    INVALID => {
                        print!("{esc}c", esc = 27 as char);
                        println!("Login first!");
                        continue 'program;
                    }
                    _ => {
                        print!("{esc}c", esc = 27 as char);
                        user_map.remove(&current_user);
                        current_user = String::from(INVALID);
                        println!("User removed!");
                        continue 'program;
                    }
                }
            }
            Some(MenuOption::Logout) => {
                if current_user.as_str() != INVALID {println!("Logged Out!")} else {println!("Not Logged in!")};
                current_user = String::from(INVALID)
            }
            Some(MenuOption::ListUsers) => {
                'listing: loop {
                    list_users(&user_map);
                    if user_decision() {continue 'program} else {continue 'listing}
                }
            }
            Some(MenuOption::Exit) => {
                print!("{esc}c", esc = 27 as char);
                print!("Exited normally!");
                break 'program;
            }
            _ => {
                print!("{esc}c", esc = 27 as char);
                print!("Exited with unexpected option!");
                break 'program;
            }
        }
    }
}
