use std::io;
use std::collections::HashMap;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    println!("Hello! Welcome to HR systems.");

    loop {
        println!("Do you seek to add, remove, list, or exit?");

        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        let mut department = String::new();
        let mut employee = String::new();

        let action = action.trim().to_lowercase();

        match action.as_str() {
            "add" => {
                println!("Which department would you like to add to?");
                // Add logic for adding to a department
                io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read line");

                department = department.trim().to_string();

                println!("Which employee would you like to add to the {} department?", &department);

                io::stdin()
                    .read_line(&mut employee)
                    .expect("Failed to read line");

                    employee = employee.trim().to_string(); // Trim any trailing newline or whitespace

                    if departments.contains_key(&department) {
                        if let Some(employees) = departments.get_mut(&department) {
                            employees.push(employee);
                        }
                    } else {
                        // If the department doesn't exist, create it and add the employee
                        departments.insert(department.clone(), vec![employee.clone()]);
                    }

                    println!("Employee has been added to the {} department", department);
            }
            "remove" => {
                println!("Which department would you like to remove from?");
                // Add logic for removing from a department
                io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read line");

                if !departments.contains_key(&department) {
                    println!("Department doesn't exist. Returning to main.");
                    continue;
                }

                println!("Which employee would you like to remove from the {} department?", &department);

                io::stdin()
                    .read_line(&mut employee)
                    .expect("Failed to read line");

                employee = employee.trim().to_string(); // Trim any trailing newline or whitespace

                if let Some(employees) = departments.get_mut(&department) {
                   if let Some(index) = employees.iter().position(|x| *x == employee) {
                        employees.remove(index);
                        println!("Employee has been removed.");
                    } else {
                        println!("Employee not found");

                    }
                }
            }
            "list" => {
                println!("Listing all departments and their employees...");
                // Add logic for listing departments
                for (key, values) in &departments {
                    println!("{} department", key);
                    for value in values {
                        println!("{}", *value);
                    }
                    println!("\n");
                }
            }
            "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown action. Please type 'add', 'remove', 'list', or 'exit'."),
        }
    }
}