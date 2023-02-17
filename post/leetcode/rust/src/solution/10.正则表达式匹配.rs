/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 *
 * https://leetcode-cn.com/problems/regular-expression-matching/description/
 *
 * algorithms
 * Hard (30.45%)
 * Likes:    1700
 * Dislikes: 0
 * Total Accepted:    128.5K
 * Total Submissions: 421.8K
 * Testcase Example:  '"aa"\n"a"'
 *
 * 给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。
 *
 *
 * '.' 匹配任意单个字符
 * '*' 匹配零个或多个前面的那一个元素
 *
 *
 * 所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "aa" p = "a"
 * 输出：false
 * 解释："a" 无法匹配 "aa" 整个字符串。
 *
 *
 * 示例 2:
 *
 *
 * 输入：s = "aa" p = "a*"
 * 输出：true
 * 解释：因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
 *
 *
 * 示例 3：
 *
 *
 * 输入：s = "ab" p = ".*"
 * 输出：true
 * 解释：".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
 *
 *
 * 示例 4：
 *
 *
 * 输入：s = "aab" p = "c*a*b"
 * 输出：true
 * 解释：因为 '*' 表示零个或多个，这里 'c' 为 0 个, 'a' 被重复一次。因此可以匹配字符串 "aab"。
 *
 *
 * 示例 5：
 *
 *
 * 输入：s = "mississippi" p = "mis*is*p*."
 * 输出：false
 *
 *
 *
 * 提示：
 *
 *
 * 0
 * 0
 * s 可能为空，且只包含从 a-z 的小写字母。
 * p 可能为空，且只包含从 a-z 的小写字母，以及字符 . 和 *。
 * 保证每次出现字符 * 时，前面都匹配到有效的字符
 *
 *
 */

use super::*;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// 递归
    /// 1. 分三种情形：普通字符，'.', '*',
    ///     a. 对于普通字符和'.', 都只需要检查单个对应位置字符；
    ///     b. 对于'*', 需要检查'*'前一个字符；
    /// 2.
    pub fn is_match(s: String, p: String) -> bool {
        _is_match(s.as_bytes(), p.as_bytes())
    }
}

fn _is_match(s: &[u8], p: &[u8]) -> bool {
    match (p, s) {
        ([x, b'*', _], [y, subs @ ..]) if *x == b'.' || x == y => _is_match(subs, p),
        ([_, b'*', subp @ ..], _) => _is_match(s, subp),
        ([x, subp @ ..], [y, subs @ ..]) if *x == b'.' || x == y => _is_match(subs, subp),
        ([], s) => s.is_empty(),
        _ => false,
    }
}
// @lc code=end

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()),
            false
        );
    }
}
