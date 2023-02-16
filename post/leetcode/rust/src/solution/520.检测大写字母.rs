/*
 * @lc app=leetcode.cn id=520 lang=rust
 *
 * [520] 检测大写字母
 *
 * https://leetcode-cn.com/problems/detect-capital/description/
 *
 * algorithms
 * Easy (57.62%)
 * Likes:    195
 * Dislikes: 0
 * Total Accepted:    68.3K
 * Total Submissions: 118.5K
 * Testcase Example:  '"USA"'
 *
 * 我们定义，在以下情况时，单词的大写用法是正确的：
 * 
 * 
 * 全部字母都是大写，比如 "USA" 。
 * 单词中所有字母都不是大写，比如 "leetcode" 。
 * 如果单词不只含有一个字母，只有首字母大写， 比如 "Google" 。
 * 
 * 
 * 给你一个字符串 word 。如果大写用法正确，返回 true ；否则，返回 false 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：word = "USA"
 * 输出：true
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：word = "FlaG"
 * 输出：false
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= word.length <= 100
 * word 由小写和大写英文字母组成
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        // "abcde"
        if word.chars().all(|c| c.is_lowercase() ) {
            return true;
        }
        // "ABCDE"
        if word.chars().all(|c| c.is_uppercase() ) {
            return true;
        }
        // "aBCde"
        if word.chars().nth(0).unwrap().is_lowercase() {
            return false;
        }
        // "AbcdEf"
        let w = word.as_str()[1..].to_string();
        if w.chars().any(|c| c.is_uppercase()) {
            return false;
        }

        return true;
    }
}
// @lc code=end

