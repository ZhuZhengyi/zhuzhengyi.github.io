/*
 * @lc app=leetcode.cn id=65 lang=cpp
 *
 * [65] 有效数字
 *
 * https://leetcode-cn.com/problems/valid-number/description/
 *
 * algorithms
 * Hard (27.34%)
 * Likes:    310
 * Dislikes: 0
 * Total Accepted:    53.9K
 * Total Submissions: 197.2K
 * Testcase Example:  '"0"'
 *
 * 有效数字（按顺序）可以分成以下几个部分：
 * 
 * 
 * 一个 小数 或者 整数
 * （可选）一个 'e' 或 'E' ，后面跟着一个 整数
 * 
 * 
 * 小数（按顺序）可以分成以下几个部分：
 * 
 * 
 * （可选）一个符号字符（'+' 或 '-'）
 * 下述格式之一：
 * 
 * 至少一位数字，后面跟着一个点 '.'
 * 至少一位数字，后面跟着一个点 '.' ，后面再跟着至少一位数字
 * 一个点 '.' ，后面跟着至少一位数字
 * 
 * 
 * 
 * 
 * 整数（按顺序）可以分成以下几个部分：
 * 
 * 
 * （可选）一个符号字符（'+' 或 '-'）
 * 至少一位数字
 * 
 * 
 * 部分有效数字列举如下：["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3",
 * "3e+7", "+6e-1", "53.5e93", "-123.456e789"]
 * 
 * 部分无效数字列举如下：["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"]
 * 
 * 给你一个字符串 s ，如果 s 是一个 有效数字 ，请返回 true 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "0"
 * 输出：true
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = "e"
 * 输出：false
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = "."
 * 输出：false
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 20
 * s 仅含英文字母（大写和小写），数字（0-9），加号 '+' ，减号 '-' ，或者点 '.' 。
 * 
 * 
 */

#include <string>

using namespace std;

// @lc code=start
class Solution {
private:
    bool isSpace(char c) { return c==' '; }
    bool isSgn(char c) { return (c=='+' || c=='-'); }
    bool isDot(char c) { return c=='.'; }
    bool isNum(char c) { return (c<='9' && c>='0'); }
    bool isE(char c) { return (c=='e'||c=='E'); }
public:
    /**
     ## 解题思路
     * 有效数字 := <[小数|整数]>[[e|E]<整数>] 
     * 小数 := [+|-]<> 
     **/
    bool isNumber(string s) {
        int i = 0;
        bool haveNum = false;

        // skip space
        while(i<s.size() && isSpace(s[i])) ++i;

        if (i<s.size() && isSgn(s[i])) ++i;

        while(i<s.size())

    }

    bool isFloat(string& s, int i, int j) {

    }

    bool isInt(string& s, int i, int j) {

    }
};
// @lc code=end

