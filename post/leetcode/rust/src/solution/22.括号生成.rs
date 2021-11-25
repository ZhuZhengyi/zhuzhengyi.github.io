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
    /// ## 解题思路
    /// * 使用一个临时字符串记录一个括号序列；
    /// * 依次将左右括号加入到临时字符串尾部；
    /// * 如果剩余的左右括号数都为0，则放完了，将临时字符串加入到结果数组中；
    /// * 否则 
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();

        fn travel(p: String, l: i32, r: i32, ans: &mut Vec<String>) {
            match (l, r) {
                (0, 0) => ans.push(p),
                (l, r)  => {
                    if l > 0 {
                        travel(format!( "{}{}", p, "(" ), l - 1, r, ans);
                    }
                    if r > l {
                        travel(format!( "{}{}", p, ")" ), l, r - 1, ans);
                    } 
                }
            }
            //左右都放完了，存入结果集
            // 左括号还有剩余
            // 右括号数多于左括号数
        }

        travel(String::new(), n, n, &mut ans);
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

