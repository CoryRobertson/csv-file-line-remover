use std::{env, fs};
use std::fs::File;
use std::io::{stdin, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        None => { panic!("Missing csv file..."); }
        Some(path) => { path }
    };

    let mut input_buffer = String::new();
    println!("Enter divisor to remove from csv file: ");
    stdin().read_line(&mut input_buffer).expect("TODO: panic message");

    let path = Path::new(file_path);

    let file = match fs::read_to_string(path) {
        Ok(f) => {f}
        Err(err) => { panic!("Unable to read file to string, error: {}", err); }
    };

    let lines: Vec<&str> = file.split("\n").collect(); // collect all the lines split by a newline into a vector of string slices

    println!("Old line count: {}", lines.len());

    let modulo: usize = {
        match input_buffer.trim().parse::<usize>() {
            Ok(num) => {num}
            Err(_) => { 2 }
        }
    }; // number to modulo by to keep a lines


    println!("Modulo: {}", modulo);

    let new_lines: Vec<&str> = lines.iter()
        .enumerate() // turn the iterator into a list of &str into a list of (index, &str) tuples
        .filter(|(index, _)| {
        index % modulo == 0
        }) // apply modulo filter, if index % modulo == 0, keep the line, if not remove it.
        .map(|(_, line_string )| *line_string )// at this point the iter is full of (usize index, &&str line), this maps it into just &str, keeping each reference happy :)
        .collect();

    println!("New line count: {}", new_lines.len());

    let new_file_name = format!("./decimated_{}", path.file_name().unwrap().to_str().unwrap());

    let new_file_path = Path::new(&new_file_name);

    let mut new_file = File::create(new_file_path).unwrap();

    for line in new_lines {
        match new_file.write_all(line.as_bytes()) {
            Ok(_) => {}
            Err(err) => { panic!("Error writing line as bytes to file, line:{}, error: {}", line, err); }
        };
        match new_file.write_all("\n".as_bytes()) {
            Ok(_) => {}
            Err(err) => { panic!("Error writing line as bytes to file, line: \\n, error: {}", err); }
        };
    }

    let _ = new_file.flush();

    println!("Press enter to close program.");
    let _inp = stdin().read_line(&mut "".to_string());

}
