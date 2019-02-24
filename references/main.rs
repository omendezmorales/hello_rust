fn main() {
    let s1 = String::from("hello Orlando");
    let mut str2= String::from("hola");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    change(&mut str2);
    println!("Here's s2 again {}",str2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
