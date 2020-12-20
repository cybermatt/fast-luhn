#![feature(specialization)]
extern crate pyo3;
extern crate rand;

use pyo3::prelude::*;
use pyo3::{Python, PyResult};
use pyo3::types::PyModule;
use rand::{thread_rng, Rng};


/// Luhn algorithm library.
#[pymodule(fast_luhn)]
fn init_module(_py: Python, m: &PyModule) -> PyResult<()> {

    /// validate(string: str) -> bool
    /// --
    /// Validate string of digits using the Luhn algorithm
    #[pyfn(m, "validate")]
    fn validate(string: &str) -> PyResult<bool> {
        Ok(luhn::validate(string))
    }

    /// digit(string: str) -> int
    /// --
    /// Return the luhn check digit
    #[pyfn(m, "digit")]
    fn digit(string: &str) -> PyResult<u32>{
        Ok(luhn::digit(string))
    }

    /// complete(string: str) -> str
    /// --
    /// Add the luhn check digit to string
    #[pyfn(m, "complete")]
    fn complete(string: &str) -> PyResult<String> {
        Ok(luhn::complete(string))
    } 

    /// generate(length: str) -> str
    /// --
    /// Ceate new luhn valid string. Length must be greater than 1.
    #[pyfn(m, "generate")]
    fn generate(length: i32) -> PyResult<String> {  // TODO: prefix
        Ok(luhn::generate(length))
    }

    Ok(())
}

mod luhn {

    use super::*;

    const RADIX: u32 = 10;

    pub fn checksum(string: &str) -> u32 {
        let mut fsum: u32 = 0;
        let length = string.chars().count();    

        for (idx, c) in string.chars().enumerate() {
            if c.is_digit(RADIX) == false { continue; }
            let mut dint = c.to_digit(RADIX).unwrap();
            if (length - idx) % 2 == 0 {
                dint *= 2;
                if dint > 9 {
                    dint -= 9;
                }
            }
            fsum += dint;
        }
        return fsum;
    }

    pub fn validate(string: &str) -> bool {
        checksum(&string) % 10 == 0
    }

    pub fn digit(string: &str) -> u32{
        let mut valid_string = string.clone().to_string();
        valid_string.push_str("0");
        let chsum = checksum(&valid_string);
        let x = 10 - (chsum % 10);
        let res = if x == 10 { 0 } else { x };
        res
    }

    pub fn complete(string: &str) -> String {
        let mut valid_string = string.to_string();
        let digit = digit(string);
        valid_string.push_str(&digit.to_string());
        valid_string
    } 

    pub fn generate(length: i32) -> String {
        let len = if length > 1 { length } else { 0 };
        let mut rng = thread_rng();
        let mut new_string = String::new();
        for _ in 0..len-1 {
            let n1: u8 = rng.gen_range(0, 10);
            new_string.push_str(&n1.to_string());
        }
        complete(&new_string)
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_checksum() {
        assert_eq!(luhn::checksum("1234567890"), 43);
        assert_eq!(luhn::checksum("5457210001000019"), 30);
        assert_eq!(luhn::checksum("5457210001000043"), 30);
        assert_eq!(luhn::checksum("4716293094402"), 50);
        assert_eq!(luhn::checksum("0"), 0);
        assert_eq!(luhn::checksum("1"), 1);
    }

    #[test]
    fn test_validate() {
        assert_eq!(luhn::validate("5152480083848100"), true);
        assert_eq!(luhn::validate("491674804530447"), false);
    }

    #[test]
    fn test_complete() {
        assert_eq!(luhn::complete("524402422676340"), "5244024226763402");
        assert_eq!(luhn::complete("530449279124605"), "5304492791246052");
    }

    #[test]
    fn test_generate() {
        let mut rng = thread_rng();
        for _ in 1..1000 {
            let length = rng.gen_range(1, 20);
            let ns = luhn::generate(length);
            assert_eq!(ns.len(), length as usize);
            assert_eq!(luhn::checksum(&ns) % 10, 0);
        }
    }

}
