use std::fs::File;
use std::io::{stdin, Write};
use std::path::Path;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        None => {
            panic!("Missing csv file...");
        }
        Some(path) => path,
    }; // find the file that was dragged into the program as input

    let mut input_buffer = String::new(); // create an input buffer for the user to type in the divisor
    println!("Enter divisor to remove from csv file (leave blank for default(1) ): ");
    stdin()
        .read_line(&mut input_buffer)
        .expect("TODO: panic message"); // read the user input

    let modulo: usize = {
        match input_buffer.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => 1,
        }
    }; // number to modulo by to keep a lines

    println!("Do you want to deduplicate the csv file(y/n)?");
    stdin()
        .read_line(&mut input_buffer)
        .expect("TODO: panic message");
    let dedupe: bool = { input_buffer.trim().to_lowercase().contains("y") }; // TODO: allow the user to deduplicate and decimate the file

    let path = Path::new(file_path); // create a path for the file that was dragged in so we can later read the file.

    let file = match fs::read_to_string(path) {
        Ok(f) => f,
        Err(err) => {
            panic!("Unable to read file to string, error: {}", err);
        }
    }; // read the file into one single massive string

    let lines: Vec<&str> = file.split("\n").collect(); // collect all the lines split by a newline into a vector of string slices

    println!("Old line count: {}", lines.len()); // print out the line count of the original file

    println!("Modulo: {}", modulo); // print out the modulo from the user input so they can verify it.

    let new_lines: Vec<&str> = lines
        .iter() // create an iterator of the lines
        .enumerate() // turn the iterator into a list of &str into a list of (index, &str) tuples
        .filter(|(index, _)| index % modulo == 0) // apply modulo filter, if index % modulo == 0, keep the line, if not remove it.
        .map(|(_, line_string)| *line_string) // at this point the iter is full of (usize index, &&str line), this maps it into just &str, keeping each reference happy :)
        .collect();

    let mut dedupe_new_lines: Vec<&str> = new_lines.clone();
    dedupe_new_lines.dedup_by_key(|line| {
        let mut split_line = line.split(",").into_iter().peekable();
        split_line.next();
        //let mut output = String::new();
        let mut data_vec = vec![];
        loop {
            if split_line.peek().is_none() || split_line.peek().is_none() {
                break;
            }
            //output = format!("{},{}",output,split_line.next().unwrap());
            // let temp = split_line.next().unwrap();
            data_vec.push(split_line.next().unwrap().trim());
        }
        println!("Line key: {:?}", data_vec);
        data_vec
    });

    let new_file_name: String = {
        if !dedupe {
            format!(
                "./decimated_{}",
                path.file_name().unwrap().to_str().unwrap()
            )
        } else {
            format!(
                "./deduplicated_{}",
                path.file_name().unwrap().to_str().unwrap()
            )
        }
    }; // create the new file name with the word decimated_ prepended to it

    let new_file_path = Path::new(&new_file_name); // create a new path to that file name

    let mut new_file = File::create(new_file_path).unwrap(); // create the new file

    if dedupe {
        println!("Dedupe line count: {}", dedupe_new_lines.len());
        println!(
            "Dedupe line count change: {}",
            (lines.len() - dedupe_new_lines.len()) as i64 * -1
        );
        for line in dedupe_new_lines {
            match new_file.write_all(line.as_bytes()) {
                Ok(_) => {}
                Err(err) => {
                    panic!(
                        "Error writing line as bytes to file, line:{}, error: {}",
                        line, err
                    );
                }
            }; // append a line as bytes, panic the program if we fail to append it, we dont want to exit with a success for the program if the file output may be bad.
            match new_file.write_all("\n".as_bytes()) {
                Ok(_) => {}
                Err(err) => {
                    panic!(
                        "Error writing line as bytes to file, line: \\n, error: {}",
                        err
                    );
                }
            }; // append a "\n" after each line so we keep each line on its own line.
        } // write each line in the new_lines to the new file
    } else {
        println!("New line count: {}", new_lines.len()); // print out the line count of the new file
        for line in new_lines {
            match new_file.write_all(line.as_bytes()) {
                Ok(_) => {}
                Err(err) => {
                    panic!(
                        "Error writing line as bytes to file, line:{}, error: {}",
                        line, err
                    );
                }
            }; // append a line as bytes, panic the program if we fail to append it, we dont want to exit with a success for the program if the file output may be bad.
            match new_file.write_all("\n".as_bytes()) {
                Ok(_) => {}
                Err(err) => {
                    panic!(
                        "Error writing line as bytes to file, line: \\n, error: {}",
                        err
                    );
                }
            }; // append a "\n" after each line so we keep each line on its own line.
        } // write each line in the new_lines to the new file
    }
    let _ = new_file.flush(); // flush the file from the buffer to the system

    println!("Press enter to close program.");
    let _inp = stdin().read_line(&mut "".to_string()); // take user input just so we can wait for them to press enter, this lets them read the program output easier.
}
