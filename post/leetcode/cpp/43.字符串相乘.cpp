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
    * 1234 * 5678 = (1000+200+30+4)*(5000+600+70+8)
                  = 1000 * 5000   
                  + 1000*600 + 200*5000  
                  + 1000*70 + 200*600 + 30*5000 
                  + 1000*8 + 200*70 + 30*600 + 4*5000
                  ...
                  + 4*8
    * 对于结果第k位的数，是由分别有nums1和nums2中下标 i+j = k
      的数字相乘并累加(再加上累积进位)得到的
      即    res[k] =  nums1[0]*nums2[k] 
                    + nums1[1]*nums2[k-1]
                    ...
                    + nums1[k]*nums2[0]
                    + carry
    */
    string multiply(string num1, string num2) {
        if(num1 == "0" || num2 == "0") {
            return "0";
        }
        if(num1 == "1" || num2 == "1") {
            return num1=="1"?num2:num1;
        }
        
        int carry = 0;   //进位
        int m = num1.size()-1;
        int n = num2.size()-1;
        
        string product;
        for (int i=0; i<=m+n || carry; ++i) {
            for (int j=max(0, i-n); j<=min(i, m); ++j)
                carry += (num1[m-j] - '0') * (num2[n-i+j] - '0');
            product += carry % 10 + '0';
            carry /= 10;
        }
        reverse(begin(product), end(product));
        return product;

    }
};
// @lc code=end

