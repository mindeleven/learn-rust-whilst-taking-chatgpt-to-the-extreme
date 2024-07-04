#![allow(dead_code, unused_imports, unused_variables)]

#[cfg(test)]
mod test {
    use std::rc::{
        Rc, 
        Weak
    };
    use std::cell::RefCell;

    // cargo test tests_smart_pointers_rc -- --nocapture
    #[test] 
    fn tests_smart_pointers_rc() {

        let mut x = Rc::new(50); // reference counter
        let y = Rc::clone(&x); // new pointer to existing data

        x = Rc::new(70); // changing x, references do not change, Rc is readonly

        dbg!(x);
        dbg!(y);

        // changing value and references with RefCell
        let x1 = Rc::new(RefCell::new(50)); // reference counter
        let y1 = Rc::clone(&x1); // new pointer to existing data
        
        // derefferencing x and borrowing it mutably 
        *x1.borrow_mut() = 70; // changing the value of x
        // because we're using RefCell we can borrow x as mutable and change it's value

        dbg!(x1.borrow()); // accessing value out of RefCell
        dbg!(y1.borrow());

    }
}
        
