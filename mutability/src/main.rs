fn main() {
    let a = "Hello";
    let b = a;

    println!("a is {} and b is {}", a, b);

    a = "World"; // cannot assign to an immutable value

    let mut c = "Hello"; // c owns "Hello"
    c = "World"; // Ok
    let d = &c; // d borrows "Hello" from c

    c = "How is it going ?"; // Cannot assign because it's borrowed

    d = &"I'm a rustacean and I love it !"; // Cannot assign because d is immutable

    let mut e = &"welcome to the rust intro !"; // Ok

    println!("Hello world, {}", e);
}
