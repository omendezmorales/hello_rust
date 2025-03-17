// this is a sample code I've found in http://diego-pacheco.blogspot.com/2020/04/having-fun-with-rust.html
use std::collections::HashMap;
use std::io;

pub fn solution(length:u32) -> u32 {
    let mut cache:HashMap<u32,u32> = HashMap::new();
    let mut result:u32 = 0;
    for i in 0..length{
        let f = fibonacci(i,&mut cache);
        if f<=4000000 && f%2==0{
            result = result + f;     
        }
    }
    return result;
}

fn fibonacci(n:u32,cache:&mut HashMap<u32,u32>) -> u32 {
    if let Some(f) = cache.get(&n) {
        return *f;
    }
    let f:u32 = match n {
        0 | 1 => 1,
        _     => fibonacci(n - 1,cache) + fibonacci(n - 2,cache),
    };
    cache.insert(n,f);
    return f;
}

fn main() {
    println!("Please input your hash length.");
      let mut guess= String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");
    let guess:u32 =  match guess.trim().parse()
    {
        Ok(num) => num,

        Err(_) => 0,
    };
    let hash_found:u32 = solution(guess);
    println!("this is a hash: {:?}", hash_found);

}

