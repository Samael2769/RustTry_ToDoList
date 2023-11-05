/**
 * @ Author: Samael
 * @ Create Time: 2023-10-30 06:04:21
 * @ Modified by: Samael
 * @ Modified time: 2023-11-06 06:37:11
 * @ Description:
 */

 use std::io::{self, Write};

 mod tasks;
 
 fn main() {
     let mut task_list = tasks::TaskList::new();
     let res = task_list.load_tasks();
     if res.is_err() {
         println!("Failed to load tasks: {}", res.err().unwrap());
     }
 
     loop {
         print!("Enter a command (add/delete/get/complete/change/quit): ");
         io::stdout().flush().unwrap();
 
         let mut input = String::new();
         io::stdin().read_line(&mut input).unwrap();
 
         match input.trim() {
             "add" => {
                 print!("Enter task title: ");
                 io::stdout().flush().unwrap();
 
                 let mut title = String::new();
                 io::stdin().read_line(&mut title).unwrap();
 
                 print!("Enter task description: ");
                 io::stdout().flush().unwrap();
 
                 let mut description = String::new();
                 io::stdin().read_line(&mut description).unwrap();
                 let t = tasks::Task::new(title.trim().to_string(), description.trim().to_string());
                 task_list.add_task(t);
             }
             "delete" => {
                 print!("Enter task title: ");
                 io::stdout().flush().unwrap();
 
                 let mut input = String::new();
                 io::stdin().read_line(&mut input).unwrap();
 
                 let t = task_list.get_task_title(&input.trim().to_string());
                 let index = task_list.get_task(t);
                 task_list.remove_task(index);
             }
             "complete" => {
                 print!("Enter task title: ");
                 io::stdout().flush().unwrap();
 
                 let mut input = String::new();
                 io::stdin().read_line(&mut input).unwrap();
 
                 let t = task_list.get_task_title(&input.trim().to_string());
                 let index = task_list.get_task(t);
                 task_list.complete_task(index);
             }
             "change" => {
                 print!("Enter task title: ");
                 io::stdout().flush().unwrap();
 
                 let mut input = String::new();
                 io::stdin().read_line(&mut input).unwrap();
 
                 let t = task_list.get_task_title(&input.trim().to_string());
                 let index = task_list.get_task(t);
                 print!("Enter new task description: ");
                 io::stdout().flush().unwrap();
                 let mut description = String::new();
                 io::stdin().read_line(&mut description).unwrap();
                 task_list.edit_task(index, input.trim().to_string(), description.trim().to_string());
             }
             "get" => {
                    print!("Enter task title: ");
                    io::stdout().flush().unwrap();
    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
    
                    let t = task_list.get_task_title(&input.trim().to_string());
                    println!("Task: {:?}", t);
             }
             "quit" => {
                 task_list.save_tasks();
                 break;
             }
             _ => {
                 println!("Invalid command");
             }
         }
     }
 }
 