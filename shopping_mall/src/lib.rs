pub use crate::mall::floor::*;
pub mod mall;

pub fn biggest_store(mall: &mall::Mall) -> Option<&store::Store> {
    let mut biggest: Option<&store::Store> = None;
    let mut max_size = 0;

    for floor in &mall.floors {
        for store in &floor.stores {
            if store.square_meters > max_size {
                max_size = store.square_meters;
                biggest = Some(store);
            }
        }
    }

    biggest
}

pub fn highest_paid_employee(mall: &mall::Mall) -> Vec<&store::employee::Employee> {
    let mut highest_paid: Vec<&store::employee::Employee> = Vec::new();
    let mut top_salary = 0.0;

    for floor in &mall.floors {
        for store in &floor.stores {
            for employee in &store.employees {
                if highest_paid.is_empty() || employee.salary == top_salary {
                    highest_paid.push(employee);
                } else if employee.salary > top_salary {
                    highest_paid.clear();
                    highest_paid.push(employee);
                    top_salary = employee.salary;
                }
            }
        }
    }

    highest_paid
}

pub fn nbr_of_employees(mall: &mall::Mall) -> usize {
    let mut counter: usize = 0;

    for floor in &mall.floors {
        for store in &floor.stores {
            counter += store.employees.len();
        }
    }

    counter += mall.guards.len();

    counter
}

pub fn check_for_securities(mall: &mut mall::Mall, guards: Vec<mall::guard::Guard>) {
    let mut mall_area: u64 = 0;

    for floor in &mall.floors {
        for store in &floor.stores {
            mall_area += store.square_meters;
        }
    }

    let mut counter: u64 = 0;
    for guard in guards {
        if counter == 0 || mall_area / counter > 200 {
            mall.guards.push(guard);
            counter += 1;
        } else {
            break;
        }
    }
}

pub fn cut_or_raise(mall: &mut mall::Mall) {
    for floor in &mut mall.floors {
        for store in &mut floor.stores {
            for employee in &mut store.employees {
                let working_hours = employee.working_hours.1 - employee.working_hours.0;
                let salary_change = if working_hours > 10 {
                    employee.salary * 0.1
                } else {
                    -employee.salary * 0.1
                };
                employee.salary += salary_change;
            }
        }
    }
}
