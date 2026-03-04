// use std::{io::Error, num};

// fn main() {
//     let io_err: Error = Error::last_os_error();
//     let parse_err: num::ParseIntError = "not a number".parse::<i32>().unwrap_err();

//     let err1: Box<Error> = From::from(io_err);
//     let err2: Box<Error> = From::from(parse_err);
// }

use std::error::Error;
use std::io;
use std::num;

fn main() {
    let io_err: io::Error = io::Error::last_os_error();
    let parse_err: num::ParseIntError = "not a number".parse::<i32>().unwrap_err();

    // OK, here are the conversions.
    let err1: Box<dyn Error> = From::from(io_err);
    let err2: Box<dyn Error> = From::from(parse_err);
}
