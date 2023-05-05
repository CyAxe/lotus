use lotus::lua::parsing::files::filename_to_string;

#[test]
fn test_filename_to_string() -> Result<(), Box<dyn std::error::Error>> {
    let expected = "Hello, world!";
    let tmp_file = std::env::temp_dir().join("test.txt");
    std::fs::write(&tmp_file, expected)?;

    let result = filename_to_string(&tmp_file.to_string_lossy())?;
    assert_eq!(result, expected);

    Ok(())
}
