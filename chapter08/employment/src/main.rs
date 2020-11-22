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
        println!("Please input new employment.");
        let mut order = String::new();
        io::stdin()
            .read_line(&mut order)
            .expect("Failed to read line");
        let person = parse_input(order);
        person.enter_member(&mut members);
        println!("{:?}", members)
    }
}

fn parse_input(order: String) -> Employee {
    let order_map: Vec<&str> = order.split(" ").collect();
    let name = order_map[1].to_string();
    let department = order_map[3].to_string();
    Employee { name, department }
}
