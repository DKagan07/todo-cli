use std::fs::OpenOptions;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::{env, fs, process};
use vibrance::style;

pub struct Todo {
    pub todo: Vec<String>,
    pub todo_path: String,
}

impl Todo {
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

    // Now make CRUD things
    pub fn list(&self) {
        println!("in list!");
        let stdout = io::stdout();

        let mut buf = BufWriter::new(stdout);
        let mut result = String::new();

        // Reading file
        let contents = fs::read_to_string(&self.todo_path).unwrap();
        for (num, item) in contents.lines().enumerate() {
            let num = num + 1;
            let item = &item[4..];
            let is_completed = &item[..4];
            let i = item.to_string();

            if is_completed.contains("*") {
                result = format!("{} {}", num, style::strikethrough(i));
            } else {
                result = format!("{} {}", num, i);
            }
        }
        buf.write_all(result.as_bytes()).unwrap()
    }
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
        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }

            // this is how the todo tasks are defined
            let line = format!("[ ] {}\n", arg);

            println!("{}", line);
            buf.write_all(line.as_bytes()).unwrap()
        }
    }
    pub fn complete() {
        println!("in complete!")
    }
    pub fn delete() {
        println!("in delete!")
    }
    pub fn update() {
        println!("in update!")
    }
}
