/*
 * @lc app=leetcode.cn id=44 lang=rust
 *
 * [44] 通配符匹配
 *
 * https://leetcode.cn/problems/wildcard-matching/description/
 *
 * algorithms
 * Hard (33.77%)
 * Likes:    1033
 * Dislikes: 0
 * Total Accepted:    135.3K
 * Total Submissions: 400.1K
 * Testcase Example:  '"aa"\n"a"'
 *
 * 给你一个输入字符串 (s) 和一个字符模式 (p) ，请你实现一个支持 '?' 和 '*' 匹配规则的通配符匹配：
 *
 *
 * '?' 可以匹配任何单个字符。
 * '*' 可以匹配任意字符序列（包括空字符序列）。
 *
 *
 *
 *
 * 判定匹配成功的充要条件是：字符模式必须能够 完全匹配 输入字符串（而不是部分匹配）。
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "aa", p = "a"
 * 输出：false
 * 解释："a" 无法匹配 "aa" 整个字符串。
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "aa", p = "*"
 * 输出：true
 * 解释：'*' 可以匹配任意字符串。
 *
 *
 * 示例 3：
 *
 *
 * 输入：s = "cb", p = "?a"
 * 输出：false
 * 解释：'?' 可以匹配 'c', 但第二个 'a' 无法匹配 'b'。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0 <= s.length, p.length <= 2000
 * s 仅由小写英文字母组成
 * p 仅由小写英文字母、'?' 或 '*' 组成
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    ///
    pub fn is_match(s: String, p: String) -> bool {
        fn _is_match(s: &[u8], p: &[u8]) -> bool {
            match (p, s) {
                ([], _) => s.is_empty(),
                ([b'*', ..], _) => (s.len() > 0 && _is_match(&s[1..], p)) || _is_match(s, &p[1..]),
                ([b'?', ..], [_, ..]) => _is_match(&s[1..], &p[1..]),
                ([a, ..], [b, ..]) if a == b => _is_match(&s[1..], &p[1..]),
                _ => false,
            }
        }

        _is_match(s.as_bytes(), p.as_bytes())
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::is_match("adceb".to_string(), "*a*b".into()), true);
    }
}
