/*
 * @lc app=leetcode.cn id=49 lang=rust
 *
 * [49] 字母异位词分组
 *
 * https://leetcode.cn/problems/group-anagrams/description/
 *
 * algorithms
 * Medium (67.80%)
 * Likes:    1456
 * Dislikes: 0
 * Total Accepted:    451K
 * Total Submissions: 665.2K
 * Testcase Example:  '["eat","tea","tan","ate","nat","bat"]'
 *
 * 给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。
 *
 * 字母异位词 是由重新排列源单词的字母得到的一个新单词，所有源单词中的字母通常恰好只用一次。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
 * 输出: [["bat"],["nat","tan"],["ate","eat","tea"]]
 *
 * 示例 2:
 *
 *
 * 输入: strs = [""]
 * 输出: [[""]]
 *
 *
 * 示例 3:
 *
 *
 * 输入: strs = ["a"]
 * 输出: [["a"]]
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= strs.length <= 10^4
 * 0 <= strs[i].length <= 100
 * strs[i] 仅包含小写字母
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - hashmap
    /// 1. 将每个字符串排序;
    /// 2. 已排序后的字符串作为key, 将每个字符串放入string->vec<string> hashmap中;
    /// 3. hashmap中的values即为结果;
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();
        for str in strs {
            let mut s = str.as_bytes().to_vec();
            s.sort();
            hashmap
                .entry(String::from_utf8(s).unwrap())
                .or_default()
                .push(str.clone());
        }
        hashmap.into_values().collect()
    }
}
// @lc code=end
