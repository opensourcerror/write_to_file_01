use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::prelude::{DateTime, Local};

fn main() {

    let local_time: DateTime<Local> = Local::now();

    loop {
    println!("append to file: ");
    let mut write_to_file = String::new();
    std::io::stdin()
        .read_line(&mut write_to_file)
        .expect("Failed to write");
    
    if write_to_file.trim().eq("quit") {
        break;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("file_name_here2.txt")  
        .unwrap();

    if let Err(e) = writeln!(file, "{} {}",local_time,  write_to_file) {
        eprintln!("err writing to file: {}", e);
    }
}
}
    
   


