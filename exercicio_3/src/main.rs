// Neste código, vamos criar uma interface de texto para permitir que o usuário adicione nomes de funcionários a um departamento da empresa. E ele possa recuperar os nomes dos funcionários de um departamento específico ou de todos os departamentos, classificados por ordem alfabética.

fn main() {
    let mut company = Company::new();
    loop {
        println!("1. Add employee");
        println!("2. List employees by department");
        println!("3. List all employees");
        println!("4. Exit");
        let option = read_option();
        match option {
            1 => {
                let name = read_string("Enter the employee name: ");
                let department = read_string("Enter the department name: ");
                company.add_employee(name, department);
            }
            2 => {
                let department = read_string("Enter the department name: ");
                company.list_employees_by_department(department);
            }
            3 => company.list_all_employees(),
            4 => break,
            _ => println!("Invalid option"),
        }
    }
}

fn read_option() -> i32 {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number"),
        }
    }
}

fn read_string(message: &str) -> String {
    loop {
        println!("{}", message);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if !input.is_empty() {
            return input.to_string();
        }
    }
}

struct Company {
    employees: std::collections::HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Company {
        Company {
            employees: std::collections::HashMap::new(),
        }
    }

    fn add_employee(&mut self, name: String, department: String) {
        let employees = self.employees.entry(department).or_insert(vec![]);
        employees.push(name);
    }

    fn list_employees_by_department(&self, department: String) {
        if let Some(employees) = self.employees.get(&department) {
            let mut employees = employees.clone();
            employees.sort();
            for employee in employees {
                println!("{}", employee);
            }
        } else {
            println!("Department not found");
        }
    }

    fn list_all_employees(&self) {
        let mut employees: Vec<_> = self.employees.iter().collect();
        employees.sort_by(|a, b| a.0.cmp(b.0));
        for (department, employees) in employees {
            println!("Department: {}", department);
            let mut employees = employees.clone();
            employees.sort();
            for employee in employees {
                println!("  {}", employee);
            }
        }
    }
}
    