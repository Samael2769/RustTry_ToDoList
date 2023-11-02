/**
 * @ Author: Samael
 * @ Create Time: 2023-10-30 06:04:21
 * @ Modified by: Samael
 * @ Modified time: 2023-11-03 05:58:56
 * @ Description:
 */

mod tasks;

fn main() {
    let mut task = tasks::TaskList::new();
    task.add_task(tasks::Task::new(String::from("title1"), String::from("description")));
    task.add_task(tasks::Task::new(String::from("title"), String::from("description")));
    let mut t = task.get_task_title("title");
    println!("{:?}", t);
    let i = task.get_task(t);
    println!("{:?}", i);
    task.complete_task(i);
    println!("{:?}", task.get_task_index(i));
    task.remove_task(i);
    println!("{:?}", task.get_nb_tasks());
}
