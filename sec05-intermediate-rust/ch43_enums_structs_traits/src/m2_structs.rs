#![allow(dead_code, unused_imports)]

// creating a type called User with a struct
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn increment_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_structs -- --nocapture
    #[test] 
    fn tests_structs() {

        let mut user_1: User = User {
            username: String::from("johndoe"),
            email: String::from("john@doe.io"),
            sign_in_count: 1,
            active: true,
        };
        
        user_1.change_email("john_x@doe.io");
        
        let mut user_2: User = User {
            username: String::from("jane"),
            email: String::from("jane@doe.io"),
            sign_in_count: 0,
            active: false,
        };

        user_2.increment_sign_in_count();
        user_2.change_email("jane_x@doe.io");

        dbg!(user_1);
        dbg!(user_2);

    }

}




