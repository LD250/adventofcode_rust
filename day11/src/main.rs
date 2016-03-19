fn main() {
    const Z: u8 = 122;
    const A: u8 = 97;

    #[derive(Debug)]
    struct PasswordMaker {
        password: Vec<u8>,
    };

    impl PasswordMaker {
        fn new(password: &str) -> PasswordMaker {
            PasswordMaker {password: password.bytes().rev().collect()}
        }

        fn incr_password(&self) -> Vec<u8> {
            let mut new_password: Vec<u8> = vec!();
            let mut do_increment = true;
            for letter in &self.password {
                if (*letter == Z) & (do_increment) {
                    new_password.push(A);
                } else if do_increment {
                    new_password.push(*letter + 1);
                    do_increment = false;
                } else {
                    new_password.push(*letter);
                }
            }
            new_password
        }

        fn meets_requirements(&self) -> bool {
            for byte in "iol".bytes(){
                if self.password.contains(&byte) {
                    return false;
                }
            }
            let mut good_pass = false;
            for i in 0..(self.password.len() - 3) {
                if (self.password[i] == self.password[i+1] + 1) & (self.password[i] == self.password[i+2] + 2) {
                    good_pass = true;
                } 
            }
            if !good_pass {
                return false;
            }
            good_pass = false;
            let mut same_byte: u8 = 0;
            for i in 0..(self.password.len() - 2) {
                match self.password[i] == self.password[i+1] {
                    true if same_byte == 0 => same_byte = self.password[i],
                    true if same_byte != self.password[i] => {
                        //println!("{:?}", good_pass);
                        good_pass = true;
                        break;
                    },
                    _ => (),
                }
            };
            good_pass
        }

        fn next_password(&self) -> PasswordMaker {
            PasswordMaker {password: self.incr_password()}
        }

        fn get_password(&self) -> String {
            let correct_order: Vec<u8> = self.password.iter().rev().map(|n| *n).collect();
            String::from_utf8(correct_order).unwrap()
        }

        fn new_password_maker(&self) -> PasswordMaker {
            let mut password_maker = self.next_password();
            loop {
                if password_maker.meets_requirements() {
                    return password_maker;
                }
                password_maker = password_maker.next_password();
            }
        }
    }

    let password_maker = PasswordMaker::new("vzbxkghb");
    //println!("{:?}", "vzbxkghb".bytes().rev().collect());
    println!("{:?}", password_maker.new_password_maker().get_password());
    println!("{:?}", password_maker.new_password_maker().new_password_maker().get_password());
    //let password_maker = PasswordMaker::new("ayz");
    //println!("{:?}", password_maker.next_password().next_password().next_password().next_password().next_password());
}
