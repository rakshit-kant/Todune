/*
Todune: A Notes Taking CLI written in Rust
Copyright (C) 2026 Rakshit Kant

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

// use serde_json::to_string_pretty;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;

fn take_input() -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Invalid Input!");
    input.trim().to_string()
}

fn create_task(tasks: &mut Vec<String>) {
    println!();
    // Get Tasks
    print!("Enter the Task: ");
    io::stdout().flush().unwrap();
    let task: String = take_input();

    tasks.push(task);

    // Create a File Named tasks.json
    let storage = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("tasks.json")
        .expect("Could not create or open tasks.json");
    let mut storage_writer = BufWriter::new(storage);
    // Write the tasks to the tasks.json file
    serde_json::to_writer_pretty(&mut storage_writer, tasks).expect("Could Not Write Tasks");
    // Save the File
    storage_writer.flush().expect("Could Not Save Tasks");
}

fn view_tasks(tasks: &mut [String]) {
    println!();
    for (index, tasks) in tasks.iter().enumerate() {
        println!("{}. {}", index + 1, tasks);
    }
}

fn delete_task(tasks: &mut Vec<String>) {
    todo!("Make the Delete Task Functionality");
}

fn matcher(tasks: &mut Vec<String>) {
    println!();
    println!("1 => Create a Task");
    println!("2 => View all Tasks");
    println!("3 => View a Task");
    println!("4 => Delete a Task");
    print!("Enter a Number: ");
    io::stdout().flush().unwrap();
    let num = take_input().trim().parse().expect("Not a Number!");
    match num {
        1 => create_task(tasks),
        2 => view_tasks(tasks),
        // 3 => search_task(),
        4 => delete_task(tasks),
        _ => println!("Yeah!"),
    }
}

fn file_checker() -> bool {
    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file_path = project_dir.join("tasks.json");

    file_path.is_file()
}

fn file_reader() -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open("tasks.json")?;
    let reader = BufReader::new(file);

    let notes: Vec<String> = serde_json::from_reader(reader)?;

    Ok(notes)
}

fn main() {
    if !file_checker() {
        let mut tasks: Vec<String> = Vec::new();
        matcher(&mut tasks);
    } else {
        let mut tasks: Vec<String> = file_reader().expect("Notes Reader Failed");
        matcher(&mut tasks);
    }
}
