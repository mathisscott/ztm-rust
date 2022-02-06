// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<u32>
}


fn main() {
    let students: Vec<Student> = vec![
        Student { name: "Becky".to_owned(), locker: Some(45) },
        Student { name: "Jenn".to_owned(), locker: None },
        Student { name: "Stacy".to_owned(), locker: Some(3) },
    ];

    for s in students {
        match s {
            Student { name, locker: None } => println!("{} has no locker", name),
            Student { name, locker: Some(num) } => println!("{} has locker no. {:?}", name, num),
        }
    }
}
