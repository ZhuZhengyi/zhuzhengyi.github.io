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

struct Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    /// ## 解题思路
    /// * 滑动窗口法
    ///   - 使用一个滑动窗口来记录每个无重复字符子串；
    ///   - 窗口的右边界`r`为遍历时当前字符数组下标；
    ///   - 窗口的左边界`l`在出现重复字符`c`时，向右滑动一格；
    ///   - 最长子串长度为所有滑动窗口长度`m`的最大值；
    ///   - 使用map来记录每个已经遍历过的字符c的最后一次出现的下标；
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new(); //
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string(),),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbb".to_string(),),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string(),),
            3
        );
    }
}
