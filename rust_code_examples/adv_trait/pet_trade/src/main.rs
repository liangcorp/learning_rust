struct Dog {
    id: u32,
    name: String,
    age: u32,
}

impl Dog {
   fn new(id: u32, name: String, age: u32) -> Self {
       Dog {
           id,
           name,
           age
       }
   }
}

trait Playable {

}

impl Playable for Dog {

}

trait Trade: Playable {
    type Pet;
    fn trade(&self, other: &Self::Pet) -> bool;
}

impl Trade for Dog {
    type Pet = Dog;
    fn trade(&self, other: &Self::Pet) -> bool {
        self.id > other.id
    }
}

fn main() {
    let dog1 = Dog::new(1, String::from("Dog1"), 10);
    let dog2 = Dog::new(2, String::from("Dog2"), 5);
    if Trade::trade(&dog1, &dog2) {
        println!("Trade accepted");
    } else {
        println!("Trade failed");
    }
}

