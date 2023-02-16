/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 *
 * https://leetcode-cn.com/problems/longest-palindromic-substring/description/
 *
 * algorithms
 * Medium (28.58%)
 * Likes:    1805
 * Dislikes: 0
 * Total Accepted:    193.3K
 * Total Submissions: 673.2K
 * Testcase Example:  '"babad"'
 *
 * 给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。
 * 
 * 示例 1：
 * 
 * 输入: "babad"
 * 输出: "bab"
 * 注意: "aba" 也是一个有效答案。
 * 
 * 
 * 示例 2：
 * 
 * 输入: "cbbd"
 * 输出: "bb"
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// * 动态规划
    /// 1. 设dp[i][j]: s[i:j+1]是否为回文子串
    /// 2. 状态转移方程：dp[i][j] = (s[i] == s[j] && dp[i+1][j-1])
    /// 3. 初始条件：dp[i][i]=true
    /* 
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        for i in 0..n {
            dp[i][i] = true;
        }

        let mut longest = 1;
        let mut res: &str = &s[0..1];
        for r in 1..n {
            for l in 0..r {
                if s.chars().nth(l) == s.chars().nth(r) && (r-l<=2 || dp[l+1][r-1] ) {
                    dp[l][r] = true;
                    if r-l+1 > longest {
                        longest = r-l+1;
                        res = &s[l..r+1];
                    } 
                }
            }
        }

        String::from(res)
    }
    */
    pub fn longest_palindrome(s: String) -> String {
        let mut window_size = s.len();
        while window_size > 0 {
            match s.as_bytes()
                    .windows(window_size)
                    .find(|slice|{ 
                        let iter = slice.iter();
                        iter.clone().eq(iter.clone().rev())
                    }) {
                        Some(slice) => return String::from_utf8(slice.to_vec()).unwrap_or("".to_string()),
                        None => window_size -= 1,
                    }
        }
        "".to_string()
    }
}
// @lc code=end

