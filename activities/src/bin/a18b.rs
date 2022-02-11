// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Role {
    Maintenance,
    Management,
    Marketing,
    LineSupervisor,
    Kitchen,
    Technicians,
}

enum Status {
    Active,
    Terminated,
}

struct Employee {
    role: Role,
    status: Status
}

fn allow_access(emp: &Employee) -> Result<(), String> {
    match emp.status {
        Status::Terminated => return Err("employeed has been terminated".to_owned()), // EARLY RETURN!!!
        _ => (),
    }

    match emp.role {
        Role::Maintenance => Ok(()),
        Role::Management => Ok(()),
        Role::Marketing => Ok(()),
        _ => Err("employee does not have access".to_owned()),
    }
}

fn print_access(emp: &Employee) -> Result<(), String> {
    let _access_attempt = allow_access(emp)?;
    println!("employee allowed in!");
    Ok(())
}

fn main() {
    let emp = Employee { role: Role::Management, status: Status::Terminated };

    match print_access(&emp) {
        Err(e) => println!("access denied: {:?}", e),
        _ => ()
    }
}
