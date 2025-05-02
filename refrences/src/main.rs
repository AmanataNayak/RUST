fn main(){
    // Refrence: A refrence provide a way to access another value without taking ownership of the value, and is called borrowing.
    let a = 'A';
    let b = a;
    
    let mut r: &char = &a;
    dbg!(*r);

    r = &b;
    dbg!(*r);   

    //mutable refrence: allow changing the value they refer to.
    let mut point = (1,2);
    let x_cord = &mut point.0;
    // point.0 = 100; // here it will faile as the x_cord is still alive.
    *x_cord = 20;
    println!("point: {point:?}");
    point.0 = 100;
    println!("point: {point:?}");

    //Slice: it give a view into larger collection.
    let a = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s = &a[2..4];
    println!("s: {s:?}");

    //slice from 0th index
    let s = &a[..4];
    println!("s: {s:?}");

    //slice to last
    let s = &a[2..];
    println!("s: {s:?}");
    

    // Strings
    // 1. &str: is a slice of UTF-8 encoded bytes.
    // 2. String: is an owned buffer of UTF-8 encoded bytes.

    let s1 = "world";
    println!("s1: {s1}");

    let mut s2 = String::from("Hello ");
    println!("s2: {s2}");

    s2.push_str(s1);
    println!("s2: {s2}");
    let s3 = &s2[2..9];
    println!("s3: {s3}");

    // Refrences validity: 
    // 1. refrences can never be null
    // 2. refrence can't outline the data they point to.
    // let x_ref = {
    //     let x = 10;
    //     &x
    // };
    // dbg!(x_ref);

    // the above commented code will give error as the x is out of scope when the debugger is called.
}