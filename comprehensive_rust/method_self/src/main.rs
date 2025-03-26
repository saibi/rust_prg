#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            laps: Vec::new(),
        }
    }

    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    fn print_laps(&self) {
        println!("Laps for {}: {} laps", self.name, self.laps.len());
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} seconds");
        }
    }

    fn finish(self) {
        let total = self.laps.iter().sum::<i32>();
        println!("{} finished in {} seconds", self.name, total);
    }
}

fn main() {
    let mut race = Race::new("Monaco GrandPrix");

    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
}
