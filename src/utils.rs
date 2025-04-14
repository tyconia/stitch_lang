pub fn uuid_str(str: &str) -> uuid::Uuid {
    const NAMESPACE: uuid::Uuid = uuid::Uuid::NAMESPACE_URL;
    uuid::Uuid::new_v5(&NAMESPACE, str.as_bytes())
}

pub fn to_file<T: serde::Serialize>(serialize: T, file_name: &str) {
    use std::fs;
    use std::io::Write;

    let pretty = crate::ui::prettier_config();

    let ron_string = ron::ser::to_string_pretty(&serialize, pretty).expect("Serialization failed");

    let mut file = fs::File::create(file_name).unwrap();
    file.write_all(ron_string.as_bytes()).unwrap();
}
