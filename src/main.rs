mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod conditions;
mod functions;
fn main() {
    functions::run();
    conditions::run();
    arrays::run();
    tuples::run();
    vars::run();
    types::run();
    print::run();
    strings::run();
    println!("{} is better than {}","Messi","Ronaldo"); // basic formatting  
    println!("{0} is better than {1} but {1} is better than {0}","Messi","Ronaldo"); // positional arguments
    println!("{name} likes to play {game} ",name="Messi",game="Football"); // named arguments
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10); // binary, hex, octal
    println!("{:?}",(12,true,"hello")); // debug trait
    println!("10 * 10 = {}",10*10); // basic math

}
