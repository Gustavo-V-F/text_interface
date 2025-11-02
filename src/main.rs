use std::collections::HashMap;
use std::fmt;
use std::io;

#[derive(Debug, Eq, Hash, PartialEq, Clone, PartialOrd, Ord)]
struct Employee(String);

impl fmt::Display for Employee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, PartialOrd, Ord)]
struct Department(String);

impl fmt::Display for Department {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Active(bool);

#[derive(Debug)]
struct Company {
    employees: HashMap<Employee, Option<Vec<Department>>>,
    departments: HashMap<Department, Option<HashMap<Employee, Active>>>,
}

impl Company {
    fn add(&mut self, employee: &Employee, department: &Department, active: Active) -> bool {
        let employee = employee.clone();
        let department = department.clone();
        match self.employees.get(&employee) {
            Some(dept_vec) if *dept_vec != None => {
                let mut dept_vec = dept_vec.clone().unwrap();
                if dept_vec.contains(&department) {
                    return false;
                } else {
                    let emply = employee.clone();
                    let dept = department.clone();
                    dept_vec.push(dept);
                    let dept_vec = Some(dept_vec);
                    self.employees.insert(emply, dept_vec);
                }
            }
            _ => {
                let emply = employee.clone();
                let dept = department.clone();
                let mut dept_vec = Vec::new();
                dept_vec.push(dept);
                let dept_vec = Some(dept_vec);
                self.employees.insert(emply, dept_vec);
            }
        };

        match self.departments.get(&department) {
            Some(emply_map) if *emply_map != None => {
                let mut emply_map = emply_map.clone().unwrap();
                if emply_map.contains_key(&employee) {
                    return false;
                } else {
                    emply_map.insert(employee, active);
                    let emply_map = Some(emply_map);
                    self.departments.insert(department, emply_map);
                    return true;
                }
            }
            _ => {
                let mut emply_map = HashMap::new();
                emply_map.insert(employee, active);
                let emply_map = Some(emply_map);
                self.departments.insert(department, emply_map);
                return true;
            }
        };
    }

    fn add_employee(&mut self, employee: &Employee) -> bool {
        let employee = employee.clone();
        match self.employees.get(&employee) {
            Some(_) => return false,
            None => {
                self.employees.insert(employee, None);
                return true;
            }
        };
    }

    fn add_department(&mut self, department: &Department) -> bool {
        let department = department.clone();
        match self.departments.get(&department) {
            Some(_) => return false,
            None => {
                self.departments.insert(department, None);
                return true;
            }
        };
    }

    fn list_employees(&self) {
        println!("Employee in Departments:");
        let mut emply_vec: Vec<(Employee, Option<Vec<Department>>)> =
            self.employees.clone().into_iter().collect();
        emply_vec.sort_by_key(|k| k.0.clone());
        for emply in emply_vec {
            let (employee, departments) = emply;
            let mut dpts = String::new();
            match departments {
                Some(dpts_vec) => {
                    for (size, dpt) in dpts_vec.iter().enumerate() {
                        let dpt = format!("{}", dpt);
                        dpts.push_str(&dpt);
                        if size < dpts_vec.len() - 1 {
                            dpts.push_str(", ");
                        }
                    }
                }
                _ => dpts.push_str(&String::from("None")),
            };

            println!("{employee} in {dpts}.");
        }
    }

    fn list_employees_by_department(&self) {
    	
    }
}

fn main() {
    let mut office = Company {
        employees: HashMap::new(),
        departments: HashMap::new(),
    };

    loop {
        let mut text = String::new();

        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line.");

        let mut words = text.split_whitespace();

        let word = words.next();

        match word {
            Some(command) if command.to_lowercase() == "add" || command.to_lowercase() == "a" => {
                let mut employee = String::new();
                let mut department = String::new();
                let mut to = false;
                for name in words {
                    match name {
                        partial_name if to == true => {
                            department.push_str(partial_name);
                            department.push_str(" ");
                            continue;
                        }
                        partial_name if partial_name.to_lowercase() != "to" && to == false => {
                            employee.push_str(partial_name);
                            employee.push_str(" ");
                            continue;
                        }
                        partial_name if partial_name.to_lowercase() == "to" && to == false => {
                            to = true;
                            continue;
                        }
                        _ => break,
                    };
                }

                if !employee.is_empty() {
                    let employee = Employee(String::from(employee.trim_end()));
                    if department.is_empty() {
                        office.add_employee(&employee);
                        println!("Added {employee} to the company.");
                    } else {
                        let department = Department(String::from(department.trim_end()));
                        office.add(&employee, &department, Active(true));
                        println!("Added {employee} to the company's {department} department.");
                    }
                } else {
                    println!("No arguments found.");
                }
            }
            Some(command)
                if command.to_lowercase() == "create" || command.to_lowercase() == "c" =>
            {
                let mut department = String::new();
                for name in words {
                    department.push_str(name);
                    department.push_str(" ");
                }

                if !department.is_empty() {
                    let department = Department(String::from(department.trim_end()));
                    office.add_department(&department);
                    println!("Added {department} department to the company.");
                } else {
                    println!("No arguments found.");
                }
            }
            Some(command) if command.to_lowercase() == "list" || command.to_lowercase() == "l" => {
                let word = words.next();
                match word {
                    Some(subcommand) if subcommand.to_lowercase() == "employees" => {
                        office.list_employees()
                    }
                    _ => continue,
                }
            }
            Some(command) if command.to_lowercase() == "exit" || command.to_lowercase() == "e" => {
                break;
            }
            _ => {
                println!("No command found.");
                continue;
            }
        };
    }
}
