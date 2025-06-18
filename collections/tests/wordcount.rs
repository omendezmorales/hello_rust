use collections::update_hm_for_wordcount;

#[test]
fn test_word_count_basic() {
    let text = "hello world hello";
    let counts = update_hm_for_wordcount(text);
    assert_eq!(counts["hello"], 2);
    assert_eq!(counts["world"], 1);
}

#[test]
fn test_word_count_empty() {
    let text = "";
    let counts = update_hm_for_wordcount(text);
    assert!(counts.is_empty());
}

#[test]
fn test_word_count_punctuation() {
    let text = "hello; world! hello?";
    let counts = update_hm_for_wordcount(text);
    assert_eq!(counts["hello"], 2);
    assert_eq!(counts["world"], 1);
}
