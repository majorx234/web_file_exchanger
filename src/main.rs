use web_file_exchanger::{backend::Backend, database_interface::DataBaseInterface};

fn main() {
    let mut dbi = DataBaseInterface::new();
    dbi.add("Heinz".to_string(), "1234".to_string());

    let backend = Backend::new();
    println!("web_file_exchanger_server: {}", backend.get_name());
    println!(
        "is Heinz in db? {}",
        dbi.compare_password(&"Heinz".to_string(), &"1234".to_string())
    );
}
