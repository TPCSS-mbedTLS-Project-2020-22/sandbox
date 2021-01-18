pub fn run(){
    println!("hello from the internal");

    println!("Number: {}", 1);

    println!("{} is from {}", "Brad", "Mass");

    println!("{0} is from {1}. {0} is also like {2}", "Brad", "Mass", "Code");

    println!("{name} likes to play {ball}", name = "Sambit", ball = "Cricket");

    println!("Binary: {:b} Hex: {:x} octal {:o}", 10, 10, 10);

    println!("{:?}", (12, true , "hello"));

    println!("10 + 19 = {}", 10+19);
}