// Using a hash map and vectors, create a text interface to allow a user to add employee names
// to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by
// department, sorted alphabetically.

use std::io;
use std::collections::HashMap;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("1 - Add new Employee");
        println!("2 - Retrieve all people from department");
        println!("3 - List all people in the company by department sorted alphabetically");
        println!("4 - exit");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");
        command = command.trim().to_string();

        match command.as_str() {
            "1" => add_employee(&mut company),
            "2" => people_from_department(&mut company),
            "3" => all_employees_by_department(&mut company),
            "4" => break,
            _ => println!("Command do not exist"),
        }
    }
}

fn add_employee (company_hash: &mut HashMap<String, Vec<String>>) {
    println!("Inform the department: ");

    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Error reading department");
    department = department.trim().to_string();

    println!("Informe the employee name: ");

    let mut employee = String::new();
    io::stdin()
        .read_line(&mut employee)
        .expect("Error readin employee name!");
    employee = employee.trim().to_string();

    company_hash.entry(department).or_default().push(employee);

    println!("{:?}", company_hash);
}

fn people_from_department (company_hash: &mut HashMap<String, Vec<String>>) {
    println!("Inform the department: ");

    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Error reading department");
    department = department.trim().to_string();
    
    match company_hash.get(&department) {
        Some(results) => println!("{:?}", results),
        _ => println!("There is no {} registred", department),
    }
}

fn all_employees_by_department (company_hash: &mut HashMap<String, Vec<String>>) {
    for (key, value) in company_hash {
        println!("{}", key);

        let mut list_of_employee = value.clone();
        list_of_employee.sort();
        println!("{:?}", list_of_employee);
    }
}
