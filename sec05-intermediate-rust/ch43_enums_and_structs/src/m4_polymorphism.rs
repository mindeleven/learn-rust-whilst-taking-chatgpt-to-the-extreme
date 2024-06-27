#![allow(dead_code, unused_imports)]

use ethers::types::Address;
use std::str::FromStr; // needed to create an address type from a string

/// structure of a function that should have the ability to receive either 
/// an address type reference or a string reference as an address
/// therefore the generic type
/// after having defined and implemented EthereumAddress trait
/// -> parameter is some generic type that's implemented EthereumAddress
fn get_etherum_address<T: EthereumAddress>(address: T) -> Address {
    let converted_address: Address = address.convert_address().unwrap();
    converted_address
}

/// possilble solution for the above described problem
/// using a trait
trait EthereumAddress {
    // this trait should allow to convert an address for different types
    // Return type is Result with Ethereum address if Ok()
    // a string with a static lifetime if Error
    fn convert_address(&self) -> Result<Address, &'static str>;
}

/// implementing EthereumAddress for a string reference
impl EthereumAddress for &str {
    // just as a reminder: self id "for &str"
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) { // returns result
            // if returning an address from a string works
            // H160 is a hash that's the same type as address
            Ok(addr) => Ok(addr), // extracting the Address out of it
            Err(_) => Err("Invalid Ethereum Address")
        }
    }
}

/// implementing EthereumAddress for an actual Address (from ethers)
impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self) // derefferencing self because we want the address itself, not a pointer
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_polymorphism -- --nocapture
    #[test] 
    fn tests_polymorphism() {
        // random address from etherscan.io
        let ethereum_addr = "0x4aBB3B9De130887942B36De71569AF6a0846Ce5E";
        let addr: Address = Address::from_str(ethereum_addr).unwrap();
        
        let new_addr = get_etherum_address(addr);
        assert_eq!(
            Address::from_str("0x4aBB3B9De130887942B36De71569AF6a0846Ce5E").unwrap(),
            new_addr
        );

        let new_addr_from_str = get_etherum_address("0x4aBB3B9De130887942B36De71569AF6a0846Ce5E");
        assert_eq!(
            Address::from_str("0x4aBB3B9De130887942B36De71569AF6a0846Ce5E").unwrap(),
            new_addr_from_str
        );

    }

}
