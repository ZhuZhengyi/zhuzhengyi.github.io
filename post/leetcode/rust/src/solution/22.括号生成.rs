/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 *
 * https://leetcode-cn.com/problems/generate-parentheses/description/
 *
 * algorithms
 * Medium (73.37%)
 * Likes:    770
 * Dislikes: 0
 * Total Accepted:    77.7K
 * Total Submissions: 105.8K
 * Testcase Example:  '3'
 *
 * 给出 n 代表生成括号的对数，请你写出一个函数，使其能够生成所有可能的并且有效的括号组合。
 * 
 * 例如，给出 n = 3，生成结果为：
 * 
 * [
 * ⁠ "((()))",
 * ⁠ "(()())",
 * ⁠ "(())()",
 * ⁠ "()(())",
 * ⁠ "()()()"
 * ]
 * 
 * 
 */

// @lc code=start
impl Solution {
    /// l, r: 剩余左右括号数
    fn travel(p: String, l: i32, r: i32, ans: &mut Vec<String>) {
        if l > r {  //剩余'(' 多于 ')', 剪支
            return
        }
        if l==0 && r == 0 { //左右都为0， 存入结果集
            ans.push(p);
            return
        }
        if l > 0 {
            Solution::travel(format!("{}{}", p, "("), l - 1, r, ans);
        }
        if r > 0 {
            Solution::travel(format!("{}{}", p, ")"), l, r - 1, ans);
        }
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();

        Solution::travel(String::new(), n, n, &mut ans);
        ans
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
   use super::*;
    #[test]
    fn it_works() {
        let t = Solution::generate_parenthesis(1);
        println!("t={:?}", t);
        let t = Solution::generate_parenthesis(2);
        println!("t={:?}", t);
        let t = Solution::generate_parenthesis(3);
        println!("t={:?}", t);
    }
}

