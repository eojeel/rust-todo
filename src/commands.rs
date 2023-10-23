use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead, Result};


// Command interface
pub trait Command {
    fn handle(&self) -> i32 {
        println!("Command not implemented");
        1
    }
}

// Add Command
pub struct AddCommand {
    args: Vec<String>
}

impl AddCommand {
    pub fn new(args: Vec<String>) -> Self {
        AddCommand {
            args
        }
    }
}

impl Command for AddCommand {
    fn handle(&self) -> i32 {
        let description = self.args.get(2);

        if let Some(desc) = description {
            let mut file = match OpenOptions::new().write(true).append(true).open("Storage.txt") {
                Ok(file) => file,
                Err(_) => return 0,
            };

            if writeln!(file, "{}", desc).is_err() {
                return 0;
            }

            println!("Todo Added!");

            1

        } else {
            println!("Please provide a description");
            0
        }
    }
}


// List Command
pub struct ListCommand {

}

impl ListCommand {
    pub fn new() -> Self {
        ListCommand {

        }
    }
}

impl Command for ListCommand {
    fn handle(&self) -> i32 {

        let contents = fs::read_to_string("Storage.txt")
        .expect("Something went wrong reading the file");

        println!("{contents}");
        0
    }
}

// Done Command
pub struct CompleteCommand {
    args: Vec<String>
}

impl CompleteCommand {
    pub fn new(args: Vec<String>) -> Self {
        CompleteCommand {
            args
        }
    }
}

impl Command for CompleteCommand {
    fn handle(&self) -> i32 {
        let line_option = self.args.get(2);
        let file_path = "Storage.txt";

        let file = File::open(file_path)
        .expect("Unable to open file");

        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

        if let Some(line_str) = line_option {
            if let Ok(line_usize) = line_str.parse::<usize>() {
                if line_usize >= lines.len() {
                    println!("No todos to complete");
                    return 0;
                }

            // Open file for writing and truncate
            let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)
            .unwrap();

            // Write back all lines except the one to be deleted
            for (i, line) in lines.iter().enumerate() {
                if i != line_usize {
                    if writeln!(file, "{}", line).is_err() {
                        println!("Failed to write to file");
                        return 0;
                        }
                    }
                }
                println!("Todo completed!");
                return 0;
            } else {
                println!("Invalid index format");
                return 0;
            }
        } else {
            println!("Index argument missing");
            return 0;
        }
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_command() {
        // prepare test
        let args = vec![
            "todo".to_string(),
            "add".to_string(),
            "my todo".to_string()
        ];


        let command = AddCommand::new(args);

        // execute test
        let exit_code = command.handle();

        // Assert
        assert_eq!(exit_code, 1);
    }

    #[test]
    fn list_command() {

        let command = ListCommand::new();

        // execute test
        let exit_code = command.handle();

        // Assert
        assert_eq!(exit_code, 0);
    }

    #[test]
    fn complete_command() {
        // prepare test
        let args = vec![
            "done".to_string(),
            "0".to_string()
        ];

        let command = CompleteCommand::new(args);

        let exit_code = command.handle();

        // Assert
        assert_eq!(exit_code, 0);
    }

}
