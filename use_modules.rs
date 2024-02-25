// Brining name into scope with "use" keyword

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {

            }
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use a::series::of;
use TrafficLight::{Red, Yellow};
// use TrafficLight::*;
fn main() {
    a::series::of::nested_modules();    //without use

    of::nested_modules();       // with use

    let red = Red;
    let red = Yellow;
    let green = TrafficLight::Green;
}