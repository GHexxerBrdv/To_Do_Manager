use crate::task::Task;

pub fn add_task(tasks: &mut Vec<Task>, title: &str, id: &mut u32) {
    let new_task = Task::new(*id, title.to_string());
    *id += 1;
    tasks.push(new_task);
    println!("new task added");
}

pub fn remove_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), String> {
    if let Some(index) = tasks.iter().position(|task| task.id == id) {
        tasks.remove(index);
        println!("Task {} removed", id);
        Ok(())
    } else {
        Err(format!("Task {} not found", id))
    }
}

pub fn mark_done(tasks: &mut Vec<Task>, id: u32) -> Result<(), String> {
    if let Some(index) = tasks.iter().position(|task| task.id == id) {
        tasks[index].completed = true;
        println!("Task {} completed", id);
        remove_task(tasks, id)
    } else {
        Err(format!("Task {} not found", id))
    }
}

pub fn list_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        task.display();
    }
}
