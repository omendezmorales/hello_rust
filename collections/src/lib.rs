use std::collections::HashMap;

pub fn update_hm_for_wordcount(text: &str) -> HashMap<&str, u32> {
    let mut map = HashMap::new();
    text.split(|c: char| {
        match c {
            '?' | '!' | ';' => true, // Punctuation delimiters
            _ => c.is_whitespace(),  // Whitespace delimiters
        }
    })
    .for_each(|word| {
        if !word.is_empty() {
            // Filter out empty strings from consecutive delimiters
            *map.entry(word).or_insert(0) += 1;
        }
    });
    map
}
