use std::{env, process};
use todo::Todo;

fn main() {
    let todo = Todo::new().unwrap_or_else(|err| {
        eprintln!("Unable to start todo list: {err}");
        process::exit(4);
    });

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Because of listed above, we have to match on args[1]
        let first_arg = &args[1];
        match &first_arg[..] {
            "list" => todo.list(),
            "add" => todo.add(&args[2..]),
            "complete" => todo.complete(&args[2..]),
            "delete" => todo.delete(&args[2..]),
            "update" => {
                let min_size = 4;
                if &args.len() < &min_size {
                    eprintln!("Not enough arguments");
                    eprintln!(
                        "Ex. todo update <item to make the change to> <what to change the item to>"
                    );
                    process::exit(3);
                }
                todo.update(&args[2], &args[3])
            }
            "uncomplete" => todo.uncomplete(&args[2..]),
            "clear" => todo.clear(),
            _ => println!("Not a valid argument, {}", TODO_HELP),
        }
    } else {
        eprintln!("{}", TODO_HELP)
    }
}

const TODO_HELP: &str = "
I see you need some help. Here are how to use the todo app!
todo list                       / lists all items in the list
todo add <args>                 / adds item(s) to the list
todo complete <args>            / completes item(s). This appears as a strikethrough
todo delete <args>              / deletes item(s) in the list
todo update <old> <new>         / changes the item from <old> to <new> in the list
todo uncomplete <args>          / if an item was struck-through, it returns it to its normal state
todo clear                      / completely clears the todo file where items are stored
";
