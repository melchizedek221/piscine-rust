// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }


pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.expect(""),
        Security::High => {
            if let Err(_) = &server {
                panic!("ERROR: program stops");
            }
            server.expect("")
        },
        Security::Medium => {
            if let Err(_) = &server {
                return String::from("WARNING: check the server");
            }
            server.unwrap()
        },
        Security::Low => {
            server.unwrap_or_else(|err| format!("Not found: {}", err))
        },
        Security::BlockServer => {
            match server {
                Ok(url) => panic!("{}", url),
                Err(err) => err,
            }
        }
    }
}


