pub fn generate_token() -> String {
    uuid::Uuid::new_v4()
        .to_string()
        .chars()
        .filter(|x| *x != '-')
        .collect::<String>()
}
