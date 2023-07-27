use add_one::add;
use add_two::add as add_2;

fn main() {
    let a = add(10);
    println!("Hello, world!, plush 10 use add_one: {}", a);
    let a = add_2(10);
    println!("Hello, world!, plush 10 use add_two: {}", a);
}
