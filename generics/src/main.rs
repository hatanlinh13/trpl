#![allow(dead_code)]

use generics::longer_string;
use generics::Summary;
use generics::Tweet;
use std::fmt::Display;

fn largest<T>(list: &[T]) -> T
    where T: PartialOrd + Copy
{
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_with_ref<'a, T>(list: &'a [T]) -> &'a T
    where T: PartialOrd
{
    let mut largest = &list[0];
    for item in list {
        if *item > *largest {
            largest = item;
        }
    }
    largest
}

struct Point<T>
{
    x: T,
    y: T,
}

impl<T> Point<T>
{
    fn x(&self) -> &T
    {
        &self.x
    }
}

impl Point<f32>
{
    fn distance_from_origin(&self) -> f32
    {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixPoint<T, U>
    where T: Clone,
          U: Clone
{
    x: T,
    y: U,
}

impl<T, U> MixPoint<T, U>
    where T: Clone,
          U: Clone
{
    fn mixup<V, W>(&self, other_point: MixPoint<V, W>) -> MixPoint<T, W>
        where T: Clone,
              U: Clone,
              V: Clone,
              W: Clone
    {
        MixPoint { x: self.x.clone(),
                   y: other_point.y.clone() }
    }
}

fn return_summarizable() -> impl Summary
{
    Tweet { username: String::from("Someone"),
            content: String::from("Blah blah"),
            reply: false,
            retweet: false }
}

struct ImportantExcerpt<'a>
{
    part: &'a str,
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main()
{
    let number_list = vec![34, 50, 22, 71, 100, 22];
    let result = largest(&number_list);
    println!("The largest number is: {}", result);

    let number_list = vec![234, 6546, 12, 325, 32, 32532, -2];
    let result = largest(&number_list);
    println!("The largest number is: {}", result);

    let char_list = vec!['a', 'c', 'z', 'x'];
    let result = largest(&char_list);
    println!("The largest char is: {}", result);

    let _integer_point = Point { x: 5, y: 10 };
    let _float_point = Point { x: 2.0, y: 4.0 };
    let _integer_and_float = MixPoint { x: 5, y: 4.0 };

    let tweet = Tweet { username: String::from("someone"),
                        content: String::from("of course, as you already know, people"),
                        reply: false,
                        retweet: false };

    println!("one new tweet: {}", tweet.summarize());

    // lifetimes
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longer_string(string1.as_str(), string2);
    println!("The longer string is: {}", result);

    let novel = String::from("Call me someone. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let _i = ImportantExcerpt { part: first_sentence };
}
