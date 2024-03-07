pub fn find_matches(
    content: &str,
    pattern: &str,
    mut write: impl std::io::Write,
) -> std::io::Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(write, "{}", line)?;
        }
    }
    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result).unwrap();
    assert_eq!(result, b"lorem ipsum\n");
}
