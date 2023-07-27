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
impl Solution {
    /// ## 解题思路
    /// - hashmap+滑动窗口
    /// 1. 设置 hashmap, 用于记录已经遍历的各个字符最后一次出现的下标;
    /// 2. 设置变量start用于记录滑窗左边起始边界;
    /// 3. 从左至右,依次遍历字符串各个字符;
    /// 4. 遍历时, 先根据字符从hashmap中检查是否重复出现,
    ///    如果出现, 且在滑窗起始边界之后, 则更新滑窗起始边界到重复字符最近下标的下一个位置;
    ///    否则, 起始边界start不变;
    /// 5. 同时计算更新最大滑窗长度;
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut char_last_index = HashMap::new(); //统计已经出现过的字符最后下标
        let mut start = 0; //滑窗起始边界
        let mut max_len = 0; //滑窗最大长度
        for (i, c) in s.bytes().enumerate() {
            // 如果当前字符重复出现过,且最近下标在滑窗起始边界之后, 则更新滑窗起始边界到最近下标+1的位置;
            // 否则字符第一次出现, 保持滑窗起始边界不变
            start = start.max(*char_last_index.get(&c).unwrap_or(&0));
            max_len = max_len.max(i + 1 - start); //更新最大长度
            char_last_index.insert(c, i + 1); //value使用i+1是因为在计算窗口长度时,
                                              //需要以重复字符的右边一个字符作为起始来计算
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
