use std::{
    fs::{self, File},
    path::Path,
};

use std::error::Error;

fn main() {
    // panic!("crash and burn");
    open_file();
    panic_show();
}

fn print_file(file_name: &str) -> Result<(), Box<dyn Error>> {
    let message: String = fs::read_to_string(file_name)?;
    println!("this is the contents of {}: {}", file_name, message);
    Ok(())
}
fn open_file() {
    let file_path = "hello.txt";
    let greeting_file_result = File::open(file_path);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file '{}'  {error:?}", file_path),
    };
    let path = Path::new(file_path);
    print_file(path.file_name().unwrap().to_str().unwrap());
}

fn panic_show() {
    let v = vec![1, 2, 3];

    v[99];
}
