use spg::cli::parse_args;
use spg::clipboard::clip_copy;
use spg::generator::generate_password;

fn main() {
    let cli = parse_args();
    let password = generate_password(cli.length, cli.symbols);

    println!("{password}");

    match clip_copy(&password) {
        Ok(_) => eprintln!("Copied to clipboard"),
        Err(err) => eprintln!("Clipboard error: {err}"),
    }
}
