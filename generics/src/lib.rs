pub trait Summary
{
    fn summarize(&self) -> String
    {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle
{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle
{
    fn summarize(&self) -> String
    {
        format!("{}, by {} ({})", self.headline, self.author, &self.location)
    }

    fn summarize_author(&self) -> String
    {
        format!("@{}", self.author)
    }
}

pub struct Tweet
{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet
{
    fn summarize(&self) -> String
    {
        format!("{} -> {}", self.summarize_author(), self.content)
    }

    fn summarize_author(&self) -> String
    {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary)
{
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_broadcast<T>(item: &T)
    where T: Summary
{
    println!("Hi folks! Check out this breaking news!!! {}",
             item.summarize());
}

pub fn longer_string<'a>(x: &'a str, y: &'a str) -> &'a str
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
