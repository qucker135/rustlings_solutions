// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
    println!("cat name: {}, cat age: {}", cat.0, cat.1);

    cat.0 = "L";
    cat.1 = 2.0;

    println!("{} is {} years old.", name, age);
    println!("cat name: {}, cat age: {}", cat.0, cat.1);

}
