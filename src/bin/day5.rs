fn main() {
    const PREFIX: &str = "uqwqemis";

    println!("Part 1: password: {}", find_password(PREFIX));
    println!("Part 2: password: {}", find_password_2(PREFIX));
}

fn find_password(prefix: &str) -> String {
    const PASSWORD_LENGTH: usize = 8;
    let mut password: String = "".to_string();
    let mut to_be_hashed: String = "".to_string();
    to_be_hashed.push_str(prefix);

    let mut num = 0;
    while password.len() < PASSWORD_LENGTH {
        to_be_hashed.truncate(prefix.len());
        to_be_hashed.push_str(num.to_string().as_str());
        let digest = md5::compute(&to_be_hashed);
        if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0 {
            password.push(format!("{:x}", digest[2] & 0x0f).parse().unwrap());
        }

        num += 1;
    }
    password
}

fn find_password_2(prefix: &str) -> String {
    const PASSWORD_LENGTH: usize = 8;
    let mut password: [Option<u8>; 8] = [None; 8];
    let mut found = 0;
    let mut to_be_hashed: String = "".to_string();
    to_be_hashed.push_str(prefix);

    let mut num = 0;
    while found < PASSWORD_LENGTH {
        to_be_hashed.truncate(prefix.len());
        to_be_hashed.push_str(num.to_string().as_str());
        let digest = md5::compute(&to_be_hashed);
        if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0 {
            let idx = (digest[2] & 0x0f) as usize;
            if idx < PASSWORD_LENGTH && password[idx].is_none() {
                password[idx] = Some((digest[3] & 0xf0)>>4);
                found += 1;
            }
        }

        num += 1;
    }
    password.iter().map(|c| format!("{:x}", (c.unwrap()))).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_password() {
        assert_eq!(find_password("abc"), "18f47a30");
    }

    #[test]
    fn test_password2() {
        assert_eq!(find_password_2("abc"), "05ace8e3");
    }
}
