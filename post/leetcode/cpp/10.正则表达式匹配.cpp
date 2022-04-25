/*
 * @lc app=leetcode.cn id=10 lang=cpp
 *
 * [10] 正则表达式匹配
 *
 * https://leetcode-cn.com/problems/regular-expression-matching/description/
 *
 * algorithms
 * Hard (31.61%)
 * Likes:    2932
 * Dislikes: 0
 * Total Accepted:    267.7K
 * Total Submissions: 847.1K
 * Testcase Example:  '"aa"\n"a"'
 *
 * 给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。
 * 
 * 
 * '.' 匹配任意单个字符
 * '*' 匹配零个或多个前面的那一个元素
 * 
 * 
 * 所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "aa", p = "a"
 * 输出：false
 * 解释："a" 无法匹配 "aa" 整个字符串。
 * 
 * 
 * 示例 2:
 * 
 * 
 * 输入：s = "aa", p = "a*"
 * 输出：true
 * 解释：因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = "ab", p = ".*"
 * 输出：true
 * 解释：".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 20
 * 1 <= p.length <= 30
 * s 只包含从 a-z 的小写字母。
 * p 只包含从 a-z 的小写字母，以及字符 . 和 *。
 * 保证每次出现字符 * 时，前面都匹配到有效的字符
 * 
 * 
 */
#include <string>
using namespace std;

// @lc code=start
class Solution {
public:
    /*
    * ## 解题思路
    * * 递归
    * 1. f(s, p): 表示s, p匹配情况；
    * 2. 显然f(s, p)可以递归表示;
    * 3. 初始边界：len(p) == 0 时，return len(s)==0;
    * 4. p中存在3类字符: 普通字母, '.', '*';
    * 3. 对于普通字母或'.'，可以用归纳法建立如下的递推：
    *    f(s,p) = match(s[0],p[0]) && f(s[1:], p[1:])
    * 4. 对于'*'字符，和前一个字符组成一个匹配单位，可以匹配0次或多次
    *    如果匹配了0次，则p前2个字符忽略, 使用p后续[2:]的来匹配s:  
    *           f(s,p) = f(s, p[2:]) 
    *    或者匹配多次，则可先匹配第一个字符，再重新用p匹配后续的s[1:]: 
    *           f(s,p)  = match(s[0], p[0]) && f(s[1:], p) 
    */
    bool isMatch(string s, string p) {
        // 如果p为空，
        if (p.length() == 0) {
            return s.length() == 0;
        }
        // 第一个字符是否匹配
        bool charMatch = false;
        if (s.length() > 0 && (p[0] == s[0] || p[0] == '.')){
            charMatch = true;
        }

        //x*匹配的情况
        if (p.length() >= 2 && p[1] == '*') {
            return isMatch(s, p.substr(2))   //x* 匹配了0个，则检查f(s, p[2:])
                || (charMatch && isMatch(s.substr(1), p));  //x*匹配多个，拆分为1个和
        } else {
            //无'*'
            return charMatch && isMatch(s.substr(1), p.substr(1));
        }

    }
};
// @lc code=end

