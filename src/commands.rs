use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

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
}
