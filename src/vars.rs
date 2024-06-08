pub fn run(){
    let name = "Krish";
    let mut age = 19;
    println!("My name is {} and I am {}",name,age);
    age = 20;

    println!("My name is {} and I am {}",name,age);

    // define constant

    const ID: i32 = 001;
    println!("ID: {}",ID);

    // assign multiple vars
    let (my_name,my_age) = ("Krish",19);
    println!("{} is {}",my_name,my_age);
  
}