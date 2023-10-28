fn main() {
    if let Err(e) = genpass::get_config() {
        eprint!("{}", e);
        std::process::exit(1);
    }
}