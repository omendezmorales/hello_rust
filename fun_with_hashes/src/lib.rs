use std::collections::HashMap;

pub fn solution(length: u32) -> u32 {
    let mut cache: HashMap<u32, u32> = HashMap::new();
    let mut result: u32 = 0;
    for i in 0..length {
        let f = fibonacci(i, &mut cache);
        if f <= 4_000_000 && f % 2 == 0 {
            result += f;
        }
    }
    result
}

pub fn fibonacci(n: u32, cache: &mut HashMap<u32, u32>) -> u32 {
    if let Some(f) = cache.get(&n) {
        return *f;
    }
    let f: u32 = match n {
        0 | 1 => 1,
        _ => fibonacci(n - 1, cache) + fibonacci(n - 2, cache),
    };
    cache.insert(n, f);
    return f;
}
