#![allow(dead_code)]

use std::io::{stdin, stdout};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

/// according to the command a printout in a certain color will happen
/// see that the program is working, output in different colors
/// based on what the program is working
/// different scenarios might have different colors in them
#[derive(Debug,PartialEq)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        // decide the print color
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        // print the agent statement
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}", agent_pos);

        // reset color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        print!("{}", agent_statement);

        // reset color once again
        stdout.execute(ResetColor).unwrap();
    }
}

// get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question); // prints out question in color blue

    // reset color
    stdout.execute(ResetColor).unwrap();

    // read user input
    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    // trim whitespace and return
    return user_response.trim().to_string();
}

#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_print_agents_msg -- --nocapture
    #[test]
    fn tests_print_agents_msg() {

        PrintCommand::AICall
            .print_agent_message(
                "Managing agent", 
                "Testing, testing, processing something"
            );
    }
}