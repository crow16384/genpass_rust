mod config;
mod generator;

fn main() {
    let conf = config::Config::new().check();
    let password = generator::Generator::new().run(conf);

    println!("{}", password);
}

/*#[cfg(test)]
mod test {
    use super::{Config, PassElements};
    use std::convert::TryFrom;

    fn convert_valid_element() {
        let expected = Config { format: vec![Ok(PassElements::Word(8))] };
        let actual = Config::try_from(&String::from("w8"));
        assert!(actual.is_ok(), "valid element should be converted to Config");
        //assert_eq!(actual.unwrap(), expected, "wrong element value");
    }
}*/
