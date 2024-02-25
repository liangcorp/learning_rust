use std::collections::HashMap;

struct Student {
    name: String,
    age: i32,
    grade: String,
}

struct StudentManager {
    student: HashMap<i32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        let mut student = HashMap::new();
        student.entry(1).or_insert(
            Student {
                name: String::from("Default Name"),
                age: 20,
                grade: 'A'.to_string(),
            });
        StudentManager { student }
    }

    fn add_student(&mut self, student: Student) -> Result<(), String> {
    }
}

fn add_student(
    student_database: &mut HashMap<i32, Student>,
    id: i32,
    name: String,
    age: i32,
    grade: String,
) {
    let student = Student { name, age, grade };
    student_database.entry(id).or_insert(student);
}

struct Item {
    id: i32,
    title: String,
    year: i32,
    type_enum: ItemType,
}

enum ItemType {
    Book,
    Magazine,
}

fn display_item_info(item: Item) {
    print!(
        "ID: {}, Title: {}, publication year: {} ",
        item.id, item.title, item.year
    );
    match item.type_enum {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Magazine"),
    }
}

fn main() {
    // Question 2
    let mut student_database: HashMap<i32, Student> = HashMap::new();

    add_student(
        &mut student_database,
        1,
        String::from("John"),
        17,
        String::from("Grade 11"),
    );

    add_student(
        &mut student_database,
        2,
        String::from("Sarah"),
        16,
        String::from("Grade 10"),
    );

    // Printing the student database

    for (id, student) in &student_database {
        println!("Student ID: {}", id);

        println!("Name: {}", student.name);

        println!("Age: {}", student.age);

        println!("Grade: {}", student.grade);

        println!("------------------");
    }

    // Question 3
    let item = Item {
        id: 1,
        title: "new world".to_string(),
        year: 1984,
        type_enum: ItemType::Book,
    };

    display_item_info(item);
}
