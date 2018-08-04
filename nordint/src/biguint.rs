// author:  Erik Nordin
// created: 07/14/2018
// updated: 08/04/2018
// version: 0.1.0
// contact: aeketn@gmail.com

use std::cmp::min;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::str::from_utf8;
use std::str::FromStr;
use ParseBigIntError;

pub const LIMIT: u64 = 1_000_000_000; // under 10 digits
pub const DIGITS_PER_BUCKET: usize = 9;

/// An unbounded, unsigned integer.
///
/// # Internal Representation
/// `BigUint` is represnted internally by a `Vector<u64>`.  
/// Each index of the vector (referred to as a `bucket`) contains
/// up to 9 digits of a number, with the highest-order digits stored at the tail.  
///
/// *Example if bucket-size were 3 digits:*  
/// Number: `123_000_000_000_000_004_560`  
/// Internal: `BigUint { [560, 4, 0, 0, 0, 0, 123] }`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BigUint {
    buckets: Vec<u64>,
}

impl BigUint {
    /// Creates an empty `BigUint` with default capacity 10
    pub fn empty() -> BigUint {
        BigUint {
            buckets: Vec::with_capacity(10),
        }
    }

    /// Creates an empbity `BigUint` with specified capacity
    pub fn with_capacity(capacity: usize) -> BigUint {
        BigUint {
            buckets: Vec::with_capacity(capacity),
        }
    }

    /// Creates a `BigUint` with the value 0
    pub fn zero() -> BigUint {
        BigUint { buckets: vec![0] }
    }

    /// Creates a `BigUint` with the value 1
    pub fn one() -> BigUint {
        BigUint { buckets: vec![1] }
    }

    /// Creates a `BigUint` from a string.
    ///
    /// # How does this differ from the FromStr trait?
    ///
    /// This function will strip out all non-digit characters and never return a `ParseIntError`  
    /// This allows strings with more flexible formatting to be passed in:  
    ///
    /// # Examples
    ///
    /// Each of these strings produces the same `BigUint`.  
    ///
    /// `"123456789123456789_123456789123456789"` : Separated by internal bucket size using an underscore.  
    /// `"000123456789123456789123456789123456789"` : Leading zeros are ignored.  
    /// `"abc123456789123456789LMNOP123456789123456789xyz"` : Letters are ignored.  
    /// `"123,456,789,123,456,789,123,456,789,123,456,789"` : Represented using commas as separators.  
    pub fn new(num_as_str: &str) -> BigUint {
        // Safe to unwrap() because all invalid characters will be filtered out.
        if num_as_str.is_empty() {
            BigUint::empty()
        } else {
            BigUint::from_str(
                &num_as_str
                    .chars()
                    .filter(|character| character.is_digit(10))
                    .collect::<String>(),
            ).unwrap()
        }
    }

    /// Returns the current capacity in `buckets`.  
    /// Filling the `BigUint` beyond capacity will cause it to resize.
    pub fn capacity(&self) -> usize {
        self.buckets.capacity()
    }

    /// # Description
    ///
    /// Calculates the traditional Fibonacci sequence up to the nth element.BigUint
    ///
    /// # Example
    /// ```
    /// extern crate nordint;
    /// use nordint::*;
    /// assert_eq!(BigUint::new("8"), BigUint::fib(6));
    /// // 1, 1, (1+1)=2, (1+2)=3, (2+3)=5, (3+5)=8
    /// ```
    pub fn fib(n: usize) -> BigUint {
        BigUint::fib_generic(BigUint::one(), BigUint::one(), n)
    }

    /// Calculates a generic Fibonacci sequence up to the nth element, provided two starting values.
    ///
    /// # Example
    /// ```
    /// extern crate nordint;
    /// use nordint::*;
    /// let first = BigUint::new("5");
    /// let second = BigUint::new("6");
    /// assert_eq!(BigUint::new("28"), BigUint::fib_generic(first, second, 5));
    /// // 5,   6,   (5+6)=11, (6+11)=17,  (11+17)=28
    /// ```
    pub fn fib_generic(mut first: BigUint, mut second: BigUint, n: usize) -> BigUint {
        match n {
            0 => BigUint::empty(),
            1 => first,
            2 => second,
            _ => {
                for i in 3..=n {
                    if 1 == i & 1 {
                        first += &second;
                    } else {
                        second += &first;
                    }
                }

                if 1 == n & 1 {
                    first
                } else {
                    second
                }
            }
        }
    }

    pub fn fac(n: usize) -> BigUint {
        let mut result = BigUint::one();
        (1..n + 1).rev().for_each(|x| {
            result *= x as u64;
        });
        result
    }
}

impl Default for BigUint {
    /// Default `BigUint` is empty.
    fn default() -> BigUint {
        BigUint::empty()
    }
}

impl FromStr for BigUint {
    type Err = ParseBigIntError;

    /// Creates a `BigUint` from a provided string.  
    /// This function will throw a `ParseIntError` if the provided string is not entirely numerical.  
    /// *Note:* If you want more flexible formatting, use `BigUing::new()`  
    fn from_str(num_as_str: &str) -> Result<Self, Self::Err> {
        if num_as_str.is_empty() {
            return Err(Self::Err::empty());
        }

        for digit in num_as_str.chars() {
            if !digit.is_digit(10) {
                return Err(Self::Err::invalid());
            }
        }

        let mut number = BigUint::with_capacity(num_as_str.len() / DIGITS_PER_BUCKET + 1);
        let mut buckets = num_as_str
            .as_bytes()
            .iter()
            .cloned()
            .skip_while(|byte| *byte == b'0')
            .collect::<Vec<u8>>();

        buckets.reverse();
        buckets.chunks_mut(DIGITS_PER_BUCKET).for_each(|chunk| {
            chunk.reverse();
            let bucket = from_utf8(chunk).unwrap();
            number.buckets.push(<u64>::from_str(bucket).unwrap());
        });
        Ok(number)
    }
}

impl ToString for BigUint {
    fn to_string(&self) -> String {
        if self.buckets.is_empty() {
            return String::new();
        }
        // Avoid generating leading zeros on the highest-order bucket.
        let mut num_as_string = self.buckets[self.buckets.len() - 1].to_string();
        // Add each bucket to the string with potential leading zeros
        for bucket in self.buckets.iter().rev().skip(1) {
            let number = &bucket.to_string();
            for _ in number.len()..DIGITS_PER_BUCKET {
                num_as_string.push('0');
            }
            num_as_string += number;
        }
        num_as_string
    }
}

impl<'a> AddAssign<&'a BigUint> for BigUint {
    fn add_assign(&mut self, rhs: &BigUint) {
        let lhs = &mut self.buckets;
        let rhs = &rhs.buckets;
        if lhs.is_empty() || rhs.is_empty() { 
            return;
        }
        let lhs_len = lhs.len();
        let rhs_len = rhs.len();
        let mut carry = add_slices(&mut lhs[..], &rhs[..]);
        if lhs_len < rhs_len {
            lhs.extend_from_slice(&rhs[lhs_len..]);
        }
        if carry == 1 {
            for index in min(lhs_len, rhs_len)..lhs.len() {
                if carry == 0 {
                    break;
                }
                carry = add_slices(&mut lhs[index..], &[carry]);
            }
            if carry == 1 {
                lhs.push(1);
            }
        }
    }
}

#[inline]
fn add_slices(lhs: &mut [u64], rhs: &[u64]) -> u64 {
    let mut carry = 0;
    lhs.iter_mut().zip(rhs.iter()).for_each(|(lx, rx)| {
        *lx += rx + carry;
        carry = if *lx >= LIMIT {
            *lx %= LIMIT;
            1
        } else {
            0
        }
    });
    carry
}

impl MulAssign<u64> for BigUint {
    fn mul_assign(&mut self, rhs: u64) {
        if rhs == 1 || self.buckets.is_empty() { 
            return;
        }
        if rhs == 0 {
            self.buckets = vec![0];
            return;
        }
        let mut carry = 0;
        for bucket in &mut self.buckets {
            *bucket *= rhs;
            *bucket += carry;
            carry = *bucket / LIMIT;
            *bucket %= LIMIT;
        }
        if 0 < carry {
            self.buckets.push(carry);
        }
    }
}