use std::fs::OpenOptions;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::{env, fs, process};
use vibrance::style;

pub struct Todo {
    pub todo: Vec<String>,
    pub todo_path: String,
}

impl Todo {
    // new Creates a new todo list
    pub fn new() -> Result<Todo, String> {
        // checking for path
        let todo_path: String = match env::var("TODO_PATH") {
            Ok(p) => p,
            Err(_) => {
                println!("No env var TODO_PATH, will default to HOME");
                let home = env::var("HOME").unwrap();
                format!("{}/.todo", home)
            }
        };

        // will create a file in todo_path
        let todo_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&todo_path)
            .expect("Unable to create todo file");

        // Create a buffer reader and a string to be filled with TODOs
        let mut buf_rdr = BufReader::new(todo_file);
        let mut contents = String::new();

        // Loads contents with the result of the buffer from the file
        buf_rdr.read_to_string(&mut contents).unwrap();

        let todo = contents.lines().map(str::to_string).collect();

        return Ok(Todo { todo, todo_path });
    }

    // list Lists all of the items in the todo list
    pub fn list(&self) {
        let stdout = io::stdout();

        let mut buf = BufWriter::new(stdout);
        let mut result = String::new();

        // Reading file
        let contents = fs::read_to_string(&self.todo_path).unwrap();
        for (num, item) in contents.lines().enumerate() {
            let num = num + 1;
            let task = &item[4..];
            let is_completed = &item[..4];

            let res = if is_completed.contains("*") {
                format!("{} {}", num, style::strikethrough(task))
            } else {
                format!("{} {}", num, task)
            };
            result.push_str(&format!("{res}\n"));
        }
        buf.write_all(result.as_bytes()).unwrap()
    }

    // add Adds an item to the todo list
    pub fn add(&self, args: &[String]) {
        // Verifying arg length,
        if args.len() < 1 {
            eprintln!("Todo add takes at least 1 argument");
            process::exit(1);
        }

        // Open the todo_file
        let todo_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.todo_path)
            .expect("Could not open todo file");

        let mut buf = BufWriter::new(todo_file);
        let content = fs::read_to_string(&self.todo_path).unwrap();
        for arg in args {
            // Checks to see if the item is already in the list, if it is, then don't add it
            if content.contains(arg) {
                eprintln!("Item already in your list! You haven't done it yet. Go do it.");
                break;
            }

            if arg.trim().is_empty() {
                continue;
            }

            // this is how the todo tasks are defined
            let line = format!("[ ] {}\n", arg);

            buf.write_all(line.as_bytes()).unwrap()
        }
    }

    // complete Completes the task, causing a strikethrough to go through the item to denote this
    pub fn complete(&self, args: &[String]) {
        let mut result = String::new();

        // Reading file
        let todo_file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(&self.todo_path)
            .expect("Could not open todo file");
        let mut buf = BufWriter::new(todo_file);
        let contents = fs::read_to_string(&self.todo_path).unwrap();

        // Making sure all tasks are in the todo list, if not, exit with error
        // Also validates the tasks are already in the todo-list
        let mut tasks = Vec::new();
        for arg in args {
            if contents.contains(arg) {
                tasks.push(arg);
            }
        }
        if tasks.len() != args.len() {
            eprint!("Some tasks aren't in your todo list, please fix");
            process::exit(1);
        }

        // Now, have to loop through the lines and * the ones that are in the tasks vec
        for (_, item) in contents.lines().enumerate() {
            let task = &item[4..];
            // let checkbox = &item[..3];

            let todo_task = if tasks.contains(&&task.to_string()) {
                let check = "[*]";
                format!("{} {}\n", check, task)
            } else {
                format!("{}\n", item.to_string())
            };
            result.push_str(&todo_task);
        }
        buf.write_all(result.as_bytes()).unwrap()
    }

    // delete Deletes an entry in the todo file
    pub fn delete(&self, args: &[String]) {
        let mut result = String::new();

        // Reading file
        // This is an unoptimal way to delete an item at scale because we're just essentially
        // re-writing the whole file without the element(s) we wanted to delete
        let contents = fs::read_to_string(&self.todo_path).unwrap();
        let todo_file = OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Could not open todo file");
        let mut buf = BufWriter::new(todo_file);

        for (_, item) in contents.lines().enumerate() {
            let task = &item[4..];
            if args.contains(&task.to_string()) {
                continue;
            } else {
                result.push_str(format!("{}\n", item).as_str())
            };
        }

        buf.write_all(result.as_bytes()).unwrap()
    }

    pub fn update(&self, what_to_change: &String, change_to: &String) {
        let mut result = String::new();

        // Reading file
        let contents = fs::read_to_string(&self.todo_path).unwrap();
        let todo_file = OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Could not open todo file");
        let mut buf = BufWriter::new(todo_file);

        if !contents.contains(what_to_change) {
            eprintln!(
                "Item that you want to change ({}) does not exist in your todo list",
                what_to_change
            );
            process::exit(1);
        };
        for (_, item) in contents.lines().enumerate() {
            let task = &item[4..];
            let checkbox = &item[..3];
            let new_task = if task == what_to_change {
                if checkbox.contains("*") {
                    format!("{} {}\n", checkbox, change_to)
                } else {
                    format!("{} {}\n", checkbox, change_to)
                }
            } else {
                format!("{}\n", item)
            };
            result.push_str(&new_task);
        }
        buf.write_all(result.as_bytes()).unwrap()
    }

    // uncomplete Removes the strikethrough of a task that was previously marked as complete
    pub fn uncomplete(&self, args: &[String]) {
        // Reading file
        let stdout = io::stdout();
        let mut stdout_buf = BufWriter::new(stdout);

        let mut result = String::new();
        let contents = fs::read_to_string(&self.todo_path).unwrap();
        let todo_file = OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Could not open todo file");
        let mut buf = BufWriter::new(todo_file);

        for arg in args {
            if !contents.contains(arg) {
                eprintln!("Item '{}' not in todo-list, cannot uncomplete it!", arg);
                process::exit(2);
            };
        }

        for (_, item) in contents.lines().enumerate() {
            let task = String::from(&item[4..]);
            let checkbox = &item[..3];

            let restored_task = if checkbox.contains("*") && args.contains(&task) {
                format!("[ ] {}\n", &task)
            } else if args.contains(&task) {
                stdout_buf
                    .write("Item not previously completed, so no errors, but beware!".as_bytes())
                    .unwrap();
                format!("{}\n", item)
            } else {
                format!("{}\n", item)
            };
            result.push_str(&restored_task);
        }

        buf.write(result.as_bytes()).unwrap();
    }

    pub fn clear(&self) {
        let stdout = io::stdout();
        let mut buf = BufWriter::new(stdout);

        // This function clears the file because of the 'truncate' option
        // Probably not correct, but tuncate makes it work
        let _ = OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Could not open todo file");

        buf.write_all("Cleared todo file".as_bytes()).unwrap()
    }
}
