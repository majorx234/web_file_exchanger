use web_file_exchanger::backend::Backend;

fn main() {
    let backend = Backend::new();
    println!("web_file_exchanger_server: {}", backend.get_name());
}
