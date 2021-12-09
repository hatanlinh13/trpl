fn main()
{
    // loop
    let mut count = 0;
    let result = 'counting_up: loop {
        println!("Count: {}", count);
        let mut remaining = 10;
        loop {
            println!("Remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up count * 2;
            }
            remaining -= 1;
        }
        count += 1;
    };
    println!("End count: {}", count);
    println!("Result: {}", result);

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF AGAIN!!!");
}
