#![allow(dead_code)]

// Named Struct
struct Person {
    name: String,
    age: u32
}

fn describe(person: &Person){
    println!("{} is {} years old", person.name, person.age);
}

// Tuple Struct
struct Point(i32, i32);

// Enum
#[derive(Debug)]
enum Direction {
    Left,
    Right
}

#[derive(Debug)]
enum PlayerMove {
    Pass, //Simple variant
    Run(Direction), // Tuple variant
    Teleport {x: i32, y: i32} // Struct ariant
}

// repr enum
#[repr(u32)]
enum Bar{
    A,
    B = 1000,
    C
}


// Type aliases: Create a name for another type. The 2 type can be used interchengably.
enum CarryableConcreteItem{
    Left,
    Right
}

type Item = CarryableConcreteItem;

// Const: Constants are evaluated at the complie time and their value are inlined whenever they used.
const DIGEST_SIZE: usize = 3;
const FILL_VALUE: u8 = calculate_fill_value();

const fn calculate_fill_value() -> u8{
    if DIGEST_SIZE < 10 {
        42
    } else {
        13
    }
}

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [FILL_VALUE; DIGEST_SIZE];
    for (index, &b) in text.as_bytes().iter().enumerate(){
        digest[index % DIGEST_SIZE] = digest[index % DIGEST_SIZE].wrapping_add(b);
    }    
    digest
}


// Static: static variable live during whole execution of program, and therefore will not move.
static BANNER: &str = "Welcome to RustOS 3.14";

// exercies:
#[derive(Debug)]
enum Event {
    ButtonPressed(Button),
    CarArrived(Floor),
    CarDoorOpened,
    CarDoorClodes
}

type Floor = i32;

#[derive(Debug)]
enum LiftDirection{
    Up,
    Down
}

#[derive(Debug)]
enum Button{
    LobbyCall(LiftDirection, Floor),
    CarFloor(Floor)
}

fn car_arrived(floor: i32) -> Event{
    Event::CarArrived(floor)
}

fn car_door_opened() -> Event{
    Event::CarDoorOpened
}

fn car_door_closed() -> Event{
    Event::CarDoorClodes
}

fn lobby_call_button_pressed(floor: i32, dir: LiftDirection) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

fn car_floor_button_pressed(floor: i32) -> Event{
    Event::ButtonPressed(Button::CarFloor(floor))
}

fn main() {
    let mut amanata = Person{
        name: String::from("Amanata"),
        age: 21
    };
    describe(&amanata);

    amanata.age = 25;
    describe(&amanata);

    let name = String::from("ama");
    let age = 39;
    let person = Person { name, age};
    describe(&person);

    let jackie = Person {
        name: String::from("Jackie"),
        ..amanata
    };
    describe(&jackie);

    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);

    let direction = Direction::Left;
    let player_move = PlayerMove::Run(direction);
    println!("On this trun: {player_move:?}");

    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);

    let digest = compute_digest("hello");
    println!("Digest: {digest:?}");

    println!("{BANNER}");


    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, LiftDirection::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}