/*
 * @lc app=leetcode.cn id=242 lang=cpp
 *
 * [242] 有效的字母异位词
 *
 * https://leetcode-cn.com/problems/valid-anagram/description/
 *
 * algorithms
 * Easy (65.23%)
 * Likes:    565
 * Dislikes: 0
 * Total Accepted:    409.2K
 * Total Submissions: 627.1K
 * Testcase Example:  '"anagram"\n"nagaram"'
 *
 * 给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。
 * 
 * 注意：若 s 和 t 中每个字符出现的次数都相同，则称 s 和 t 互为字母异位词。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: s = "anagram", t = "nagaram"
 * 输出: true
 * 
 * 
 * 示例 2:
 * 
 * 
 * 输入: s = "rat", t = "car"
 * 输出: false
 * 
 * 
 * 
 * 提示:
 * 
 * 
 * 1 
 * s 和 t 仅包含小写字母
 * 
 * 
 * 
 * 
 * 进阶: 如果输入字符串包含 unicode 字符怎么办？你能否调整你的解法来应对这种情况？
 * 
 */

// @lc code=start
class Solution {
public:
    /**
     * ## 解题思路
     * 1. 使用map来统计char 数；
     * 2. 遍历时，s中的字符count+1,t中的字符count-1;
     * 3. 最后判断map中所有count是否都为0；
    */
    bool isAnagram(string s, string t) {
        if (s.length()!=t.length()) {
            return false;
        }
        map<char, int> m;
        for(int i=0; i<s.length(); i++) {
            m[s[i]]++;
            m[t[i]]--;
        }
        for(auto a: m) {
            if (a.second != 0) {
                return false;
            }
        }
        return true;
    }
};
// @lc code=end

