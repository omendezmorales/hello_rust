use fun_with_hashes::solution;

#[test]
fn test_fibonacci_small_numbers() {
    let mut cache = HashMap::new();
    assert_eq!(fibonacci(0, &mut cache), 1);
    assert_eq!(fibonacci(1, &mut cache), 1);
    assert_eq!(fibonacci(2, &mut cache), 2);
    assert_eq!(fibonacci(3, &mut cache), 3);
    assert_eq!(fibonacci(4, &mut cache), 5);
}

#[test]
fn test_fibonacci_cache_reuse() {
    let mut cache = HashMap::new();
    fibonacci(10, &mut cache);
    // Cache should now be populated
    assert_eq!(cache.len(), 11);
    assert_eq!(cache.get(&10), Some(&89));
}

#[test]
fn test_solution_even_values() {
    // Should sum even fibonacci numbers <= 4,000,000
    assert_eq!(solution(100), 4_613_732);
}
