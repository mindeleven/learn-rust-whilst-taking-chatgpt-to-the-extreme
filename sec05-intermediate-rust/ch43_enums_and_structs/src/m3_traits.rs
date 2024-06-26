#![allow(dead_code, unused_imports)]

/// traits allow to do an interface definition
/// they allow to define a shared interface for methods 
/// that different types need to implement
trait Attacker {
    fn choose_style(&self) -> String;
}

enum Character {
    Warrior,
    Archer,
    Wizard
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => "wing chun".to_string(),
            Character::Archer => "kung fu".to_string(),
            Character::Wizard => "tai chi".to_string(),
        }   
    }
}


#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_traits -- --nocapture
    #[test] 
    fn tests_traits() {

        let my_character: Character = Character::Archer;
        let chosen_fighting_style = my_character.choose_style();
        dbg!(chosen_fighting_style);
    
    }

}

