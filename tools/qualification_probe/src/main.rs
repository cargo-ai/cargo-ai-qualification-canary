fn main() {
    if let Err(error) = qualification_probe::run(std::env::args().nth(1).as_deref()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
