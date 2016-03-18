fn main() {

    #[derive(Debug)]
    struct PasswordMaker {
        password: String,
    };

    impl PasswordMaker {
        fn new(password: &str) -> PasswordMaker {
            PasswordMaker {password: password.to_string()}
            //PasswordMaker {password: password}
        }

        fn incr_password(&self) -> String {
            self.password.clone() + "!!"
        }

        fn meets_requirements(&self) -> bool {
            true
        }

        fn next_password(&self) -> PasswordMaker {
            PasswordMaker {password: self.incr_password()}
        }

        fn get_new_password(&self) -> PasswordMaker {
            loop {
                let password_maker = self.next_password();
                if password_maker.meets_requirements() {
                    return password_maker;
                }
            }
        }
    }

    let password_maker = PasswordMaker::new("123");
    println!("{:?}", password_maker.get_new_password());
}
