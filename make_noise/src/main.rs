trait MakesNoise {
    fn make_noise(self) -> ();
}

struct Dog;
struct Cat;

impl MakesNoise for Dog {
    fn make_noise(self) -> () {
        println!("WOOF WOOF");
        // Who let the dogs out ?
    }
}

impl MakesNoise for Cat {
    fn make_noise(self) -> () {
        println!("Everybody wants to be a cat !");
        // Because the cat's the only cat, who knows where it's at
    }
}

fn main() -> () {
    let a = Dog {};
    let b = Cat {};
    a.make_noise();
    b.make_noise();
}
