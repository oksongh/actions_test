fn unagi_check(s: &str) -> bool {
    s.contains("unagi")
}

fn fish_check(s: &str) -> bool {
    s.contains("saba") || s.contains("hugu")
}
fn main() {
    unagi_check("unagi");
    println!("Hello, world!");
}
#[test]
fn test_ci_pass_desu() {
    assert!(unagi_check("unagi"));
    assert!(unagi_check("may unagi"));
    assert!(unagi_check("unagi is here"));

    assert!(!unagi_check("una"));
}
#[test]
fn test_ci_pass2_desu() {
    assert!(fish_check("saba"));
    assert!(fish_check("hugu"));
    assert!(fish_check("saba and hugu"));

    assert!(!fish_check("unagi"));
    assert!(!fish_check("una"));
    assert!(!fish_check("usagi"));
}
