trait Animal {
    fn leg_count(&self) -> u32;
}
trait Pet: Animal {
    fn talk(&self) -> String;

    fn greet(&self) {
        println!("what's your name? {}", self.talk());
    }
}

struct Dog {
    name: String,
    age: i8,
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!(
            "Woof! My name is {} and I'm {} years old",
            self.name, self.age
        )
    }
}

impl Animal for Dog {
    fn leg_count(&self) -> u32 {
        4
    }
}

#[derive(Debug)]
struct Meters(i32);

#[derive(Debug)]
struct MetersSquared(i32);

trait Multiply {
    type Output;
    fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Meters {
    type Output = MetersSquared;

    fn multiply(&self, other: &Self) -> Self::Output {
        MetersSquared(self.0 * other.0)
    }
}

#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn main() {
    let fido = Dog {
        name: "Fido".to_string(),
        age: 3,
    };

    fido.greet();
    println!("Fido has {} legs", fido.leg_count());

    println!("{:?}", Meters(10).multiply(&Meters(20)));

    let p1 = Player::default();
    let mut p2 = p1.clone();
    p2.name = "Player 2".to_string();
    println!("{:?}", p1);
    println!("{:?}", p2);

    tester_main();
}

use std::fmt::Display;

pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: impl Display);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: impl Display) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

fn do_things(logger: &impl Logger) {
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}

// TODO: Define and implement `VerbosityFilter`.
struct VerbosityFilter {
    max_verbosity: u8,
    inner: StderrLogger,
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: impl Display) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

fn tester_main() {
    let l = VerbosityFilter {
        max_verbosity: 3,
        inner: StderrLogger,
    };
    do_things(&l);
}
