#[cfg(test)]
#[allow(dead_code, unused_variables)]
mod test {
    extern crate hello_world_macro;
    use hello_world_macro::HelloWorld;

    // #[derive(HelloWorld)] -> ERROR: expected trait, found derive macro `HelloWorld`
    struct User {
        username: String,
        email: String,
    }

    #[test]
    // cargo test tests_proc_macro_ex01 -- --nocapture
    fn tests_proc_macro_ex01() {
        
        let user = User {
            username: "Alice".into(),
            email: "alice@alicesrestaurant.net".into(),
        };
    
        // user.hello_world();

    }
}
