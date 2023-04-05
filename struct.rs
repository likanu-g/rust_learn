struct Student {
    name: &'static str,
    score: i32,
}

fn main() {
    let score = 59;
    let username = "zhnagsan";

    let mut student = Student{
        score,
        name: username,
    };
    student.score = 60;
    println!("name: {}, score: {}", student.name, student.score);
}