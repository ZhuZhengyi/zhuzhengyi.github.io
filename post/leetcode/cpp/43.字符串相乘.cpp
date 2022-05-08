/*
 * @lc app=leetcode.cn id=43 lang=cpp
 *
 * [43] 字符串相乘
 *
 * https://leetcode-cn.com/problems/multiply-strings/description/
 *
 * algorithms
 * Medium (44.84%)
 * Likes:    917
 * Dislikes: 0
 * Total Accepted:    221.8K
 * Total Submissions: 494.8K
 * Testcase Example:  '"2"\n"3"'
 *
 * 给定两个以字符串形式表示的非负整数 num1 和 num2，返回 num1 和 num2 的乘积，它们的乘积也表示为字符串形式。
 * 
 * 注意：不能使用任何内置的 BigInteger 库或直接将输入转换为整数。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: num1 = "2", num2 = "3"
 * 输出: "6"
 * 
 * 示例 2:
 * 
 * 
 * 输入: num1 = "123", num2 = "456"
 * 输出: "56088"
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= num1.length, num2.length <= 200
 * num1 和 num2 只能由数字组成。
 * num1 和 num2 都不包含任何前导零，除了数字0本身。
 * 
 * 
 */

#include <string>
using namespace std;

// @lc code=start
class Solution {
public:
    /*
    ## 解题思路
    * 
    */
    string multiply(string num1, string num2) {
        if(num1 == "0" || num2 == "0") {
            return 0;
        }
        if(num1 == "1" || num2 == "1") {
            return num1=="1"?num2:num1;
        }

    }
};
// @lc code=end

