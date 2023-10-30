pub fn add_int(a: i32, b: i32) -> i32 { 
    return a + b;
}

#[test]
fn test_add() {
    assert_eq!(add_int(1, 2), 3);
}