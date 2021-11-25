/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 *
 * https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/description/
 *
 * algorithms
 * Medium (32.18%)
 * Likes:    2861
 * Dislikes: 0
 * Total Accepted:    295.5K
 * Total Submissions: 917.9K
 * Testcase Example:  '"abcabcbb"'
 *
 * 给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。
 * 
 * 示例 1:
 * 
 * 输入: "abcabcbb"
 * 输出: 3 
 * 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
 * 
 * 
 * 示例 2:
 * 
 * 输入: "bbbbb"
 * 输出: 1
 * 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
 * 
 * 
 * 示例 3:
 * 
 * 输入: "pwwkew"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
 * 请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
 * 
 * 
 */
// @lc code=start
use std::collections::HashMap;

impl Solution {
    /// ## 解题思路
    /// * 使用一个map记录个字符上一次出现的位置；
    /// * 遍历字符串，根据map计算和上一个字符出现位置的差；
    /// * 记录最大的差，即为结果
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut last_i = 0;
        let mut max_len = 0;
        for (i, c) in s.chars().enumerate() {
            last_i = last_i.max(*map.get(&c).unwrap_or(&0));
            max_len = max_len.max(i + 1 - last_i);
            map.insert(c, i + 1);
        }
        max_len as i32
    }
}
// @lc code=end

