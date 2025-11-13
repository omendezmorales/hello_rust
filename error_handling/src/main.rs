use std::{
    fs::{self, File},
    path::Path,
};

use std::error::Error;

fn main() {
    // panic!("crash and burn");
    open_file();
    open_file2();
    panic_show();
}

fn print_file(file_name: &str) -> Result<(), Box<dyn Error>> {
    let message: String = fs::read_to_string(file_name)?;
    println!("this is the contents of {}:\n {}", file_name, message);
    Ok(())
}
fn open_file() {
    let file_path = "hello.txt";
    let greeting_file_result = File::open(file_path);

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file '{}'  {error:?}", file_path),
    };
    let path = Path::new(file_path);
    let _ = print_file(path.file_name().unwrap().to_str().unwrap());
}

fn open_file2() {
    use std::fs::File;
    use std::io::ErrorKind;
    let file_name = "hello.txt";
    let _greeting_file = File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error| {
                panic!("Problem creating the file {} : {error:?}", file_name);
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
fn panic_show() {
    let v = vec![1, 2, 3];

    v[99];
}
