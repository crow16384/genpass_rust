use genpass::{Config, Generator};

fn main() {
    let conf = Config::new().check();
    let password = Generator::new().run(conf);

    println!("{}", password);
}
