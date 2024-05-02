pub mod mall;
pub use mall::floor::*;
pub use mall::floor::store::*;
pub use mall::*;

pub fn biggest_store(m: Mall) -> store::Store{
    let mut my_big = 0;
    let mut my_big_store = store::Store::new("",0, vec![employee::Employee{
        name: "".to_string(),
        age: 0,
        working_hours: (0, 0),
        salary: 0.0,
    }]);
    for fl in m.floors {
        for st in fl.stores {
            if st.square_meters > my_big {
                my_big = st.square_meters;
                my_big_store = st;
            }
        }
    }
    my_big_store
}

pub fn highest_paid_employee(m: Mall) -> Vec<employee::Employee>{
    let mut my_big = 0.0;
    let mut my_vec:Vec<employee::Employee> = Vec::new();
    for fl in &m.floors {
        for st in &fl.stores {
            for emply in &st.employees{
                if emply.salary >= my_big {
                    my_big = emply.salary;
                }
            }
        }
    }
    for fl in m.floors {
        for st in fl.stores {
            for emply in st.employees{
                if emply.salary >= my_big {
                    my_vec.push(emply);
                }
            }
        }
    }
    my_vec
}

pub fn nbr_of_employees(m: Mall) -> usize {
    let mut nbr_empl = 0;
    for fl in m.floors {
        for st in fl.stores {
            for _emply in st.employees{
                nbr_empl += 1;
            }
        }
    }
    
   nbr_empl as usize + m.guards.len()
}

pub fn check_for_securities(m: &mut Mall, v: Vec<guard::Guard>){
    let mut cnt_gd_fl = 0;
    for fl in &m.floors {
        cnt_gd_fl += fl.size_limit / 200;
    }

    if cnt_gd_fl as usize > m.guards.len() {
        let d:usize = m.guards.len().abs_diff(cnt_gd_fl as usize);
        for i in 0..=d{
            m.hire_guard(v[i].clone());
            // if i < v.len() as u64{
            // }
        }
    }
}

pub fn cut_or_raise(m: &mut Mall) {
    for fl in &mut m.floors {
        for st in &mut fl.stores {
            for emply in &mut st.employees {
                if (emply.working_hours.1 - emply.working_hours.0) > 10 as u8 {
                    emply.salary += (emply.salary * 0.1 as f64);
                } else {
                    emply.salary -= (emply.salary * 0.1 as f64);
                }
            }
        }
    }
}