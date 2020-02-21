use std::ffi::OsStr;

//for a long discussion on the (context of the) code below, see: 
// https://fasterthanli.me/blog/2020/working-with-strings-in-rust/
fn main() {
    let arg = std::env::args()
    .skip(1)
    .next()
    .expect("should have one argument");
    println!("{}", arg.to_uppercase());

    match OsStr::to_str(OsStr::new(&arg)) {
        Some(arg) => println!("valid UTF-8: {}", arg),
        None => println!("not valid UTF-8: {:?}", arg),
    }
}
