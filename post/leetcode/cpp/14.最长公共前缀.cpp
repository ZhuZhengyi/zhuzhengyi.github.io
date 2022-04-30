/*
 * @lc app=leetcode.cn id=14 lang=cpp
 *
 * [14] 最长公共前缀
 *
 * https://leetcode-cn.com/problems/longest-common-prefix/description/
 *
 * algorithms
 * Easy (42.24%)
 * Likes:    2215
 * Dislikes: 0
 * Total Accepted:    817.1K
 * Total Submissions: 1.9M
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
class Solution {
public:
    /**
     * ## 解题思路
     * * 双层遍历
     * * 遍历时，依次将各个字符串字符和strs[0]的字符对比
     */
    string longestCommonPrefix(vector<string>& strs) {
        string res;
        bool out = false; 
        for(int i=0; !out; i++) {
            for(string& s: strs) {
                if (i>=s.length() || s[i] != strs[0][i]) {
                    out=true;
                    break;
                }
            }
            if (!out) {
                res += strs[0][i];
            }
        }
        return res;
    }
};
// @lc code=end

