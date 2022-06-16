use std::io::{self, Read, Write};
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use comfy_table::*;
use comfy_table::presets::UTF8_FULL;

fn display_tasks(filepath: &str) -> Result<(), io::Error> { 
    let mut file = match File::options()
        .read(true)
        .open(filepath) {
        Err(_) => {
            File::options()
                .read(true)
                .open(filepath)
                .expect("Failed to open file")
        },
        Ok(file) => file
    };

    // Get contents of file to string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read to string");

    // Display contents of file in table
    let mut table = Table::new();

    // Hashmap for table coloring
    let mut status_to_color = HashMap::<&str, comfy_table::Color>::new();
    status_to_color.insert("done", Color::Green);
    status_to_color.insert("pending", Color::Red);

    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(80)
        .set_header(vec![
            Cell::new("#").add_attribute(Attribute::Bold),
            Cell::new("Task").add_attribute(Attribute::Bold),
            Cell::new("Status").add_attribute(Attribute::Bold)
        ]);

    if contents != " " && contents != "\n"{
        // add rows if contents is not empty
        for (idx, line) in contents.trim().trim_matches('\n').split("\n").enumerate() {
            let cells = line.split(",").take(2).collect::<Vec<&str>>();

            let task = cells[0];
            let status = cells[1];

            table.add_row(vec![
                Cell::new(idx),
                Cell::new(task),
                Cell::new(status)
                    .fg(status_to_color[&status])
            ]);
        }
    }
    println!("{table}");

    Ok(())
}

pub fn add(filepath: &str, task: &str) -> Result<(), io::Error> {
    let mut file = match File::options()
        .read(true)
        .write(true)
        .open(filepath) {
            
        Err(_) => {
            File::options()
                .read(true)
                .write(true)
                .create(true)
                .open(filepath)
                .expect("Failed to create file")
        },
        Ok(file) => file
    };

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read to string");

    let mut task_trimmed: String = String::from(task.trim().trim_matches('\n'));
    task_trimmed.push_str(",pending\n");

    file.write(task_trimmed.as_bytes())
        .expect("Failed to write Add");

    display_tasks(filepath).expect("Expect: Failed to display tasks");

    Ok(())
}


pub fn remove<'a>(filepath: &'a str, task_num: &'a usize) -> Result<(), &'a str> {
    // First check if file exists
    if !Path::exists(Path::new(filepath)) {
        return Err("File does not exist");
    }

    let mut file = match File::options()
        .read(true)
        .open(filepath) {
            
        Err(_) => {
            File::options()
                .read(true)
                .create(true)
                .open(filepath)
                .expect("Failed to create file")
        },
        Ok(file) => file
    };

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read to string");

    let mut lines: Vec<&str> = contents.trim().trim_matches('\n').split("\n").collect();

    if lines.len() > *task_num {
        lines.remove(*task_num);

        let mut file = File::options()
            .read(true)
            .write(true)
            .truncate(true)
            .open(filepath)
            .expect("Failed to create file");

        let mut lines = lines.join("\n");
        lines.push_str("\n");
        file.write(lines.as_bytes())
            .expect("Failed to write after removal");

        display_tasks(filepath).expect("Expect: Failed to display tasks");

        Ok(())
    } else {
        Err("Task number out of bounds")
    }
}

pub fn list(filepath: &str) -> Result<(), io::Error> {
    display_tasks(filepath).expect("Expect: Failed to display tasks");

    Ok(())
}
