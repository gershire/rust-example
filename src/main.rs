use std::io;
use rocksdb::{DB, Options};

fn main() -> io::Result<()> {
    let path = "./data";
    let db = DB::open_default(path).unwrap();
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
                "put" => handle_put(&split, &db),
                "get" => handle_get(&split, &db),
                "delete" => handle_delete(&split, &db),
                _ => println!("Unknown command: {}", buffer)
            }
        }
    }
    // Uncomment the next line to destroy the database on exit
    // let _ = DB::destroy(&Options::default(), path);
    Ok(())
}

fn handle_put(cmd: &Vec<&str>, db: &rocksdb::DB) {
    if let Some(key) = cmd.get(1) {
        if let Some(value) = cmd.get(2) {
            println!("ADDING RECORD: {} -> {}", *key, *value);
            db.put(*key, *value).unwrap();
        } else {
            println!("Value not specified")
        }
    } else {
        println!("Key not specified")
    }
}

fn handle_get(cmd: &Vec<&str>, db: &rocksdb::DB) {
    if let Some(key) = cmd.get(1) {
        println!("GETTING RECORD: {}", *key);
        match db.get(*key) {
            Ok(Some(value)) => println!("the value is: {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("error: {}", e)
        }
    } else {
        println!("Key not specified")
    }
}

fn handle_delete(cmd: &Vec<&str>, db: &rocksdb::DB) {
    if let Some(key) = cmd.get(1) {
        println!("DELETING RECORD: {}", *key);
        db.delete(*key).unwrap();
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
