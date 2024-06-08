// use crate::print;

pub fn run(){
    let x = 1 ; 

    let y = 25.5; // default f64

   // add explicit type

    let z:i64 = 4545454545;

    // find max size
    println!("Max i32: {}",std::i32::MAX);

    // boolean
    let is_active = true;
    let is_greater:bool = 10 > 5;

    // let a1 = 'a';


    println!("{:?}",(x,y,z,is_active,is_greater));
}