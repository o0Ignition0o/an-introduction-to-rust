fn lowest_above_zero(temperatures: Vec<i32>) -> i32 {
    temperatures.iter().cloned().filter(|t| t > &0).fold(
        i32::max_value(),
        |lowest_temperature, current| {
            if current < lowest_temperature {
                current
            } else {
                lowest_temperature
            }
        },
    )
}

enum ThingInTheSky {
    Bird(i32),
    Plane(bool),
    Superman,
    SomethingElse,
}

fn what_is_it(thing: ThingInTheSky) -> () {
    match thing {
        ThingInTheSky::Bird(number_of_wings) => {
            println!("Is that a bird ? it has {} wings!", number_of_wings)
        }
        ThingInTheSky::Plane(is_big) => println!("Is that a huge plane ? {}", is_big),
        ThingInTheSky::Superman => println!("No, it's superman ! ! !"),
    }
}

fn main() {
    let temperatures = vec![-12, -8, -1, 2, 3, 12];
    println!(
        "The lowest temperature above zero is {}",
        lowest_above_zero(temperatures) // The lowest temperature above zero is 2
    );
    what_is_it(ThingInTheSky::Superman);
}
