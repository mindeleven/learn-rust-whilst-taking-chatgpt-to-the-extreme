#![allow(dead_code, unused_imports, unused_variables)]

#[cfg(test)]
mod test {
    use std::borrow::BorrowMut;
    use std::rc::{
        Rc, 
        Weak
    };
    use std::cell::RefCell;

    // cargo test tests_smart_pointers_weak -- --nocapture
    #[test] 
    fn tests_smart_pointers_weak() {
        
        // smart pointers, reference counting with Weak
        // using a downgraded weak pointer instead of a smart pointer
        #[derive(Debug)]
        struct House {
            address_number: u16,
            street: String,
            furniture: RefCell<Vec<Rc<Furniture>>> // reference counting furniture
        }

        #[derive(Debug)]
        struct Furniture {
            id: String,
            description: String,
            // house: Rc<House>, // creating a Weak reference, going only one way
            // Rc<> creates an circular reference and therefore an infinite loop
            // using Weak instead
            house: Weak<House>,
        }

        let house1 = Rc::new(House { // mutable because its a Rc
            address_number: 123,
            street: "coding avenue".to_string(),
            furniture: RefCell::new(vec![])
        });

        let table = Rc::new(Furniture {
            id: "table1".to_string(),
            description: "kitchen table".to_string(),
            house: Rc::downgrade(&house1), // Weak reference -> downgrade instead of clone
        });

        let desk = Rc::new(Furniture {
            id: "desk1".to_string(),
            description: "office desk".to_string(),
            house: Rc::downgrade(&house1), // Weak reference -> downgrade instead of clone
        });

        house1.furniture.borrow_mut().push(Rc::clone(&table));
        house1.furniture.borrow_mut().push(Rc::clone(&desk));

        dbg!(house1);
    }
}
        
