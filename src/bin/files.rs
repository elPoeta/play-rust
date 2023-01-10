use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};

fn main() {
    let file_name = String::from("archivo");
    let extension = String::from("txt");
    let content = String::from("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.\n  elPoeta.");
    match write_file(&file_name, &extension, &content) {
        Ok(()) => println!("Write file Ok!"),
        Err(e) => println!("Error {}", e),
    }

    match read_file(&file_name, &extension) {
        Ok(c) => println!("{}", c),
        Err(e) => println!("Error {}", e),
    }
}

fn write_file(file_name: &String, extension: &String, content: &String) -> io::Result<()> {
    let mut buffer = BufWriter::new(File::create(
        [file_name.to_owned(), extension.to_owned()].join("."),
    )?);

    buffer.write_all(content.as_bytes())?;
    buffer.flush()?;
    Ok(())
}

fn read_file(file_name: &String, extension: &String) -> io::Result<String> {
    let file = File::open([file_name.to_owned(), extension.to_owned()].join("."))?;
    let reader = BufReader::new(file);
    let mut content = String::new();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                content.push_str(&l);
                content.push_str("\n");
            }
            Err(_) => continue,
        }
    }

    Ok(content)
}
