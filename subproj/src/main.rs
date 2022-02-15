#[tokio::main]
async fn main() {
    console_subscriber::ConsoleLayer::builder()
    .server_addr(([127, 0, 0, 1], 5555))
    .init();
    println!("Hello, worldleb!");
    let mastr = get_string();
    println!("{mastr}");
    let mst2 = get_string();
}

fn get_string() -> String {
    String::from("hilloaaaab")
}
