#![allow(dead_code)]

trait Animal {
    fn eat(&self);
}

#[derive(Clone)]
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn eat(&self) {
        println!("Dog {} is eating", self.name);
    }
}

#[derive(Clone)]
struct Cat {
    name: String,
}

impl Animal for Cat {
    fn eat(&self) {
        println!("Cat {} is eating", self.name);
    }
}

fn animals_eat_mad(animals: &Vec<Box<dyn Animal>>) {
    for animal in animals {
        animal.eat();
    }
}

fn animals_eat(animals: &[Box<dyn Animal>]) {
    for animal in animals {
        animal.eat();
    }
}

fn animal_name_mad(name: &String) {
    println!("This is {}", name);
}

fn animal_name(name: &str) {
    println!("This is {}", name);
}

fn dog_me_mad(dog: &Box<Dog>) {
    println!("I am Dog {}", dog.name);
}

fn dog_me(dog: &Dog) {
    println!("I am Dog {}", dog.name);
}

fn main() {
    let oreo = "oreo".to_owned();
    let cheetos = "cheetos";
    animal_name(&oreo);
    animal_name(cheetos);

    let dog = Dog {
        name: cheetos.to_owned(),
    };
    dog_me(&dog);
    dog_me(&Box::new(&dog));

    let cat = Cat { name: oreo };
    let animals_vec: Vec<Box<dyn Animal>> = vec![Box::new(dog.clone()), Box::new(cat.clone())];
    let animals_array: [Box<dyn Animal>; 2] = [Box::new(dog), Box::new(cat)];
    animals_eat(&animals_vec);
    animals_eat(&animals_array);
}
