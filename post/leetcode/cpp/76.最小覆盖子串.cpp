/*
 * @lc app=leetcode.cn id=76 lang=cpp
 *
 * [76] 最小覆盖子串
 *
 * https://leetcode-cn.com/problems/minimum-window-substring/description/
 *
 * algorithms
 * Hard (43.18%)
 * Likes:    1565
 * Dislikes: 0
 * Total Accepted:    219.7K
 * Total Submissions: 508.7K
 * Testcase Example:  '"ADOBECODEBANC"\n"ABC"'
 *
 * 给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 ""
 * 。
 * 
 * 
 * 
 * 注意：
 * 
 * 
 * 对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
 * 如果 s 中存在这样的子串，我们保证它是唯一的答案。
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "ADOBECODEBANC", t = "ABC"
 * 输出："BANC"
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = "a", t = "a"
 * 输出："a"
 * 
 * 
 * 示例 3:
 * 
 * 
 * 输入: s = "a", t = "aa"
 * 输出: ""
 * 解释: t 中两个字符 'a' 均应包含在 s 的子串中，
 * 因此没有符合条件的子字符串，返回空字符串。
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * s 和 t 由英文字母组成
 * 
 * 
 * 
 * 进阶：你能设计一个在 o(n) 时间内解决此问题的算法吗？
 */

// @lc code=start
class Solution {
public:
    /*
    * ## 解题思路
    * * 滑动窗口
    * * 右指针r: 如果当前窗口没有完整包含t，则r右移，扩大窗口范围，直到窗口完整包含t；
    * * 左指针l: 如果当前已经完整包含t，则l右移，减小窗口范围，直到窗口为完整包含t的最小窗口；
    * * 
    */
    string minWindow(string s, string t) {
        string res;   //结果
        unordered_map<char, int> s_map;   //当前窗口内字符数统计
        unordered_map<char, int> t_map;   //目标字符数统计
        int valid_count = 0;      //当前窗口内的有效字符数

        // 初始化目标hash数组
        for(auto c: t) t_map[c]++;

        for(int r=0, l=0; r<s.length(); r++) {
            s_map[s[r]]++;     //当前字符统计次数+1；
            
            //当前字符统计数未超过目标字符统计数时，增加总有效字符数；
            if(s_map[s[r]] <= t_map[s[r]]) valid_count++;

            // 当前窗口左界字符计数>目标字符计数时，收缩左指针
            while(s_map[s[l]] > t_map[s[l]]) s_map[s[l++]]--;
            
            // 刚好
            if (valid_count == t.length()) {
                if(res.empty() || r+1-l < res.size()) {
                    res=s.substr(l, r+1-l);
                } 
            }
        }
       return res;
    }
};
// @lc code=end

