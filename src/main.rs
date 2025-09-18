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
        let mut rng = rand::rng();
        for animal in &mut self.animals {
            let damage = rng.random_range(0..=20) as f32;
            animal.apply_damage(damage);
        }
        self.hour += 1;
    }

    fn feed_animals(&mut self) {
        let mut rng = rand::rng();
        let monkey_boost = rng.random_range(10..=25) as f32;
        let giraffe_boost = rng.random_range(10..=25) as f32;
        let elephant_boost = rng.random_range(10..=25) as f32;

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




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animal_creation_defaults() {
        let monkey = Animal::new(AnimalType::Monkey);
        assert_eq!(monkey.animal_type, AnimalType::Monkey);
        assert_eq!(monkey.health, 100.0);
        assert_eq!(monkey.state, AnimalState::Alive);
    }

    #[test]
    fn test_apply_damage_reduces_health() {
        let mut giraffe = Animal::new(AnimalType::Giraffe);
        giraffe.apply_damage(20.0); // 20% of 100 → 80
        assert!((giraffe.health - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_apply_damage_monkey_death_threshold() {
        let mut monkey = Animal::new(AnimalType::Monkey);
        monkey.health = 25.0;
        monkey.apply_damage(10.0); // health goes down
        assert_eq!(monkey.state, AnimalState::Dead);
    }

    #[test]
    fn test_apply_damage_giraffe_death_threshold() {
        let mut giraffe = Animal::new(AnimalType::Giraffe);
        giraffe.health = 45.0;
        giraffe.update_state();
        assert_eq!(giraffe.state, AnimalState::Dead);
    }

    #[test]
    fn test_elephant_cannot_walk_then_dead() {
        let mut elephant = Animal::new(AnimalType::Elephant);
        elephant.health = 65.0;
        elephant.update_state();
        assert_eq!(elephant.state, AnimalState::CannotWalk);

        // apply more damage to trigger death
        elephant.apply_damage(10.0);
        assert_eq!(elephant.state, AnimalState::Dead);
    }

    #[test]
    fn test_feed_increases_health_but_not_above_100() {
        let mut monkey = Animal::new(AnimalType::Monkey);
        monkey.health = 90.0;
        monkey.feed(50.0); // should cap at 100
        assert_eq!(monkey.health, 100.0);
    }

    #[test]
    fn test_dead_animals_do_not_change_state_or_health() {
        let mut giraffe = Animal::new(AnimalType::Giraffe);
        giraffe.health = 0.0;
        giraffe.state = AnimalState::Dead;

        giraffe.apply_damage(50.0);
        giraffe.feed(50.0);

        assert_eq!(giraffe.health, 0.0);
        assert_eq!(giraffe.state, AnimalState::Dead);
    }

    #[test]
    fn test_zoo_creation_contains_15_animals() {
        let zoo = Zoo::new();
        assert_eq!(zoo.animals.len(), 15); // 5 of each
    }

    #[test]
    fn test_zoo_pass_hour_increases_time() {
        let mut zoo = Zoo::new();
        let initial_hour = zoo.hour;
        zoo.pass_hour();
        assert_eq!(zoo.hour, initial_hour + 1);
    }

    #[test]
    fn test_zoo_feed_animals_caps_health() {
        let mut zoo = Zoo::new();
        // set all animals near max health
        for animal in &mut zoo.animals {
            animal.health = 99.0;
        }
        zoo.feed_animals();
        for animal in &zoo.animals {
            assert!(animal.health <= 100.0);
        }
    }
}
