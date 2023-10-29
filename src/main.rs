use genpass::{Config, Generator};

fn main() {
    let conf = Config::new().check();

    println!("{:?}", conf);

    let mut g = Generator::new();

    println!("{:?}", g.gen_word(4));
    println!("{:?}", g.gen_digits(6));
    println!("{:?}", g.gen_special(4));
}
