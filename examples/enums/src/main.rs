pub enum Status {
    Queued,
    Running,
    Failed,
}

pub fn print_status(status: Status) {
    match status {
        Status::Queued => println!("Queued!"),
        Status::Running => println!("Running!"),
        Status::Failed => println!("Failed!"),
    }
}


fn main() {
    let stat = Status::Queued;
    print_status(stat);
}
