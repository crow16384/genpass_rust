#[derive(Debug)]
enum Ex {
    One(u8),
    Two(u8),
}

fn main() {
    let conf = genpass::get_config();

    println!("{:?}",conf);
    println!("{:?}",conf.format[0]);

   let ee = Ex::Two(10);
   if let Ex::Two(d) = ee {
    println!("{:?}",d);
   }
   //println!("{:?}",ee[0]);

}