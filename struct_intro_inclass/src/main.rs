struct Student { 
    name: String,
    major: String
}

impl Student {
    fn new(n: String, m: String) -> Self {
        Student {
            name: n,
            major: m
        }
    }

    fn get_major(&self) -> &String {
        return &self.major
    }

    fn set_major(&mut self, new_major: String) {
        self.major = new_major
    }
}


fn main() {
    let mut my_student = Student::new(("Dana").to_string(), ("Computer Science").to_string());

    println!("Name of student is: {}", my_student.name);
    println!("Major of student is: {}", my_student.get_major());
    my_student.set_major(("Accounting").to_string());
    println!("Student changed major to: {}", my_student.get_major());
}
