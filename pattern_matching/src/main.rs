//irrefurtable patterns
fn takes_tuple(tuple: (i32, char, bool)) {
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;
    println!("a:{a}, b:{b}, c:{c}");

    let (a, b, c) = tuple;
    println!("a:{a}, b:{b}, c:{c}");

    let (_, b, c) = tuple;
    println!("a:{a}, b:{b}, c:{c}");

    let (.., c) = tuple;
    println!("a:{a}, b:{b}, c:{c}");
}

fn new_tuples(tuple: (i32, char, bool, u8)) {
    let (first, .., last) = tuple;
    println!("first: {first}, last: {last}");
}


struct Foo {
    x: (u32, u32),
    y: u32
}

enum RESULT {
    Ok(i32),
    Err(String)
}


fn divide_in_two(n: i32) -> RESULT{
    if n % 2 == 0{
        RESULT::Ok(n/2)
    }else {
        RESULT::Err(format!("cannot divide {n} into two equal parts"))
    }
}


// Let control flow
use std::time::Duration;

// 1. if let expression
fn sleep_for(secs: f32) {
    if let Ok(duration) = Duration::try_from_secs_f32(secs){
        std::thread::sleep(duration);
        println!("slept for {duration:?}");
    }
}

// 2. while let
fn while_let(){
    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(c) = name.pop() {
        dbg!(c);
    }
}

// let else: For the common case of matching a pattern and returning from the function, use let else. 
// The “else” case must diverge (return, break, or panic - anything but falling off the end of the block).
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String>{
    let Some(s) = maybe_string else {
        return Err(String::from("got None"));
    };

    let Some(first_byte_char) = s.chars().next() else {
        return Err(String::from("got empty string"));
    };

    let Some(digit) = first_byte_char.to_digit(16) else {
        return Err(String::from("not a hex digit"));
    };

    Ok(digit)
}

fn main() {
    takes_tuple((777, 'a', true));
    new_tuples((77, 'a', true, 20));

    let input = 'x';
    match input {
        'q' => println!("Quiting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _ => println!("Something else"),
    }

    // destructuring structs
    let foo = Foo{x: (1, 2), y: 2};
    match foo {
        Foo {x: (1, b), y} => println!("x.0 = 1, b = {b}, y = {y}"), 
        Foo {y: 2, x: i} => println!("y = 2, x = {i:?}"),
        Foo {y, ..} => println!("y = {y}, other fields were ignored")
    }

    // destructuring tuples
    let n = 101;
    match divide_in_two(n) {
        RESULT::Ok(half) => println!("{n} divide by two is {half}"),
        RESULT::Err(msg) => println!("sorry, an error happened: {msg}")
    }

    sleep_for(-10.0);
    sleep_for(0.8);

    while_let();

    println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));
}
