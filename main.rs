use crate::archive::arch::arch_file as arp;
use rand::{Rng,thread_rng};
use rand::distributions::Alphanumeric;


mod archive;

fn main(){
    arp("somefile.txt");

    let mut rng = rand::thread_rng();
    let a:i32 = rng.gen();
    println!("{}",a);
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    println!("Gen string: {}", rand_string);
    


    
}


