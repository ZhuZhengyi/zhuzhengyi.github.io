/*
 * @lc app=leetcode.cn id=459 lang=rust
 *
 * [459] 重复的子字符串
 *
 * https://leetcode-cn.com/problems/repeated-substring-pattern/description/
 *
 * algorithms
 * Easy (50.97%)
 * Likes:    577
 * Dislikes: 0
 * Total Accepted:    86.3K
 * Total Submissions: 169.4K
 * Testcase Example:  '"abab"'
 *
 * 给定一个非空的字符串，判断它是否可以由它的一个子串重复多次构成。给定的字符串只含有小写英文字母，并且长度不超过10000。
 * 
 * 示例 1:
 * 
 * 
 * 输入: "abab"
 * 
 * 输出: True
 * 
 * 解释: 可由子字符串 "ab" 重复两次构成。
 * 
 * 
 * 示例 2:
 * 
 * 
 * 输入: "aba"
 * 
 * 输出: False
 * 
 * 
 * 示例 3:
 * 
 * 
 * 输入: "abcabcabcabc"
 * 
 * 输出: True
 * 
 * 解释: 可由子字符串 "abc" 重复四次构成。 (或者子字符串 "abcabc" 重复两次构成。)
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// * 将原始字符串s加倍为新字符串s2；
    /// * 如果原字符串中包含重复子串，则s2[1..2l-1]也包含原字符串
    pub fn repeated_substring_pattern(s: String) -> bool {
        if s.len() < 2 {
            return false;
        }

        let s2 = format!("{}{}", s, s);
        s2[1..2*s.len()-1].contains(&s)
    }
}
// @lc code=end

