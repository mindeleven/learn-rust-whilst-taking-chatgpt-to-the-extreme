#![allow(dead_code, unused_imports)]

// structure of a function that should have the ability to receive either 
// an address type reference or a string reference as an address
// therefore the generic type
fn get_etherum_address<T>(_address: T) {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_polymorphism -- --nocapture
    #[test] 
    fn tests_polymorphism() {

        dbg!("tests_polymorphism");
    
    }

}
