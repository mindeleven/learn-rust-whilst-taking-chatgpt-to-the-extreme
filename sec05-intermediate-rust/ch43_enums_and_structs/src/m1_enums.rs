#![allow(dead_code, unused_imports)]

#[derive(Debug)]
enum CarColor {
    Red,
    Green,
    Blue,
    Silver
}

fn create_car_color_blue() -> CarColor {
    let my_car_color = CarColor::Blue;
    my_car_color
}


#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_enums -- --nocapture
    #[test] 
    fn tests_enums() {
        let my_car_color = create_car_color_blue();
        dbg!(my_car_color);
    }

}