#[allow(dead_code)]
struct Person {
    pub first_name: String,
    pub last_name: Option<String>,
    pub age: i32,
}

fn main() {
    let mut persons: Vec<Person> = vec![Person {
        first_name: "Nouman".to_string(),
        last_name: Some("Azam".to_string()),
        age: 1,
    }];

    persons.push(Person {
        first_name: "Kamran".to_string(),
        last_name: Some("Khan".to_string()),
        age: 2,
    });

    persons.push(Person {
        first_name: "Rahul".to_string(),
        last_name: None,
        age: 6,
    });

    persons.push(Person {
        first_name: "Imran".to_string(),
        last_name: Some("Rehman".to_string()),
        age: 6,
    });

    let ages: Vec<i32> = persons.iter().map(|x| x.age).collect();

    println!("The ages of persons are {:?}", ages);
}
