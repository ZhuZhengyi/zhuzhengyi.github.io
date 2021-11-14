/*
 * @lc app=leetcode.cn id=20 lang=cpp
 *
 * [20] 有效的括号
 *
 * https://leetcode-cn.com/problems/valid-parentheses/description/
 *
 * algorithms
 * Easy (44.52%)
 * Likes:    2719
 * Dislikes: 0
 * Total Accepted:    809.1K
 * Total Submissions: 1.8M
 * Testcase Example:  '"()"'
 *
 * 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
 * 
 * 有效字符串需满足：
 * 
 * 
 * 左括号必须用相同类型的右括号闭合。
 * 左括号必须以正确的顺序闭合。
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "()"
 * 输出：true
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = "()[]{}"
 * 输出：true
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = "(]"
 * 输出：false
 * 
 * 
 * 示例 4：
 * 
 * 
 * 输入：s = "([)]"
 * 输出：false
 * 
 * 
 * 示例 5：
 * 
 * 
 * 输入：s = "{[]}"
 * 输出：true
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * s 仅由括号 '()[]{}' 组成
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /*
    * ## 解题思路
    * - 从左到右顺序遍历每个字符
    * - 使用一个栈来保存没有配对的字符
    * - 如果为左括号，则将对应的右括号入栈；
    * - 如果为右括号，
    *   - 如果栈为空，则不匹配，返回false
    *   - 否则，栈不为空，则检查栈顶字符是否与当前字符相等：
    *       - 如果相等，则继续；
    *       - 否则，不匹配
    * 
    */
    bool isValid(string s) {
        stack<char> parens;
        map<char, char> pairs = {
            {')', '('},
            {'}', '{'},
            {']', '['},
        };

        for(char& c: s) {
            switch(c) {
                case '(':
                case '{':
                case '[': parens.push(c); break;
                case '}': 
                case ']': 
                case ')': {
                    if (parens.empty() || parens.top() != pairs[c] ) {
                        return false;
                    } else {
                        parens.pop();
                    }
                    break;
                }
                default:
                    break;

            }
        }

        return parens.empty();
    }
};
// @lc code=end

