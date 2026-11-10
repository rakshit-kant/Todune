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
use std::fs::File;
use std::io::{self, BufWriter, Write};

fn take_input() -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Invalid Input!");
    input.trim().to_string()
}

fn create_task(tasks: &mut Vec<String>) {
    // Get Tasks
    print!("Enter the Task: ");
    io::stdout().flush().unwrap();
    let task: String = take_input();

    tasks.push(task);

    // Create a File Named tasks.json
    let storage = File::create("tasks.json").expect("Could not create tasks.json");
    let mut storage_writer = BufWriter::new(storage);
    // Put the contents of tasks to the tasks.json file
    serde_json::to_writer_pretty(&mut storage_writer, tasks).expect("Could Not Write Tasks");
    storage_writer.flush().expect("Could Not Save Tasks");
}

// fn view_tasks() {}

fn matcher(tasks: &mut Vec<String>) {
    println!("1 => Create a Task");
    println!("2 => View all Tasks");
    println!("3 => Search a Task");
    println!("4 => Delete a Task");
    print!("Enter a Number: ");
    io::stdout().flush().unwrap();
    let num = take_input().trim().parse().expect("Not a Number!");
    match num {
        1 => create_task(tasks),
        // 2 => view_tasks(),
        // 3 => search_task(),
        // 4 => delete_task(),
        _ => println!("Yeah!"),
    }
}

fn main() {
    let mut tasks: Vec<String> = Vec::new();
    matcher(&mut tasks);
}
