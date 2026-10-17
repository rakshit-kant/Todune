/*one line to give the program's name and a brief idea of what it does.>
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
along with this program.  If not, see <https://www.gnu.org/licenses/>.*/

use std::io::Write;

struct Note {
    title: String,
    body: String,
}

fn take_input() -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Invalid Input!");

    input.trim().to_string()
}

fn main() {
    print!("Enter the Title of the Note: ");
    std::io::stdout().flush().unwrap();
    let title = take_input();

    print!("Enter the Body of the ");
    std::io::stdout().flush().unwrap();
    let body = take_input();

    let note = Note { title, body };

    println!("Title: {}", note.title);
    println!("Body: {}", note.body);
}
