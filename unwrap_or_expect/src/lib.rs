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
        Security::Unknown => server.unwrap(),
        Security::High => {
            server.clone().unwrap_or_else(|_| panic!("ERROR: program stops"))
        },
        Security::Medium => {
            match server {
                Ok(url) => url,
                Err(_) => String::from("WARNING: check the server"),
            }
        },
        Security::Low => {
            match server {
                Ok(url) => url,
                Err(err) => format!("Not found: {}", err),
            }
        },
        Security::BlockServer => {
            match server {
                Ok(url) => panic!("{}", url),
                Err(err) => err,
            }
        },
    }
}
