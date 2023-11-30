use std::fs::File;
use std::io::ErrorKind;
use std::fs;
use std::io;


fn main() {
    // panic!("crash and burn");

    // let greeting_file = File::open("hello.txt").unwrap();
    read_file();
    read_file_with_closures();
    read_file_with_expect("expect.txt".to_string());
    let user = read_username_from_file();
    println!("This is the user read from the file {:#?}", user.unwrap());
}

fn _out_of_bounds() {
    let v = vec![1, 2, 3];

    v[99];
}

fn read_file() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn read_file_with_closures() {
    let _greeting_file = File::open("closure.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("closure.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });
}

fn read_file_with_expect(filename:String){
    let greeting_file = File::open(filename.clone())
    .expect( "file should be included in this project");

}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}