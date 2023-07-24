/*
 * @lc app=leetcode.cn id=132 lang=rust
 *
 * [132] 分割回文串 II
 *
 * https://leetcode.cn/problems/palindrome-partitioning-ii/description/
 *
 * algorithms
 * Hard (49.93%)
 * Likes:    684
 * Dislikes: 0
 * Total Accepted:    78.6K
 * Total Submissions: 157.3K
 * Testcase Example:  '"aab"'
 *
 * 给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是回文。
 *
 * 返回符合要求的 最少分割次数 。
 *
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "aab"
 * 输出：1
 * 解释：只需一次分割就可将 s 分割成 ["aa","b"] 这样两个回文子串。
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "a"
 * 输出：0
 *
 *
 * 示例 3：
 *
 *
 * 输入：s = "ab"
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * s 仅由小写英文字母组成
 *
 *
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 dp[i]: 表示将字符串s[..i]进行回文分隔的最小划分数;
    /// 2. 递推关系: dp[i] = min(dp[j]) + 1 ( 0<j<i, s[j..i]为回文串 )
    /// 3. 初始值: dp[0] = [], dp[1] = [];
    /// 4. 目标值: dp[n]
    /// 5. 为了计算 s[j..i]是否为回文串,
    ///    令 g[j][i]: 表示s[j..i]是否为回文串
    /// 6. 则 g[j][i] = g[j+1][i-1] && s[j] == s[i] ( j < i )
    /// 7. g[i][i+1] = true
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        // 预计算子回文串
        let mut g = vec![vec![true; n]; n];
        for i in (0..n).rev() {
            for j in i + 1..n {
                g[i][j] = s[i] == s[j] && g[i + 1][j - 1];
            }
        }

        // 根据回文子串, 计算最小回文子串划分次数
        let mut dp = vec![(n - 1) as i32; n];
        for i in 0..n {
            // 如果s[..i]为回文串
            if g[0][i] {
                dp[i] = 0; // 则最小划分次数为0
            } else {
                // 否则, s[..i]不为回文串
                // 则依次找出s[..i] 中j满足s[j..i]为回文串的最小dp[..j]
                for j in 0..i {
                    if g[j + 1][i] {
                        dp[i] = dp[i].min(dp[j] + 1); // dp[i] 为满足条件的最小dp[j] + 1
                    }
                }
            }
        }

        dp[n - 1]
    }
}
// @lc code=end

struct Solution;
