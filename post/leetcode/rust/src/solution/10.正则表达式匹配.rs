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
    /// - 两种方法:
    /// 1. 递归
    /// 2. 动态规划
    pub fn is_match(s: String, p: String) -> bool {
        /// - 递归
        /// 1. 模式字符串p中包含以下三种类型的字符: 普通字符，'.', '*';
        /// 2. 对于p,总共可分为以下几种形式:
        ///     a. p为空, 则取决于s是否也为空;
        ///     b. p以.*开头或a*开头(a相等), 则s的开头部分已经匹配, 需要继续匹配s除开首字符后部分和p是否匹配;
        ///     c. p以a*开头字符不同, 那么*代表0次匹配前面字符, 则p去掉a*部分, 剩下部分和s继续匹配；
        ///     d. p以.或x且x相匹配, 则需要看s和p剩下的部分subs, subp是否匹配;
        ///     e. 其他情况都不匹配;
        fn _is_match_rec(s: &[u8], p: &[u8]) -> bool {
            match (p, s) {
                // p为空
                ([], _) => s.is_empty(),
                // p: a*匹配
                ([a, b'*', ..], _) => {
                    _is_match_rec(s, &p[2..])
                        || (s.len() > 0 && (*a == b'.' || *a == s[0]) && _is_match_rec(&s[1..], p))
                }
                // p: .匹配
                ([b'.', ..], [_, ..]) => _is_match_rec(&s[1..], &p[1..]),
                // p: 普通字符匹配
                ([a, ..], [b, ..]) => a == b && _is_match_rec(&s[1..], &p[1..]),
                //其他情况
                _ => false,
            }
        }

        /// - 动态规划
        /// 1. 令 dp[i][j]: s[..i] 和 p[..j]是否匹配;
        /// 2. 目标: dp[s.len()][p.len()]
        /// 3. 递推关系:
        ///     dp[i+1][j+1] =
        ///         ( p[j] == '.' && dp[i][j] )
        ///      || ( p[j] == s[i] && dp[i][j] )
        ///      || ( p[j] == '*' && ( dp[i+1][j-1]
        ///                            || ( (p[j-1] == '.' || p[j-1] == s[i]) && dp[i][j+1] )
        ///                          )
        ///         )
        /// 4. 初始条件:
        ///     dp[0][0] = true, (s, p均为空)
        ///     dp[0][i+1] = p[i] == '*' && dp[0][i-1], (s为空)
        fn _is_match_dp(s: &[u8], p: &[u8]) -> bool {
            let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];

            //s为空,p为空
            dp[0][0] = true;
            for j in 0..p.len() {
                if j > 0 {
                    //s为空, p不为空
                    dp[0][j + 1] = p[j] == b'*' && dp[0][j - 1];
                }
                for i in 0..s.len() {
                    dp[i + 1][j + 1] = match p[j] {
                        b'.' => dp[i][j],
                        b'*' => {
                            dp[i + 1][j - 1] //*匹配0次
                                || ((p[j - 1] == b'.' || p[j - 1] == s[i]) && dp[i][j + 1])
                            //*匹配1或多次
                        }
                        a => dp[i][j] && s[i] == a,
                    };
                }
            }

            dp[s.len()][p.len()]
        }

        //_is_match_rec(s.as_bytes(), p.as_bytes())
        _is_match_dp(s.as_bytes(), p.as_bytes())
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
