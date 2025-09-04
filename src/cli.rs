use crate::task::Task;
use crate::storage;

pub fn handle_command(cmd: &str, tasks: &mut Vec<Task>, id: &mut u32) -> bool {
    let parts: Vec<&str> = cmd.trim().splitn(2, ' ').collect();
    
    match parts[0] {
        "add" => {
            if parts.len() > 1 {
                storage::add_task(tasks, parts[1], id);
            } else {
                println!("Usage: add <task>");
            }
        }
        "list" => {
            storage::list_tasks(tasks);
        }
        "done" => {
            if parts.len() > 1 {
                if let Ok(id) = parts[1].parse::<u32>() {
                    if let Err(e) = storage::mark_done(tasks, id) {
                        println!("Error marking task as done: {}", e);
                    }
                } else {
                    println!("Invalid ID");
                }
            } else {
                println!("Usage: done <id>");
            }
        }
        "remove" => {
            if parts.len() > 1 {
                if let Ok(id) = parts[1].parse::<u32>() {
                    if let Err(e) = storage::remove_task(tasks, id) {
                        println!("Error removing task: {}", e);
                    }
                } else {
                    println!("Invalid ID");
                }
            } else {
                println!("Usage: remove <id>");
            }
        }
        "exit" => {
            return false;
        }
        "" => {}
        _ => println!("Invalid command {}", parts[0]),
    }
    
    return true;
}