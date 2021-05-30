/**
 * Lifetime Ellision
 * 1. Each parameter that is a reference gets its own
 *      lifetime parameter
 * 2. If there is exactly one input lifetime parameter, that lifetime
 *      is assigned to all output lifetime parameters
 * 3. If there are multiple input lifetime parameter, but one of them is
 *      &self or &mut self because this is a method, the lifetime of
 *      self is assigned to all output lifetime parameters.
 */

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "Hello";
    let s2 = "Bye";

    let result = longest(s1, s2);
    println!("{}", result);
}