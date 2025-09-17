use eframe::{egui, epi};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Alive,
    CannotWalk,
    Dead,
}

#[derive(Debug, Clone)]
struct Animal {
    species: &'static str,
    health: f32,
    state: State,
}

impl Animal {
    fn new(species: &'static str) -> Self {
        Self {
            species,
            health: 100.0,
            state: State::Alive,
        }
    }

    fn apply_damage(&mut self, percentage: f32) {
        if self.state == State::Dead {
            return;
        }

        self.health -= self.health * (percentage / 100.0);
        self.health = self.health.max(0.0);
        self.update_state();
    }

    fn feed(&mut self, percentage: f32) {
        if self.state == State::Dead {
            return;
        }

        self.health += self.health * (percentage / 100.0);
        if self.health > 100.0 {
            self.health = 100.0;
        }
        self.update_state();
    }

    fn update_state(&mut self) {
        match self.species {
            "Elephant" => {
                if self.state == State::CannotWalk && self.health < 70.0 {
                    self.state = State::Dead;
                } else if self.health < 70.0 {
                    self.state = State::CannotWalk;
                } else {
                    self.state = State::Alive;
                }
            }
            "Monkey" => {
                if self.health < 30.0 {
                    self.state = State::Dead;
                } else {
                    self.state = State::Alive;
                }
            }
            "Giraffe" => {
                if self.health < 50.0 {
                    self.state = State::Dead;
                } else {
                    self.state = State::Alive;
                }
            }
            _ => {}
        }
    }
}

struct ZooApp {
    animals: Vec<Animal>,
    hours_passed: u32,
}

impl ZooApp {
    fn new() -> Self {
        let mut animals = Vec::new();
        for _ in 0..5 {
            animals.push(Animal::new("Monkey"));
            animals.push(Animal::new("Giraffe"));
            animals.push(Animal::new("Elephant"));
        }
        Self { animals, hours_passed: 0 }
    }

    fn pass_time(&mut self) {
        let mut rng = rand::thread_rng();
        for animal in &mut self.animals {
            let damage = rng.gen_range(0.0..=20.0);
            animal.apply_damage(damage);
        }
        self.hours_passed += 1;
    }

    fn feed_animals(&mut self) {
        let mut rng = rand::thread_rng();
        let monkey_bonus = rng.gen_range(10.0..=25.0);
        let giraffe_bonus = rng.gen_range(10.0..=25.0);
        let elephant_bonus = rng.gen_range(10.0..=25.0);

        for animal in &mut self.animals {
            match animal.species {
                "Monkey" => animal.feed(monkey_bonus),
                "Giraffe" => animal.feed(giraffe_bonus),
                "Elephant" => animal.feed(elephant_bonus),
                _ => {}
            }
        }
    }
}

impl epi::App for ZooApp {
    fn name(&self) -> &str {
        "Zoo Simulator"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Zoo Simulator");
            ui.label(format!("Hours passed: {}", self.hours_passed));

            if ui.button("Pass an hour").clicked() {
                self.pass_time();
            }

            if ui.button("Feed the animals").clicked() {
                self.feed_animals();
            }

            ui.separator();
            ui.heading("Animals");
            for animal in &self.animals {
                ui.label(format!(
                    "{} - Health: {:.1}%, State: {:?}",
                    animal.species, animal.health, animal.state
                ));
            }
        });
    }
}

fn main() {
    let app = ZooApp::new();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
