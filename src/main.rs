mod config;
mod generator;

fn main() {
    let conf = config::Config::new().check();
    let passwords = generator::Generator::new().run(conf);

    passwords.iter().for_each(|x| println!("{}", x));
}
