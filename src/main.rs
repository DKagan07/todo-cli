use std::{env, process};
use todo_cli::Todo;

fn main() {
    let todo = Todo::new().unwrap_or_else(|err| {
        eprintln!("Unable to start todo list: {err}");
        process::exit(4);
    });

    let args: Vec<String> = env::args().collect();

    // args[0] is the name of the program
    // args[1] is the first argument passed in (todo), etc...
    if args.len() > 1 {
        // Because of listed above, we have to match on args[1]
        let first_arg = &args[2];
        match &first_arg[..] {
            "list" => todo.list(),
            "add" => todo.add(&args[3..]),
            "complete" => todo.complete(&args[3..]),
            "delete" => todo.delete(&args[3..]),
            "update" => {
                let min_size = 5;
                if &args.len() < &min_size {
                    eprintln!("Not enough arguments");
                    eprintln!(
                        "Ex. todo update <item to make the change to> <what to change the item to>"
                    );
                    process::exit(3);
                }
                todo.update(&args[3], &args[4])
            }
            "uncomplete" => todo.uncomplete(&args[3..]),
            "clear" => todo.clear(),
            _ => println!("blablabla"),
        }
    } else {
        println!("Did not provide enough arguments. Try: ...");
        // fail here
    }
}
