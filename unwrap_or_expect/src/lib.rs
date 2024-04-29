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
        Security::Unknown => {
            server.expect("")
        },
        Security::High => {
            &server.unwrap_or_else(|_| panic!("ERROR: program stops"))
        },
        Security::Medium => {
            &server.unwrap_or("WARNING: check the server".to_string())
        },
        Security::Low => {
            &server.unwrap_or_else(|e| format!("Not found: {}", e))
        },
        Security::BlockServer => {
            if let Ok(message) = &server {
                panic!("{}", message);
            } else {
                server.unwrap_err()
            }
        },
    }
}