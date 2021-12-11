#[derive(Debug)]
enum IpAddr
{
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message
{
    Quit,
    Move
    {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message
{
    fn call(&self)
    {
        dbg!(self);
    }
}

#[derive(Debug)]
enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8
{
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[derive(Debug)]
enum UsState
{
    Alabama,
    Alaska,
    // --snip--
}

fn plus_one(x: Option<i32>) -> Option<i32>
{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main()
{
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    dbg!(&home);
    dbg!(&loopback);

    let msg = Message::Write(String::from("Hello!"));
    msg.call();
    let mov = Message::Move { x: 11, y: 22 };
    mov.call();
    let clr = Message::ChangeColor(255, 255, 255);
    clr.call();
    let quit = Message::Quit;
    quit.call();

    // Option enum
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(&five);
    dbg!(&six);
    dbg!(&none);

    // if let syntax
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1
    }
}
