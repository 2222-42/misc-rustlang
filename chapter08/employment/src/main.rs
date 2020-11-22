use std::io;

#[derive(Debug)]
struct Employee {
    name: String,
    department: String,
}

#[derive(Debug)]
struct Members(Vec<Employee>);

// impl Members {
//     pub fn new() -> Members {
//         Members(Vec::new())
//     }
// }

impl Employee {
    pub fn enter_member(self, members: &mut Vec<Employee>) {
        members.push(self)
    }
}

// impl Members {
//     fn add_member(self, member: Employee) {
//         self.push(member)
//     }
// }

fn main() {
    let mut members: Vec<Employee> = Vec::new();
    loop {
        println!("Please input.");
        let mut order = String::new();
        io::stdin()
            .read_line(&mut order)
            .expect("Failed to read line");
        let order: String = match order.trim().parse() {
            Ok(string) => string,
            Err(_) => continue,
        };
        parse_input(order, &mut members);
    }
}

fn parse_input(order: String, members: &mut Vec<Employee>) {
    let order_map: Vec<&str> = order.split(" ").collect();
    let command = order_map[0];
    match command {
        "Add" => {
            let name = order_map[1].to_string();
            let department = order_map[3].to_string();
            add_member(name, department, members);
        }
        _ => (),
    }
}

fn add_member(name: String, department: String, members: &mut Vec<Employee>) {
    let person = Employee { name, department };
    person.enter_member(members);
    println!("{:?}", members)
}
