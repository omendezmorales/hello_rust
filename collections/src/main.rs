use collections::update_hm_for_wordcount;
use std::collections::HashMap;

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
    #[warn(unused_variables)]
    let _does_not_exist = v.get(100);
    //        println!("The first element is: {}", first)
    show_strings();
    println!("\nCalling hash map functions >>");

    println!("Calling hash map create function >>");
    print_hm(create_hm());
    println!("Updating a Value Based on the Old Value");
    update_hm();
    let text = "hello! world; wonderful? world";
    let counts = update_hm_for_wordcount(text);
    println!("word counts: {:?}", counts);
}

fn update_hm() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("prior to update> {:?}", scores);
    scores.insert(String::from("Blue"), 25);

    println!("after update> {:?}", scores);
    println!("Inserting a Value If the Key Has No Value, i.e. using entry method");
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!(" {:?}", scores);
}

fn show_strings() {
    let _s = String::new();

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

fn create_hm() -> HashMap<String, i32> {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);
    return scores;
}

fn print_hm(scores: HashMap<String, i32>) {
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
