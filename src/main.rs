use rand::Rng;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum AnimalType {
    Monkey,
    Giraffe,
    Elephant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AnimalState {
    Alive,
    Dead,
    CannotWalk,
}

#[derive(Debug, Clone)]
struct Animal {
    animal_type: AnimalType,
    health: f32,
    state: AnimalState,
}

impl Animal {
    fn new(animal_type: AnimalType) -> Self {
        Animal {
            animal_type,
            health: 100.0,
            state: AnimalState::Alive,
        }
    }

    fn apply_damage(&mut self, percentage: f32) {
        if self.state == AnimalState::Dead {
            return;
        }
        self.health -= self.health * (percentage / 100.0);
        if self.health < 0.0 {
            self.health = 0.0;
        }
        self.update_state();
    }

    fn feed(&mut self, percentage: f32) {
        if self.state == AnimalState::Dead {
            return;
        }
        self.health += self.health * (percentage / 100.0);
        if self.health > 100.0 {
            self.health = 100.0;
        }
        self.update_state();
    }

    fn update_state(&mut self) {
        match self.animal_type {
            AnimalType::Monkey => {
                if self.health < 30.0 {
                    self.state = AnimalState::Dead;
                } else {
                    self.state = AnimalState::Alive;
                }
            }
            AnimalType::Giraffe => {
                if self.health < 50.0 {
                    self.state = AnimalState::Dead;
                } else {
                    self.state = AnimalState::Alive;
                }
            }
            AnimalType::Elephant => {
                if self.health < 70.0 {
                    if self.state == AnimalState::CannotWalk {
                        self.state = AnimalState::Dead;
                    } else {
                        self.state = AnimalState::CannotWalk;
                    }
                } else {
                    self.state = AnimalState::Alive;
                }
            }
        }
    }
}

struct Zoo {
    animals: Vec<Animal>,
    hour: u32,
}

impl Zoo {
    fn new() -> Self {
        let mut animals = Vec::new();
        for _ in 0..5 {
            animals.push(Animal::new(AnimalType::Monkey));
            animals.push(Animal::new(AnimalType::Giraffe));
            animals.push(Animal::new(AnimalType::Elephant));
        }
        Zoo { animals, hour: 0 }
    }

    fn pass_hour(&mut self) {
        let mut rng = rand::thread_rng();
        for animal in &mut self.animals {
            let damage = rng.gen_range(0..=20) as f32;
            animal.apply_damage(damage);
        }
        self.hour += 1;
    }

    fn feed_animals(&mut self) {
        let mut rng = rand::thread_rng();
        let monkey_boost = rng.gen_range(10..=25) as f32;
        let giraffe_boost = rng.gen_range(10..=25) as f32;
        let elephant_boost = rng.gen_range(10..=25) as f32;

        for animal in &mut self.animals {
            match animal.animal_type {
                AnimalType::Monkey => animal.feed(monkey_boost),
                AnimalType::Giraffe => animal.feed(giraffe_boost),
                AnimalType::Elephant => animal.feed(elephant_boost),
            }
        }
    }

    fn display(&self) {
        println!("\n===== ZOO STATUS (Hour: {}) =====", self.hour);
        for (i, animal) in self.animals.iter().enumerate() {
            let a_type = match animal.animal_type {
                AnimalType::Monkey => "Monkey",
                AnimalType::Giraffe => "Giraffe",
                AnimalType::Elephant => "Elephant",
            };
            let a_state = match animal.state {
                AnimalState::Alive => "Alive",
                AnimalState::Dead => "Dead",
                AnimalState::CannotWalk => "Cannot Walk",
            };
            println!(
                "{}: {} | Health: {:.1}% | State: {}",
                i + 1,
                a_type,
                animal.health,
                a_state
            );
        }
        println!("===============================\n");
    }
}

fn main() {
    let mut zoo = Zoo::new();
    loop {
        zoo.display();
        println!("Choose an action: (1) Pass Hour, (2) Feed, (3) Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => zoo.pass_hour(),
            "2" => zoo.feed_animals(),
            "3" => {
                println!("Exiting simulator.");
                break;
            }
            _ => println!("Invalid input, try again."),
        }
    }
}
