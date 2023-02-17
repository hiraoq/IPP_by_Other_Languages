pub fn load_dictionary(file_path: &str) -> Vec<&str> {
    let dic = vec![""];
    return dic;
}

#[test]
fn test_load_dictionary() {
    assert_eq!(load_dictionary("path"), vec!["".to_string()]);
}
