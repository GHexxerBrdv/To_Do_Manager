#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, title: String) -> Task {
        Task {
            id,
            title,
            completed: false,
        }
    }

    pub fn display(&self) {
        println!(
            "id: {}, title: {}, completed: {}",
            self.id, self.title, self.completed
        )
    }
}
