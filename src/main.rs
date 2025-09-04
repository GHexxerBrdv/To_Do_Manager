mod cli;
mod storage;
mod task;

use std::io::{self, Write};
use task::Task;

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    let mut id = 0;
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");

        if !cli::handle_command(&cmd, &mut tasks, &mut id) {
            break;
        }
    }
}
