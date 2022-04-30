/*
 * @lc app=leetcode.cn id=344 lang=cpp
 *
 * [344] 反转字符串
 *
 * https://leetcode-cn.com/problems/reverse-string/description/
 *
 * algorithms
 * Easy (78.41%)
 * Likes:    580
 * Dislikes: 0
 * Total Accepted:    522.6K
 * Total Submissions: 666.2K
 * Testcase Example:  '["h","e","l","l","o"]'
 *
 * 编写一个函数，其作用是将输入的字符串反转过来。输入字符串以字符数组 s 的形式给出。
 * 
 * 不要给另外的数组分配额外的空间，你必须原地修改输入数组、使用 O(1) 的额外空间解决这一问题。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = ["h","e","l","l","o"]
 * 输出：["o","l","l","e","h"]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = ["H","a","n","n","a","h"]
 * 输出：["h","a","n","n","a","H"]
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 10^5
 * s[i] 都是 ASCII 码表中的可打印字符
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /*
    * ## 解题思路
    * * 双指针法
    */
    void reverseString(vector<char>& s) {
        int l = 0, r = s.size()-1;
        while(l<r) {
            swap(s[l++], s[r--]);
        }
    }
};

void swap(char& a, char& b) {
    a ^= b;
    b ^= a;
    a ^= b;
}
// @lc code=end

