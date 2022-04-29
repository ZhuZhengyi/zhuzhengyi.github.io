/*
 * @lc app=leetcode.cn id=394 lang=cpp
 *
 * [394] 字符串解码
 *
 * https://leetcode-cn.com/problems/decode-string/description/
 *
 * algorithms
 * Medium (56.01%)
 * Likes:    1124
 * Dislikes: 0
 * Total Accepted:    163.2K
 * Total Submissions: 291.3K
 * Testcase Example:  '"3[a]2[bc]"'
 *
 * 给定一个经过编码的字符串，返回它解码后的字符串。
 * 
 * 编码规则为: k[encoded_string]，表示其中方括号内部的 encoded_string 正好重复 k 次。注意 k 保证为正整数。
 * 
 * 你可以认为输入字符串总是有效的；输入字符串中没有额外的空格，且输入的方括号总是符合格式要求的。
 * 
 * 此外，你可以认为原始数据不包含数字，所有的数字只表示重复的次数 k ，例如不会出现像 3a 或 2[4] 的输入。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "3[a]2[bc]"
 * 输出："aaabcbc"
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = "3[a2[c]]"
 * 输出："accaccacc"
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = "2[abc]3[cd]ef"
 * 输出："abcabccdcdcdef"
 * 
 * 
 * 示例 4：
 * 
 * 
 * 输入：s = "abc3[cd]xyz"
 * 输出："abccdcdcdxyz"
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 30
 * s 由小写英文字母、数字和方括号 '[]' 组成
 * s 保证是一个 有效 的输入。
 * s 中所有整数的取值范围为 [1, 300] 
 * 
 * 
 */

#include <string>

using namespace std;

// @lc code=start
class Solution {
public:
    /**
     * ## 解题思路
     * * 深度遍历
     * 
    */
    string decodeString(string s) {
        int i=0;
        return dfs(s, i);
    }

    /*
    * 递归decode
    * s: 待decode的字符串， 使用引用可避免多余的拷贝；
    * i: 当前decode的起始字符下标，需要将i值返回到调用上级，所以使用引用；
    * 返回：从i开始的decode字符串；
    */
    string dfs(string& s, int& i) {
        string res;
        int repeat = 0;
        while (i < s.length()) {
            char c = s[i++];
            if (c>='0' && c<='9') {
                repeat = 10*repeat + c-'0';
            } else if ( c == '[') {
                string subs = dfs(s, i);
                while (repeat>0) {
                    res += subs;
                    repeat--;
                }
            } else if ( c == ']' ) {
                return res;
            } else {
                res += c;
            }
        }

        return res;
    }
};
// @lc code=end

