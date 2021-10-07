use std::io;
use std::str::Split;
use rocksdb::{DB, Options};

fn main() -> io::Result<()> {
    help();
    let mut buffer = String::new();
    loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        let split: Vec<&str> = buffer.trim().split(" ").collect();
        if let Some(command) = split.get(0) {
            match *command {
                "help" => help(),
                "quit" => break,
                "put" => handle_put(&split),
                "get" => handle_get(&split),
                "delete" => handle_delete(&split),
                _ => println!("Unknown command: {}", buffer)
            }
        }
    }
    Ok(())
}

fn handle_put(cmd: &Vec<&str>) {
    if let Some(key) = cmd.get(1) {
        if let Some(value) = cmd.get(2) {
            println!("ADDING RECORD: {} -> {}", *key, *value);
        } else {
            println!("Value not specified")
        }
    } else {
        println!("Key not specified")
    }
}

fn handle_get(cmd: &Vec<&str>) {
    if let Some(key) = cmd.get(1) {
        println!("GETTING RECORD: {}", *key);
    } else {
        println!("Key not specified")
    }
}

fn handle_delete(cmd: &Vec<&str>) {
    if let Some(key) = cmd.get(1) {
        println!("DELETING RECORD: {}", *key);
    } else {
        println!("Key not specified")
    }
}

fn help() {
    println!("This is the RocksDB example.");
    println!("  put <key> <value> -- create or update a database record.");
    println!("  get <key>         -- get value from the database.");
    println!("  delete <key>      -- remove a record from the database.");
    println!("  help              -- list commands.");
    println!("  quit              -- exit the program.")
}
