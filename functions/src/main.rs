fn main()
{
    println!("Hello, world!");

    another_function(15);
    print_labeled_measurement(11, 'm');

    let y = five();
    println!("The value of y is: {}", y);

    let z = plus_one(10);
    println!("The value of z is: {}", z);
}

fn another_function(x: u32)
{
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: u32, unit_label: char)
{
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> u32
{
    5
}

fn plus_one(x: u32) -> u32
{
    x + 1
}
