mod backend;
mod frontend;

fn main() {
    #[cfg(feature = "server")]
    print_addr();

    frontend::run();
}

#[cfg(feature = "server")]
fn print_addr() {
    use std::env;

    let ip = env::var("IP").unwrap_or("127.0.0.1".into());
    let port = env::var("PORT").unwrap_or("8080".into());
    eprintln!("Serving at: {}:{}", ip, port);
}
