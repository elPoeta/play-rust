use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};

fn main() {
    let file_name = String::from("archivo");
    let extension = String::from("txt");
    let content = String::from("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.\n  elPoeta.");
    match write_file(&file_name, &extension, content) {
        Ok(()) => println!("Write file Ok!"),
        Err(e) => println!("Error {}", e),
    }

    match read_file(&file_name, &extension) {
        Ok(c) => println!("{}", c),
        Err(e) => println!("Error {}", e),
    }
}

fn write_file(file_name: &str, extension: &str, content: String) -> io::Result<()> {
    //let path = [file_name.clone(), extension.clone()].join(".");
    let path = String::from(file_name) + "." + extension;
    let mut buffer = BufWriter::new(File::create(path)?);
    buffer.write_all(content.as_bytes())?;
    buffer.flush()?;
    Ok(())
}

fn read_file(file_name: &str, extension: &str) -> io::Result<String> {
    //let path = [file_name.to_owned(), extension.to_owned()].join(".");
    // let path = file_name.to_owned() + "." + extension;
    let path = String::from(file_name) + "." + extension;
    let file = File::open(path)?;
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
