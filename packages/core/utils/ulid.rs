use ulid::Ulid;
pub fn ulid() -> String {
    Ulid::new().to_string().to_lowercase()
}
