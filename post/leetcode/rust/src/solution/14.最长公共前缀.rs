/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 *
 * https://leetcode-cn.com/problems/longest-common-prefix/description/
 *
 * algorithms
 * Easy (41.07%)
 * Likes:    1831
 * Dislikes: 0
 * Total Accepted:    644K
 * Total Submissions: 1.6M
 * Testcase Example:  '["flower","flow","flight"]'
 *
 * 编写一个函数来查找字符串数组中的最长公共前缀。
 * 
 * 如果不存在公共前缀，返回空字符串 ""。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：strs = ["flower","flow","flight"]
 * 输出："fl"
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：strs = ["dog","racecar","car"]
 * 输出：""
 * 解释：输入不存在公共前缀。
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= strs.length <= 200
 * 0 <= strs[i].length <= 200
 * strs[i] 仅由小写英文字母组成
 * 
 * 
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// 依次比较其他字符串各位字符和第一个字符串字符，记录相同的前缀；
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }
        let str0 = &strs[0];
        let mut i = 0;
        while i < strs[0].len() {
            for s in &strs[1..] {
                if s.len() < i || str0.chars().nth(i) != s.chars().nth(i) {
                    return (&str0[..i]).to_string();
                }
            }
            i = i+1;
        }
        return (&str0[..i]).to_string();


    }
}
// @lc code=end

