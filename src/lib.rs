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
        println!("in list!");
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
        println!("in add!");
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
            println!("{}", arg);

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

            println!("{}", line);
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
                println!("item to be deleted: {}", task);
                continue;
            } else {
                println!("item to keep around: {}", task);
                result.push_str(format!("{}\n", item).as_str())
            };
        }
        println!("result.trim(): {}.", result.trim());

        buf.write_all(result.as_bytes()).unwrap()
    }

    pub fn update() {
        println!("in update!")
    }
}
