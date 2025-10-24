use url_shortener::validate_url;

#[test]
fn test_valid_urls() {
    assert_eq!(validate_url("https://example.com".to_string()), Some(true));
    assert_eq!(validate_url("https://sub.example.com/path".to_string()), Some(true));
    assert_eq!(validate_url("https://github.com/user/repo".to_string()), Some(true));
}

#[test]
fn test_invalid_urls() {
    assert_eq!(validate_url("http://example.com".to_string()), Some(false)); // HTTP not HTTPS
    assert_eq!(validate_url("example.com".to_string()), None); // Missing scheme
    assert_eq!(validate_url("ftp://example.com".to_string()), Some(false)); // Wrong scheme
    assert_eq!(validate_url("".to_string()), None); // Empty
    assert_eq!(validate_url("not-a-url".to_string()), None); // Invalid format
}
