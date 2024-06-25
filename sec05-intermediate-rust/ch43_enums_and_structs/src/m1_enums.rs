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

// option enum with generic type
#[derive(Debug)]
enum GivenOption<T> {
    Some(T),
    None
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

// same like above with build in Result
fn check_under_five_build_in(num: u8) -> Result<u8, String> {
    if num < 5 {
        Result::Ok(num)
    } else {
        Result::Err(String::from("Not under 5!!!"))
    }
}


fn remainder_zero(num: f32) -> GivenOption<f32> {
    let remainder_check = num % 10.0;
    if remainder_check != 0.0 {
        GivenOption::Some(remainder_check)
    } else {
        GivenOption::None
    }

}

// same like above with build in Option
fn remainder_zero_build_in(num: f32) -> Option<f32> {
    let remainder_check = num % 10.0;
    if remainder_check != 0.0 {
        Option::Some(remainder_check)
    } else {
        Option::None
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

        let under_five_res_bi = check_under_five_build_in(2);
        dbg!(under_five_res_bi.unwrap());
        
        /* let under_five_res_bi = check_under_five_build_in(7);
        // thread 'm1_enums::test::tests_enums' panicked at src/m1_enums.rs:78:32:
        // called `Result::unwrap()` on an `Err` value: "Not under 5!!!"
        dbg!(under_five_res_bi.unwrap()); */

        let remainder = remainder_zero(12.2);
        dbg!(remainder);

        let remainder = remainder_zero(100.0);
        dbg!(remainder);

        let remainder_build_in = remainder_zero_build_in(12.2);
        dbg!(remainder_build_in);

        let remainder_build_in = remainder_zero_build_in(100.0);
        dbg!(remainder_build_in);
    }

}