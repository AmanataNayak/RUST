// function
// overloading is not supported always take fixed number of paramaters. Deafult arguemnts are not supported.
fn gcd(a: u32, b:u32) -> u32{
    if b > 0{
        return gcd(b, a%b);
    }else{
        return a;
    }
}

// exercise
fn collatz_length(mut n: i32) -> u32 {
    if n == 1{
        1
    }else if n%2==0{
        1 + collatz_length(n/2)
    }else {
        1 + collatz_length(3*n+1)
    }
}

fn main() {
    let z = 13;
    let x = {
        let y = 10;
        dbg!(y);
        z - y
    };
    dbg!(x);
    // dbg!(y); out of scope

    // if statements
    let x = 10;
    if x == 0 {
        println!("zero!");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }

    // if expersion
    let size = if x < 20 {"small"} else {"large"};
    println!("Size: {size}");

    // match expersion: can be used to check a value against one or more options:
    let val = 1;
    match val {
        1 => println!("one"),
        10 => println!("ten"),
        100 => println!("hundred"), 
        _ => {
            println!("Something else");
        }
    }

    let flag = true;
    let val = match flag {
        true => 1,
        false => 0
    };

    println!("The value of {flag} is {val}");

    // loop: while, for, loop
    // while
    let mut x = 250;
    while x >= 10 {
        x = x / 2;
        dbg!(x);
    }
    // for 
    for y in 1..5 {
        dbg!(y);
    }

    for y in [2, 4, 8, 16, 32, 64]{
        dbg!(y);
    }

    // loop
    let mut i = 0;
    loop {
        i += 1;
        dbg!(i);
        if i > 100{
            break;
        }
    }

    // break & continue
    let mut i = 0;
    loop {
        i += 1;
        if i > 5{
            break;
        }else if  i % 2 == 0{
            continue;
        } 
        dbg!(i);
    }

    // labels
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2{
        for j in 0..=2{
            elements_searched +=1;
            if s[i][j] == target_value{
                break 'outer;
            }
        }
    }
    dbg!(elements_searched);

    // function
    dbg!(gcd(20, 30));
    println!("Length: {}", collatz_length(11));
}
