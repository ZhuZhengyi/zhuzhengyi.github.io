// solution.rs
//

pub use crate::*;

/// Solution
pub struct Solution {
}

impl Solution {
    pub fn new() -> Self {
        Solution{}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = Solution::new();
        println!("test1");
    }

}
