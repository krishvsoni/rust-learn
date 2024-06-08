pub fn run(){
    let mut hello = String::from("Hello");
    println!("{}",hello);
    println!("Length: {}",hello.len());
    hello.push('W');
    hello.push_str("orld");
    println!("Length: {}",hello.len());

    for word in  hello.split_whitespace(){
        println!("{}",word);
    }

    let mut s = String::with_capacity(10);
    s.push(char::from(65));

    // assertion testing
    assert!(s.capacity() >= 10);
    assert_eq!(s.len(),1);


}