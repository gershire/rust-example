use std::io;
use std::sync::Arc;
use rocksdb::DB;

mod kafka_consumer;

#[tokio::main]
async fn main() {
    let bootstrap_server = "localhost:9093";
    let topic = "test-topic";
    let path = "./data";
    let db = Arc::new(DB::open_default(path).unwrap());

    let mut rx = kafka_consumer::listen(bootstrap_server, topic).await;

    let dbr = db.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            handle_message(msg, dbr.as_ref()).await;
        }
    });

    cli_loop(db.as_ref());
}

async fn handle_message(message: String, db: &rocksdb::DB) {
    println!("Handling message {}", message);
    let option = message.trim().find(" ");
    match option {
        Some(index) => {
            let key = &message[0..index];
            let value = &message[index..];
            db.put(key, value);
        }
        None => println!("No key present!")
    }
}

fn cli_loop(db: &rocksdb::DB) {
    let mut buffer = String::new();
    loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer);
        let split: Vec<&str> = buffer.trim().split(" ").collect();
        if let Some(command) = split.get(0) {
            match *command {
                "help" => help(),
                "quit" => break,
                "get" => handle_get(&split, &db),
                "delete" => handle_delete(&split, &db),
                _ => println!("Unknown command: {}", buffer)
            }
        }
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
    println!("  get <key>         -- get value from the database.");
    println!("  delete <key>      -- remove a record from the database.");
    println!("  help              -- list commands.");
    println!("  quit              -- exit the program.")
}
