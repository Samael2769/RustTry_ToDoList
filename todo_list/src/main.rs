/**
 * @ Author: Samael
 * @ Create Time: 2023-10-30 06:04:21
 * @ Modified by: Samael
 * @ Modified time: 2023-11-02 05:54:18
 * @ Description:
 */

mod tasks;

fn main() {
    let t = tasks::Task::new(String::from("title"), String::from("description"));
    println!("{} {} {}", t.get_title(), t.get_description(), t.get_completed());
}
