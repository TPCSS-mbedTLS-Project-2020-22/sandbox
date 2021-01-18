pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers[2] = 10;

    numbers.push(5);
    numbers.push(20);

    numbers.pop();
    println!("{:?}", numbers);

    println!("{}", numbers[0]);
    
    println!("{}", numbers.len());

    //println!("{}", std::mem::size_of_val(&numbers))

    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);


    for x in numbers.iter(){
        println!("NUmber : {}", x);
    }

    for x in numbers.iter_mut(){
        *x *=2
    }
    println!("Numbers : {:?}", numbers);
}