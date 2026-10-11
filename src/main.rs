use std::io;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand::RngExt;

const START_GOLD: u64 = 100;
const FOOD_PRICE: u64 = 5;
const MAX_SEED: u64 = 9999;
const Buy: u64 = 1;
const Hunt: u64 = 2;
const Trade: u64 = 3;
const Status: u64 = 4;
const Quit: u64 = 5;

const CaravanDice: u64 = 3;
const CaravanMulti: u64 = 2;


fn print_menu() {
println!("1. Buy food");
println!("2. Hunt");
println!("3. Trade with caravan");
println!("4. View status");
println!("5. Quit");

}

fn can_afford(gold: u64, cost: u64) -> bool {

gold >= cost

}
fn hunt_food(roll: u64) -> u64 {

match roll {
1 => 0,
2 | 3 => 10,
4 | 5 => 20,
6 => 40,
_ => 0,
  }

}

fn read_number(min: u64, max: u64) -> u64 {
loop {
println!("Enter a number from {min} to {max}:");

let mut input = String::new();

            io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<u64>() {
            Ok(number) if number >= min && number <= max => {
                return number;
            }
            _ => {
                println!("Invalid number. Try again.");
           }
       }
   }
}
fn roll_die(seed: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    rng.random_range(1..=6)
}

fn roll_many(seed: u64, count: u64) -> u64 {
    let mut total = 0;

    for i in 0..count {
        total += roll_die(seed + i);
    }

    total
}

fn print_status(gold: u64, food: u64, day: u64) {
println!("Gold: {gold}");
println!("Food: {food}");
println!("Day: {day}");
}

fn main() {
print_menu();

 println!("{}", can_afford(100,5));

println!("Hunt roll 1: {}", hunt_food(1));
println!("Hunt roll 3: {}", hunt_food(3));
println!("Hunt roll 5: {}", hunt_food(5));
println!("Hunt roll 6: {}", hunt_food(6));

let number = read_number(1, 5);
println!("You entered: {number}");

println!("Rolling: {}", roll_die(100));

println!("Total: {}", roll_many(100, 3));

print_status(100, 0, 1);
let seed = read_number(1, MAX_SEED);

    let mut gold = START_GOLD;
    let mut food = 0;
    let mut day = 1;
    let mut roll_count = 0;

    loop {
        print_menu();

        let choice = read_number(Buy, Quit);

        match choice {
            Buy => {
                if !can_afford(gold, FOOD_PRICE) {
                    println!("You cannot afford any food.");
                    continue;
                }

                let max_food = gold / FOOD_PRICE;
                let quantity = read_number(1, max_food);

                gold -= quantity * FOOD_PRICE;
                food += quantity;
                day += 1;

                println!("You bought {quantity} food.");
            }

            Hunt => {
                let roll = roll_die(seed + roll_count);
                roll_count += 1;

                let food_found = hunt_food(roll);
                food += food_found;
                day += 1;

                println!("You hunted and found {food_found} food.");
            }

            Trade => {
                let total = roll_many(seed + roll_count, CavaranDice);
                roll_count += CavaranDice;

                gold += CavaranDice * total;
                day += 1;

                println!("The caravan gave you {} gold.", CavaranMulti * total);
            }

            Status => {
                print_status(gold, food, day);
            }

            Quit => {
                println!("Final status:");
                print_status(gold, food, day);
                break;
            }

            _ => unreachable!(),
        }
    }
}


