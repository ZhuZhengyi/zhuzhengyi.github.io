/*
 * @lc app=leetcode.cn id=3 lang=cpp
 *
 * [3] 无重复字符的最长子串
 *
 * https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/description/
 *
 * algorithms
 * Medium (38.28%)
 * Likes:    6750
 * Dislikes: 0
 * Total Accepted:    1.4M
 * Total Submissions: 3.7M
 * Testcase Example:  '"abcabcbb"'
 *
 * 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: s = "abcabcbb"
 * 输出: 3 
 * 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
 * 
 * 
 * 示例 2:
 * 
 * 
 * 输入: s = "bbbbb"
 * 输出: 1
 * 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
 * 
 * 
 * 示例 3:
 * 
 * 
 * 输入: s = "pwwkew"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
 * 请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
 * 
 * 
 * 示例 4:
 * 
 * 
 * 输入: s = ""
 * 输出: 0
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 
 * s 由英文字母、数字、符号和空格组成
 * 
 * 
 */
#include <string>
#include <unordered_map>
using namespace std;

// @lc code=start
class Solution {
public:
    /*
    * @brief 解题思路
    * @param
    * 
    * * 滑动窗口 + map
    */
    int lengthOfLongestSubstring(string s) {
        unordered_map<char, int> charIndexMap;  //已经遍历过的字符的最右下标；
        int left = -1;       //滑动窗口左指针，初始是在第一个字符左边
        int max_len = 0;     //最大长度
        for(int i=0; i<s.length(); i++) {
            auto c = s[i];

            //如果之前出现过，且落在滑动窗口左指针的右边，则移动左指针
            if (charIndexMap.find(c) != charIndexMap.end()) {
                left = max(left, charIndexMap[c]);
            }

            //最大子数组长度
            max_len = max(max_len, i-left);

            //
            charIndexMap[c] = i;
        }
        
        return max_len;
    }
};
// @lc code=end

