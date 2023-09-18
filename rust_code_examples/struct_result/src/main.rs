use std::collections::HashMap;

struct Student {
    id: i32,
    name: String,
    grade: String,
}

struct StudentManager {
    student: HashMap<i32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        let student = HashMap::from([(
            1,
            Student {
                id: 1,
                name: String::from("Default Name"),
                grade: 'A'.to_string(),
            },
        )]);
        StudentManager { student }
    }

    fn add_student(&mut self, student: Student) -> Result<(), String> {
        let id = student.id;

        if self.student.contains_key(&id) {
            Err("Student exists".to_string())
        } else {
            self.student.entry(id).or_insert(student);
            Ok(())
        }
    }

    fn get_student(&self, id: i32) -> Option<&Student> {
        self.student.get(&id)
    }
}
fn main() {
    let mut student = StudentManager::new();
    match student.add_student(Student {
        id: 2,
        name: "Chen Liang".to_string(),
        grade: 'A'.to_string(),
    }) {
        Ok(_) => println!("Add success"),
        Err(e) => println!("{}", e),
    }

    match student.get_student(3) {
        Some(student) => println!("{} found", student.name),
        None => println!("Student not found"),
    }
}
