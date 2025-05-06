#[derive(Debug)]
struct CarRace{
    name: String,
    laps: Vec<i32>
}

// to associate functions to struct or enum use impl.
impl CarRace{
    // No receiver, a static method
    fn new(name: &str) -> Self{
        Self { name: String::from(name), laps: Vec::new() }
    }

    // Exclusive borrowed read-write access to self
    fn add_lap(&mut self, lap: i32){
        self.laps.push(lap);
    }

    // Shared and read-only borrowed access to self
    fn print_laps(&self){
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate(){
            println!("Lap {idx}: {lap} sec");
        }
    }

    // Exclusive ownership of self
    fn finish(self){
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}


//traits: abstract over types. similar to interfaces

// Super trait
trait Animal {
    fn leg_count(&self) -> u32;
}
trait Pet: Animal{
    fn talk(&self) -> String;

    fn greet(&self){
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}

struct Dog{
    name: String,
    age: i8
}

impl Animal for Dog{
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Pet for Dog{
    fn talk(&self) -> String {
        format!("Woof, my name is {}", self.name)
    }
}

// Associated type are placeholder types which are supplied by the trait implementation.
#[derive(Debug)]
struct Meters(i32);
#[derive(Debug)]
struct MeterSquared(i32);

//Derive: Supported traits can be automatically implemented for your custom types.
#[derive(Debug, Clone, Default)]
struct Player{
    name: String,
    strength: u8,
    hit_points: u8
}

trait Multiply{
    type Output;
    fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Meters{
    type Output = MeterSquared;
    fn multiply(&self, other: &Self) -> Self::Output {
        MeterSquared(self.0*other.0)
    }
}



// Exercise
trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

/// Only log messages up to the given verbosity level.
struct VerbosityFilter {
    max_verbosity: u8,
    inner: StderrLogger,
}

impl Logger for VerbosityFilter{
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity{
            self.inner.log(verbosity, message);
        }
    }
}

fn main() {
    let mut race = CarRace::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();

    let fido = Dog { name: String::from("Fido"), age: 5 };
    fido.greet();

    println!("{:?}", Meters(10).multiply(&Meters(20)));



    let p1 = Player::default(); // Default trait adds `default` constructor.
    let mut p2 = p1.clone(); // Clone trait adds `clone` method.
    p2.name = String::from("EldurScrollz");
    // Debug trait adds support for printing with `{:?}`.
    println!("{p1:?} vs. {p2:?}");

    let logger = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}
