use urlshortener::validation::validate_url;

#[test]
fn test_valid_urls() {
    assert!(validate_url("https://example.com".to_string()).is_some());
    assert!(validate_url("https://sub.example.com/path".to_string()).is_some());
}

#[test]
fn test_invalid_urls() {
    assert!(validate_url("example.com".to_string()).is_none()); // Missing scheme
    assert!(validate_url("ftp://example.com".to_string()).is_none()); // Wrong scheme
    assert!(validate_url("".to_string()).is_none()); // Empty
}
