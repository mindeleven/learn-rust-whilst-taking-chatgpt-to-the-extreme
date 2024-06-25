#![allow(dead_code, unused_imports)]

#[derive(Debug)]
enum CarColor {
    Red,
    Green,
    Blue,
    Silver
}

// result enum with generic type
#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E)
}

fn create_car_color_blue() -> CarColor {
    let my_car_color = CarColor::Blue;
    my_car_color
}

fn check_under_five(num: u8) -> GivenResult<u8, String> {
    if num < 5 {
        GivenResult::Ok(num)
    } else {
        GivenResult::Err(String::from("Not under 5!!!"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_enums -- --nocapture
    #[test] 
    fn tests_enums() {
        let my_car_color = create_car_color_blue();
        dbg!(my_car_color);

        let under_five_res = check_under_five(2);
        dbg!(under_five_res);

        let under_five_res = check_under_five(7);
        dbg!(under_five_res);
    }

}