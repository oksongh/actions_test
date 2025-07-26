fn unagi_check(s: &str) -> bool {
    s.contains("unagi")
}
fn main() {
    unagi_check("unagi");
    println!("Hello, world!");
}
#[test]
fn test_ci_desu() {
    assert!(unagi_check("unagi"));
    assert!(unagi_check("may unagi"));
    assert!(unagi_check("unagi is here"));

    assert!(!unagi_check("una"));
}
