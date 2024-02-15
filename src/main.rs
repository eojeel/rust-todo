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

    html! {
        <div>
        <div>
        <div class="h-100 w-full flex items-center justify-center bg-teal-lightest font-sans">
        <div class="bg-white rounded shadow p-6 m-4 w-full lg:w-3/4 lg:max-w-lg">
            <div class="mb-4">
                <h1 class="text-grey-darkest">{ "Todo List" }</h1>
                <div class="flex mt-4">
                    <input type="text" class="shadow appearance-none border rounded w-full py-2 px-3 mr-4 text-grey-darker"/>

                    <button class="flex-no-shrink p-2 border-2 rounded text-teal border-teal hover:text-white hover:bg-teal">{ "Add" }</button>
                </div>
            </div>
            <div>
                <div class="flex mb-4 items-center">
                    <p class="w-full text-grey-darkest">{ "Add another component to Tailwind Components" }</p>
                    <button class="flex-no-shrink p-2 ml-4 mr-2 border-2 rounded hover:text-white text-green border-green hover:bg-green">{ "Done" }</button>
                    <button class="flex-no-shrink p-2 ml-2 border-2 rounded text-red border-red hover:text-white hover:bg-red">{ "Remove" }</button>
                </div>
            </div>
        </div>
        </div>
    </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
