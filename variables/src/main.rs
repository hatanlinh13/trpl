const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

fn main()
{
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    x = THREE_HOURS_IN_SECONDS;
    println!("The value of x is: {}", x);
}
