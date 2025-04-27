fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a*b + b*c + c*a;
}

fn take_u32(x: u32) {
    println!("u32: {x}");
}


fn take_i8(x: i8){
    println!("u8: {x}");
}

// exercise
fn fib(n: u32) -> u32{
    if n < 2{
        return n;
    }else{
        return fib(n-1) + fib(n-2);
    }
}


fn main() {
    let x = 10;
    println!("x: {x}");
    // x = 20; this gives an error as x is immutable. to allow edit use mut
    let mut y = 20;
    println!("y: {y}");
    y = 30;
    println!("y: {y}");

    //values
    let a = -1_000; //signed
    let b = 1_000; //unsigned
    let c = 3.14; // floating point number
    let d = 'a'; //char
    let e = true; // boolean
    println!("Signed: {a}");
    println!("Unsigned: {b}");
    println!("floating: {c}");
    println!("char: {d}");
    println!("Boolean: {e}");

    // Airthmetic
    println!("result: {}", interproduct(120, 100, 248));

    let x= 10;
    let y = 20;
    take_u32(x);
    take_i8(y);
    // take_u32(y);

    // exercise
    let n = 20;
    println!("fib({n}) = {}", fib(n));
} 
