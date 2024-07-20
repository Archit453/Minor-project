use std::{collections::HashMap, fs::File, path::Path};

use serde::Deserialize;



fn main() {
    let action = std::env::args().nth(1).expect("Please provide an action");
    let item = std::env::args().nth(2).expect("Please provide an item");

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(msg) => println!("An error occurred: {}", msg),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(msg) => println!("An error occurred: {}", msg),
            },
        }
    }else if action == "show" {
        todo.show();
    }else if action == "show-c"{
        todo.show_c();
    }else if action == "show-ic"{
        todo.show_ic();
    } else if action == "remove" {
        match todo.remove(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo saved"),
                Err(msg) => println!("An error occurred: {}", msg),
            },
        }
    } else {
        println!("Unknown action: {}", action);
    }
}

struct Todo {
    map: HashMap<String, bool>,    // use rust built in HashMap to store key - val pairs
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }                               // insert a new item into our map.
                                    // active state is set to true by default.

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;

        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(bool_value) => Some(*bool_value = false),
            None => None,
        }
    }

    pub fn show(&self) {
        for (key, &val) in &self.map {
            let status = if val { "incomplete" } else { "complete" };
            println!("{}: {}", key, status);
        }
    }

    pub fn show_c(&self) {
        for (key, &val) in &self.map {
            let status = if val { "incomplete" } else { "complete" };
            if status == "complete" {
                println!("{} {}",key,status);
            }
        }
    }

    pub fn show_ic(&self) {
        for (key, &val) in &self.map {
            let status = if val { "incomplete" } else { "complete" };
            if status == "incomplete" {
                println!("{} {}",key,status);
            }
        }
    }

    fn remove(&mut self, key: &String) -> Option<()> {
        self.map.remove(key).map(|_| ())
    }
}