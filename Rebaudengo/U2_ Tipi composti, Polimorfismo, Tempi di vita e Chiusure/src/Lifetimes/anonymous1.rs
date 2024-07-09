struct Worker<'a> {
    name: &'a str,
    id: u32,
}
impl Worker<'_> {
    fn new(name: &str, id: u32) -> Worker{
        Worker { name, id }
    }

    fn get_name(&self) -> &str {
        self.name
    }
}
fn main() {
    let name = "Alice";
    let id = 1001;

    let worker = Worker::new(name, id);

    println!("Worker name: {}", worker.get_name());
}
