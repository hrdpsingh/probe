use hostname::get;

fn main() {
    let hostname = get().expect("Failed to get hostname");
    println!("Hostname: {}", hostname.to_string_lossy());
}