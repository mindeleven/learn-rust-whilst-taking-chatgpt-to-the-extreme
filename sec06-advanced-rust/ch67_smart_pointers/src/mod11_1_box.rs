#![allow(dead_code, unused_imports, unused_variables)]

#[cfg(test)]
mod test {
    use super::*;

    // cargo test tests_smart_pointers_box -- --nocapture
    #[test] 
    fn tests_smart_pointers_box() {
        
        #[derive(Debug)]
        struct Node {
            id: u64,
            next: Option<Box<Node>>,
        }

        let node = Node { 
            id: 0, 
            next: Some(Box::new(
                Node { id: 0, next: Some(Box::new(
                    Node { id: 1, next: Some(Box::new(
                        Node { id: 2, next: Some(Box::new(
                            Node { id: 3, next: Some(Box::new(
                                Node { id: 4, next: Some(Box::new(
                                    Node { id: 5, next: None }
                                ))}
                            ))}
                        ))}
                    ))}
                ))}
            )) 
        };
        
        // Node { id: 0, next: None }

        dbg!(node);
    
    }
}
