#[derive(Debug)]
struct Job {
    title: String,
    salary: u32,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    job: Job, // Composition: Person "has a" Job
}

impl Person {
    fn new(name: &str, age: u8, title: &str, salary: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
            job: Job {
                title: title.to_string(),
                salary,
            },
        }
    }

    fn introduce(&self) {
        println!(
            "Hi, I'm {} ({} years old). I work as a {} earning ${}/year.",
            self.name, self.age, self.job.title, self.job.salary
        );
    }
}

fn main() {
    let alice = Person::new("Alice", 30, "Software Engineer", 120_000);
    alice.introduce();

    // Debug print for quick inspection
    println!("{:?}", alice);
}

