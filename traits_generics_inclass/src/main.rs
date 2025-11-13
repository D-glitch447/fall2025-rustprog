trait ShowInfo {
    fn show_info(&self);
}


//Undergraduate Student
struct Undergrad {
    name: String,
    major: String,
    gpa: f32, 
}

//Graduate student
struct Grad {
    name: String,
    major: String,
    gpa: f32,
    thesis: String,
}


impl ShowInfo for Undergrad {
    fn show_info(&self) {
        println!("Undergrad Student");
        println!("Name: {}", self.name);
        println!("Major: {}", self.major); 
        println!("GPA: {}",self.gpa);
        println!();
    }
}

impl ShowInfo for Grad {
    fn show_info(&self) {
        println!("Undergrad Student");
        println!("Name: {}", self.name);
        println!("Major: {}", self.major); 
        println!("GPA: {}",self.gpa);
        println!("Thesis: {}", self.thesis);
        println!();
    }
}

struct Enrollment <T: ShowInfo> {
    students: Vec<Box<T>>,
}

impl <T:ShowInfo> Enrollment<T> {
    fn new() -> Self {
        Self { students: vec![] }
    }

    fn add_student(&mut self, student: T) {
        self.students.push(Box::new(student));
    }

    fn show_all(&self) {
        self.students.iter().for_each(|student| student.show_info());
    }
}

fn main() {
    let mut undergrad_enroll = Enrollment::<Undergrad>::new();
    undergrad_enroll.add_student(Undergrad {
        name: "Dana".into(),
        major: "Computer Science".into(),
        gpa: 3.7,
    });
    undergrad_enroll.add_student(Undergrad {
        name: "Jesus".into(),
        major: "Engineering".into(),
        gpa: 3.4,
    });

    let mut grad_enroll = Enrollment::<Grad>::new();
    grad_enroll.add_student(Grad {
        name: "Jacob".into(),
        major: "Data Science".into(),
        gpa: 3.9,
        thesis: "Neutral Network Optimization".into(),
    });

    println!("=== Undergraduate Enrollment ===");
    undergrad_enroll.show_all();

    println!(" === Graduate Enrollment === ");
    grad_enroll.show_all();
}