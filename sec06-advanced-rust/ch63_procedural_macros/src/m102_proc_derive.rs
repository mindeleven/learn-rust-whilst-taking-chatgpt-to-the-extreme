#[cfg(test)]
#[allow(dead_code, unused_variables)]
mod test {
    extern crate hello_world_macro;
    use hello_world_macro::HelloWorld;

    use hello_world::HelloWorld;

    #[derive(HelloWorld)]
    struct User {
        username: String,
        email: String,
    }

    #[test]
    // cargo test tests_proc_derive -- --nocapture
    fn tests_proc_derive() {
        
        let user = User {
            username: "Alice".into(),
            email: "alice@alicesrestaurant.net".into(),
        };
    
        user.hello_world();

    }
}
