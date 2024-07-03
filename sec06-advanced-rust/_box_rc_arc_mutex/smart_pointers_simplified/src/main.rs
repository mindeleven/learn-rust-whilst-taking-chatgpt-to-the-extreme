#![allow(dead_code, unused_variables)]
/// coding along with the youtube video 
/// "Box / Rc / Arc / Mutex - Smart Pointers Simplified - Rust" by Bocksdin Coding
/// @ https://www.youtube.com/watch?v=mNHdD69iLzA

/// let's begin with a box 
/// a box takes a piece of data and places it on the heap
/// especially helpful when you don't know the amount of memory you need
/// at the time you're writing and compiling your code
/// example: link list that can have a potentially infinite size
/// a linked list is a series of nodes that are linked with each other via memory reference
/// defining a linked list in Rust like this returns an error:
/* 
struct LinkListNode {
    value: i32,
    next: LinkListNode // "recursive type `LinkListNode` has infinite size"
}
*/
/// to avoid this we have to give it a defined size
/// Box<> has a defined size that varies by the data it is given
/// a Box is a reference with no guarantee that the value is still there if you try to access it
/// therefore we need to wrap it in an Option
#[derive(Debug)]
struct LinkListNode {
    value: i32,
    next: Option<Box<LinkListNode>>
}

mod m1_rc;

fn main() {
    println!("Coding along with \"Box / Rc / Arc / Mutex - Smart Pointers Simplified - Rust\" by Bocksdin Coding");
}
