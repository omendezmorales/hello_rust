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
        println!("seventh element is {}",&v[6]);
        let does_not_exist = &v[100];
        let does_not_exist = v.get(100);
//        println!("The first element is: {}", first)
}
