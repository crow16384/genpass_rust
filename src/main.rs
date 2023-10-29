use genpass::Config;

fn main() {
    let conf = Config::new().check();

    println!("{:?}", conf);
}
