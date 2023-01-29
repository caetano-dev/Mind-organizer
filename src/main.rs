use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write};

fn read_lines(filename: &str) -> Vec<String> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file: {}", err);
            return vec![];
        }
    };
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                println!("Error reading line: {}", err);
                break;
            }
        };
        lines.push(line);
    }
    lines
}

fn write_lines(filename: &str, lines: Vec<String>) {
    let file = match File::create(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Error creating file: {}", err);
            return;
        }
    };
    let mut writer = BufWriter::new(file);
    let mut completed_tasks = Vec::new();
    let mut todos = Vec::new();
    let mut ideas = Vec::new();
    for line in lines {
        if line.starts_with("[x]") {
            completed_tasks.push(line);
        } else if line.starts_with("[]") {
            todos.push(line);
        } else if line.starts_with("-"){
            ideas.push(line);
        }
    }
    if !completed_tasks.is_empty() {
        writer.write_all(b"Completed tasks:\n").unwrap();
        for task in completed_tasks {
            writer.write_all(task.as_bytes()).unwrap();
            writer.write_all(b"\n").unwrap();
        }
        writer.write_all(b"\n").unwrap();
    }
    if !todos.is_empty() {
        writer.write_all(b"To-do's:\n").unwrap();
        for todo in todos {
            writer.write_all(todo.as_bytes()).unwrap();
            writer.write_all(b"\n").unwrap();
        }
        writer.write_all(b"\n").unwrap();
    }
    if !ideas.is_empty() {
        writer.write_all(b"Ideas:\n").unwrap();
        for idea in ideas {
            writer.write_all(idea.as_bytes()).unwrap();
            writer.write_all(b"\n").unwrap();
        }
    }
    writer.flush().unwrap();
}
fn main() {
    let lines = read_lines("file.txt");
    write_lines("file.txt", lines);
}
