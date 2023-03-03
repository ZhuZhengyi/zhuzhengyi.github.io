/*
 * @lc app=leetcode.cn id=301 lang=rust
 *
 * [301] 删除无效的括号
 *
 * https://leetcode.cn/problems/remove-invalid-parentheses/description/
 *
 * algorithms
 * Hard (55.14%)
 * Likes:    796
 * Dislikes: 0
 * Total Accepted:    87.2K
 * Total Submissions: 158.2K
 * Testcase Example:  '"()())()"'
 *
 * 给你一个由若干括号和字母组成的字符串 s ，删除最小数量的无效括号，使得输入的字符串有效。
 *
 * 返回所有可能的结果。答案可以按 任意顺序 返回。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "()())()"
 * 输出：["(())()","()()()"]
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "(a)())()"
 * 输出：["(a())()","(a)()()"]
 *
 *
 * 示例 3：
 *
 *
 * 输入：s = ")("
 * 输出：[""]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * s 由小写英文字母以及括号 '(' 和 ')' 组成
 * s 中至多含 20 个括号
 *
 *
 */


// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 广度优先搜索
    /// 1. 依次删除s中的各个字符；
    /// 2. 使用一个hashset记录每次删除后的字符串；
    /// 3.
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut level = std::collections::HashSet::new();
        level.insert(s); //
        while !level.is_empty() {
            let mut res = Vec::new();

            // 判断set中各个字符串是否有效
            for s in level.iter() {
                if Solution::is_valid(s) {
                    //将有效的字符串加入到结果集中
                    res.push(s.clone());
                }
            }

            // 如果结果集不为空，则输出结果
            if !res.is_empty() {
                return res;
            }

            // 依次删除当前集中的字符
            let mut next_level = std::collections::HashSet::new();
            for s in level.iter() {
                for i in 0..s.len() {
                    let mut s = s.clone();
                    s.remove(i);
                    next_level.insert(s);
                }
            }

            level = next_level;
        }

        vec![]
    }

    fn is_valid(s: &str) -> bool {
        let mut unpaired_lefts = 0;
        for c in s.chars() {
            match c {
                '(' => unpaired_lefts += 1,
                ')' => {
                    if unpaired_lefts <= 0 {
                        return false;
                    }
                    unpaired_lefts -= 1;
                }
                _ => {}
            }
        }

        unpaired_lefts == 0
    }
}
// @lc code=end
//
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::remove_invalid_parentheses("()())()".into()),
            vec!["()()()".to_string(), "(())()".to_string()]
        );
        assert_eq!(
            Solution::remove_invalid_parentheses("(a)())()".into()),
            vec!["(a)()()".to_string(), "(a())()".to_string()]
        );
        assert_eq!(
            Solution::remove_invalid_parentheses(")(".into()),
            vec!["".to_string()]
        );
    }
}
