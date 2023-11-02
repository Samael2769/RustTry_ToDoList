/**
 * @ Author: Samael
 * @ Create Time: 2023-11-02 05:46:19
 * @ Modified by: Samael
 * @ Modified time: 2023-11-03 05:57:09
 * @ Description:
 */
 
 #[derive(Debug)]
 #[derive(PartialEq)]
 pub struct Task {
     title: String,
     description: String,
     completed: bool,
    }
    
pub struct TaskList {
    tasks: Vec<Task>,
}

impl Task {
    pub fn new(title: String, description: String) -> Task {
        Task {
            title,
            description,
            completed: false,
        }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_completed(&self) -> bool {
        self.completed
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }
}


impl TaskList {

    pub fn new() -> TaskList {
        TaskList {
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, index: usize) {
        self.tasks.remove(index);
    }

    pub fn get_nb_tasks(&self) -> usize {
        self.tasks.len()
    }

    pub fn edit_task(&mut self, index: usize, title: String, description: String) {
        self.tasks[index].set_title(title);
        self.tasks[index].set_description(description);
    }

    pub fn get_task_index(&self, index: usize) -> &Task {
        &self.tasks[index]
    }

    pub fn get_task_title(&self, title: &str) -> &Task {
        for task in &self.tasks {
            if task.get_title() == &title {
                return task;
            }
        }
        panic!("No task with title: {}", title);
    }

    pub fn get_task(&self, task: &Task) -> usize {
        for (index, t) in self.tasks.iter().enumerate() {
            if t == task {
                return index;
            }
        }
        panic!("No task with title: {}", task.get_title());
    }

    pub fn complete_task(&mut self, index: usize) {
        self.tasks[index].set_completed(true);
    }

    pub fn uncomplete_task(&mut self, index: usize) {
        self.tasks[index].set_completed(false);
    }
}