use genpass::Password;

fn main() {
    if let Err(e) = genpass::get_config() {
        eprint!("{}", e);
        std::process::exit(1);
    }

    let s = String::from("a4");
    println!("{:?}",Password::from(s));
}