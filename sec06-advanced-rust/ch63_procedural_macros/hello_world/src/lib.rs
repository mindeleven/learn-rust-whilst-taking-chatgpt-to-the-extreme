pub trait HelloWorld {
    fn hello_world(&self) {
        println!("Printing from the trait itself");
    }
}
