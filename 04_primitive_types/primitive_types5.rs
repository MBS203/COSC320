fn main() {
    let cat = ("Furry McFurson", 3.5);

    // Destructuring the tuple into variables `name` and `age`
    let (name, age) = cat;

    // Corrected println! syntax using named arguments
    println!("{name} is {age} years old", name = name, age = age);
}
