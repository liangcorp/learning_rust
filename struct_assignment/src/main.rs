/**
 * Author: Chen Liang
 * Description: Create a structure named as Student
 *              Fields:
 *                  - Name
 *                  - Marks of three subjects
 *              Create instance of struct using associated function
 *              Find which subject has the highest mark using methods
 *              Can we create multiple impl blocks?
 */
struct Student {
    name: String,
    math: u32,
    physics: u32,
    chemistry: u32,
}

impl Student {
    fn create(name: String, math: u32, physics: u32, chemistry: u32)
                                                          -> Student {
        Student {
            name,
            math,
            physics,
            chemistry,
        }
    }
}

impl Student {
    fn highest_mark(&self) -> String {
        if self.math > self.physics && self.math > self.chemistry {
            String::from("math")
        }
        else if self.physics > self.math && self.physics > self.chemistry {
            String::from("physics")
        }
        else {
            String::from("chemistry")
        }
    }
}

fn main() {
    let bob = Student::create(String::from("Bob"), 78, 90, 67);
    println!("Subject that has the highest mark for {} is {}", bob.name, bob.highest_mark());
}
