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

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - dfs
    /// 1. 使用一个临时字符串记录一个括号序列；
    /// 2. 依次将左右括号加入到临时字符串尾部；
    /// 3. 如果剩余的左右括号数都为0，则放完了，将临时字符串加入到结果数组中；
    /// 4. 否则 
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();

        fn dfs(p: String, l: i32, r: i32, ans: &mut Vec<String>) {
            match (l, r) {
                (0, 0) => ans.push(p), //左右都放完了，存入结果集
                (l, r) => {
                    if l > 0 {      // 先放'(' 
                        dfs(format!( "{}{}", p, "(" ), l - 1, r, ans);
                    }
                    if r > l {      // 如果剩余')'个数> '(', 放一个'('
                        dfs(format!( "{}{}", p, ")" ), l, r - 1, ans);
                    } 
                }
            }
        }

        dfs(String::new(), n, n, &mut ans);
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

