use std::env;


pub fn get_password(args_password: String) -> Option<String> {
    if args_password.is_empty() {
        match  env::var("PAPM_PASSWORD") {
            Ok(a) => {Some(a)},
            _ => {None}
        }
    } else {
        Some(args_password)
    }
}