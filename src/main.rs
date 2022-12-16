use crate::ProgramDirection::{Dedupe, DedupeMod, Mod, ModDedupe};
use std::fs::File;
use std::io::{stdin, Write};
use std::path::Path;
use std::{env, fs};

static MOD_ARG: &str = "-m";
static DEDUPE_ARG: &str = "-d";
static MOD_DEDUPE_ARG: &str = "-md";
static DEDUPE_MOD_ARG: &str = "-dm";

#[derive(PartialEq, Clone)]
enum ProgramDirection {
    Mod,
    Dedupe,
    ModDedupe,
    DedupeMod,
}

fn direction_from_string(input: &String) -> Result<ProgramDirection, String> {
    if input.eq(MOD_ARG) {
        Ok(Mod)
    } else if input.eq(DEDUPE_ARG) {
        Ok(Dedupe)
    } else if input.eq(MOD_DEDUPE_ARG) {
        Ok(ModDedupe)
    } else if input.eq(DEDUPE_MOD_ARG) {
        Ok(DedupeMod)
    } else {
        Err("bad argument".to_string())
    }
}

struct ProgramOptions {
    file_path: String,
    program_direction: ProgramDirection,
    modulo: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let program_option: ProgramOptions = {
        // .expect("Error looking for input arguments for program direction, try running program with -m, -d, -md, -dm after file name.")
        let program_direction = match direction_from_string(args.get(2).unwrap_or(&"".to_string())) {
            Ok(dir) => {dir}
            Err(_) => {
                let mut input_buffer = String::new();
                loop {
                    println!("Input a valid program direction (-m, -d, -md, -dm):");
                    println!("-m modulo lines");
                    println!("-d deduplicate lines");
                    println!("-md modulo lines, then deduplicate lines");
                    println!("-dm deduplicate lines, then modulo lines");
                    let _ = stdin().read_line(&mut input_buffer);
                    if let Ok(dir) = direction_from_string(&input_buffer.trim().to_string()) {
                        break dir;
                    }
                }
            }
        };
        ProgramOptions {
            file_path: {
                match args.get(1) {
                    None => {
                        let mut input_buffer = String::new();
                        println!("Missing csv file, input a file name for the program to use:");
                        let _ = stdin().read_line(&mut input_buffer);
                        input_buffer.trim().to_string()
                        // panic!(
                        //     "Missing csv file, try running program with argument of a filename that exists."
                        // );
                    }
                    Some(path) => path.to_string(),
                }
            },
            program_direction: program_direction.clone(),
            modulo: {
                let modulo_input = args
                    .get(3)
                    .unwrap_or(&"".to_string())
                    .trim()
                    .parse::<usize>();
                if let Ok(modu) = modulo_input {
                    modu
                } else {
                    let mut input_buffer = String::new();
                    loop {
                        if program_direction == Dedupe { break 1; }
                        println!("Input a modulo to use for line removal(empty for default of 1):");

                        let _ = stdin().read_line(&mut input_buffer);
                        if let Ok(dir) = input_buffer.trim().parse::<usize>() {
                            break dir;
                        }
                        if input_buffer.trim().is_empty() { break 1; }
                    }
                }
            },
        }
    }; // args are as follows: "<filename> <ProgramDirection> <OPTIONAL modulo>"

    let path = Path::new(&program_option.file_path); // create a path for the file that was dragged in so we can later read the file.

    let file = match fs::read_to_string(path) {
        Ok(f) => f,
        Err(err) => {
            panic!("Unable to read file to string, error: {}", err);
        }
    }; // read the file into one single massive string

    let lines: Vec<&str> = file.split('\n').collect(); // collect all the lines split by a newline into a vector of string slices

    println!("Old line count: {}", lines.len()); // print out the line count of the original file

    let new_file_name: String = {
        match program_option.program_direction {
            Mod => {
                format!(
                    "./decimated_{}",
                    path.file_name().unwrap().to_str().unwrap()
                )
            }
            Dedupe => {
                format!(
                    "./deduplicated_{}",
                    path.file_name().unwrap().to_str().unwrap()
                )
            }
            ModDedupe => {
                format!(
                    "./decimated_deduplicated_{}",
                    path.file_name().unwrap().to_str().unwrap()
                )
            }
            DedupeMod => {
                format!(
                    "./deduplicated_decimated_{}",
                    path.file_name().unwrap().to_str().unwrap()
                )
            }
        }
    }; // create the new file name with the word decimated_ prepended to it

    let new_file_path = Path::new(&new_file_name); // create a new path to that file name

    let mut new_file = File::create(new_file_path).expect("Unable to create new file, missing permissions possibly?"); // create the new file

    // FIXME: currently running two in of the functions one after another seems to not sync the file data correctly. Maybe move to holding the file in a massive string?
    let new_file_line_count = match program_option.program_direction {
        Mod => {
            println!("Mod");
            modulo_line_count(&mut new_file, &lines, program_option.modulo)
        }
        Dedupe => {
            println!("Dedupe");
            dedupe_file(&mut new_file, &lines)
        }
        ModDedupe => {
            println!("ModDedupe");
            let intermediary_line_count = modulo_line_count(&mut new_file, &lines, program_option.modulo);
            println!("Intermediary line count after modulo: {}", intermediary_line_count);
            let _ = new_file.sync_all();
            let _ = new_file.flush();
            let _ = new_file.sync_all(); // these dont seem to work, going to do further testing on another platform.
            dedupe_file(&mut new_file, &lines)
        }
        DedupeMod => {
            println!("DedupeMod");
            let intermediary_line_count = dedupe_file(&mut new_file, &lines);
            println!("Intermediary line count after dedupe: {}", intermediary_line_count);
            let _ = new_file.sync_all();
            let _ = new_file.flush();
            let _ = new_file.sync_all(); // these dont seem to work, going to do further testing on another platform.
            modulo_line_count(&mut new_file, &lines, program_option.modulo)
        }
    };

    println!(
        "Total line change amount: {}",
        lines.len() - new_file_line_count
    );
    let percent_change = (new_file_line_count as f32 / lines.len() as f32) * 100.0;
    println!("Total line percentage change: {:.2}%", percent_change);
    println!("New line count: {}", new_file_line_count);

    let _ = new_file.flush(); // flush the file from the buffer to the system
}

fn modulo_line_count(new_file: &mut File, lines: &Vec<&str>, modulo: usize) -> usize {
    // else, user decided to modulo the file line-count
    let new_lines: Vec<&str> = lines
        .iter() // create an iterator of the lines
        .enumerate() // turn the iterator into a list of &str into a list of (index, &str) tuples
        .filter(|(index, _)| index % modulo == 0) // apply modulo filter, if index % modulo == 0, keep the line, if not remove it.
        .map(|(_, line_string)| *line_string) // at this point the iter is full of (usize index, &&str line), this maps it into just &str, keeping each reference happy :)
        .collect();

    println!("Modulo: {}", modulo); // print out the modulo from the user input so they can verify it.

    for line in &new_lines {
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
    new_lines.len()
}

fn dedupe_file(new_file: &mut File, lines: &Vec<&str>) -> usize {
    let mut dedupe_new_lines: Vec<&str> = lines.clone();

    dedupe_new_lines.dedup_by_key(|line| {
        let mut split_line = line.split(',').into_iter().peekable();
        split_line.next();
        let mut output = String::new();
        loop {
            if split_line.peek().is_none() || split_line.peek().is_none() {
                break;
            }
            output = format!("{},{}", output, split_line.next().unwrap());
        }
        output
    });

    for line in &dedupe_new_lines {
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
    dedupe_new_lines.len()
}
