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

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 递归
    /// 1. 模式字符串p中包含以下三种类型的字符: 普通字符，'.', '*';
    /// 2. 对于p,总共可分为以下几种形式:
    ///     a. p以.*开头或a*开头(a相等), 则s的开头部分已经匹配, 需要继续匹配s除开首字符后部分和p是否匹配;
    ///     b. p以a*开头字符不同, 那么*代表0次匹配前面字符, 则p去掉a*部分, 剩下部分和s继续匹配；
    ///     c. p以.或x且x相匹配, 则需要看s和p剩下的部分subs, subp是否匹配;
    ///     d. p为空, 则取决于s是否也为空;
    ///     e. 其他情况都不匹配;
    pub fn is_match(s: String, p: String) -> bool {
        /// s,p字节数组是否匹配
        fn _is_match(s: &[u8], p: &[u8]) -> bool {
            match (p, s) {
                // p为空
                ([], _) => s.is_empty(),
                // p: a*开头
                ([a, b'*', ..], _) => {
                    (s.len() > 0 && (*a == b'.' || *a == s[0]) && _is_match(&s[1..], p))
                        || _is_match(s, &p[2..])
                }
                // p: .或单字符匹配
                ([a, ..], [b, ..]) if (*a == b'.' || a == b) => _is_match(&s[1..], &p[1..]),
                //其他情况
                _ => false,
            }
        }

        _is_match(s.as_bytes(), p.as_bytes())
    }
}

// @lc code=end
//
struct Solution;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(
            Solution::is_match("aaa".to_string(), "a*a".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match(
                "aabcbcbcaccbcaabc".to_string(),
                ".*a*aa*.*b*.c*.*a*".to_string()
            ),
            true
        );
    }
}
