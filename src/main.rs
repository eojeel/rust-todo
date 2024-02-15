use yew::prelude::*;
use std::process;
use std::env;

mod commands;

use commands::*;

// fn main() -> () {
//     let args: Vec<String> = env::args().collect();

//     let command = args.get(1).unwrap_or_else(|| {
//         println!("Please provide a command");
//         display_help();
//         process::exit(0);
//     });

//     let exit_code = match command.as_str() {
//         "add" => AddCommand::new(args).handle(),
//         "complete" => CompleteCommand::new(args).handle(),
//         "edit" => EditCommand::new(args).handle(),
//         "list" => ListCommand::new().handle(),
//         _ => {
//             println!("Invalid command");
//             1
//         }
//     };

//     process::exit(exit_code);

// }


#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn display_help() -> () {
    println!("Usage: todo <command> <args>");

    println!();
    println!("Commands:");
    println!(" Add <description> - add a new todo");
    println!(" Done <id> - mark a todo as done");
    println!(" List - list all todos");
    println!();
}
