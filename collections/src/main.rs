fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    //let first = &v[0];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    v.push(7);
    v.push(8);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    println!("seventh element is {}", &v[6]);
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    //        println!("The first element is: {}", first)
    show_strings();
}

fn show_strings() {
    let s = String::new();

    let data = "initial contents";

    let s = data.to_string();
    println!("{}", s);
    // the method also works on a literal directly:
    let mut s = "initial contents".to_string();
    s.push_str(" bar"); //efective appending
    println!("after updating s : {}", s);
    let s = String::from("initial contents");
    let hello = String::from("Здравствуйте");
    println!("{}, and in UNICODE: {}", s, hello);
    iterate_string_elements(hello);
}

fn iterate_string_elements(s: String) {
    println!("using characters");
    for c in s.chars() {
        print!(" {}", c);
    }
    println!("\n using bytes");

    for b in s.bytes() {
        print!(" {}", b);
    }
}
