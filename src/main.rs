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
fn main() {
print_menu();

 println!("{}", can_afford(10,5));



println!("Hunt roll 1: {}", hunt_food(1));
println!("Hunt roll 3: {}", hunt_food(3));
println!("Hunt roll 5: {}", hunt_food(5));
println!("Hunt roll 6: {}", hunt_food(6));
}

