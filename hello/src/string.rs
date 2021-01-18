pub fn run(){
    let mut hello = String::from( "Hello world");

    println!("Length:{}" ,hello.len());

    //hello.push_str("orld!");

    println!("capacity :{}", hello.capacity());

    println!("Cotains 'World' {}", hello.contains("Hello"));

    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(10, s.capacity());
    println!("{}", s);


    println!("{}", hello);



}