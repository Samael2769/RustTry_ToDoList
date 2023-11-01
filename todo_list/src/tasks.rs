/**
 * @ Author: Samael
 * @ Create Time: 2023-11-02 05:46:19
 * @ Modified by: Samael
 * @ Modified time: 2023-11-02 05:55:04
 * @ Description:
 */

pub struct Task {
    title: String,
    description: String,
    completed: bool,
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