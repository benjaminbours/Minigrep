use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum UserOption {
    Add,
    List,
    Leave,
}

fn main() {
    let mut company_departments = HashMap::new();
    loop {
        let option = display_options();

        println!("option: {:#?}", option);

        match option {
            UserOption::Add => add_name_to_department(&mut company_departments),
            UserOption::List => add_name_to_department(&mut company_departments),
            UserOption::Leave => break,
        }
    }
}

fn display_options() -> UserOption {
    loop {
        println!("What do you want to do:");
        println!("1. Add employee names to a department of the company");
        println!("2. List all people in the company by department");
        println!("3. Leave");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        match option.trim().parse() {
            Ok(num) => match num {
                1 => break (UserOption::Add),
                2 => break (UserOption::List),
                3 => break (UserOption::Leave),
                _ => {
                    println!("Enter 1, 2 or 3");
                    continue;
                }
            },
            Err(_) => {
                println!("Enter 1, 2 or 3");
                continue;
            }
        };
    }
}

fn add_name_to_department(company_departments: &mut HashMap<String, Vec<String>>) {
    println!("Use this shape: Add {{person}} to {{department}}'");
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let words: Vec<&str> = user_input.trim().split(" ").collect();
    println!("{:?}", words);

    let name = words[1];
    let department = words[3];

    let company_department = company_departments
        .entry(String::from(department))
        .or_insert(vec![]);

    company_department.push(String::from(name));

    println!("name: {}, department: {}", name, department);

    println!("{} as been added to {}", name, department);

    println!("company {:#?}", company_departments);
}
