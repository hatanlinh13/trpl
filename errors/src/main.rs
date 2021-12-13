#![allow(dead_code)]
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main()
{
    let _f = match File::open("hello.txt") {
        Ok(file) => {
            println!("Opened file: {:?}", file);
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => {
                    println!("Created file: {:?}", file);
                    file
                }
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => {
                panic!("Problem opening then file: {:?}", other_error);
            }
        },
    };

    let _ff = File::open("goodbye.txt").unwrap_or_else(|error| {
                                           if error.kind() == ErrorKind::NotFound {
                                               File::create("goodbye.txt").unwrap_or_else(|error| {
                                                   panic!("Problem creating the file: {:?}", error);
                                               })
                                           } else {
                                               panic!("Problem opening the file: {:?}", error);
                                           }
                                       });
}

fn read_username_from_file() -> Result<String, io::Error>
{
    let mut s = String::new();
    File::open("users.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_another_way() -> Result<String, io::Error>
{
    fs::read_to_string("users.txt")
}
