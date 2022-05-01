/*
 * @lc app=leetcode.cn id=50 lang=cpp
 *
 * [50] Pow(x, n)
 *
 * https://leetcode-cn.com/problems/powx-n/description/
 *
 * algorithms
 * Medium (37.83%)
 * Likes:    944
 * Dislikes: 0
 * Total Accepted:    283.4K
 * Total Submissions: 749.2K
 * Testcase Example:  '2.00000\n10'
 *
 * 实现 pow(x, n) ，即计算 x 的 n 次幂函数（即，x^n^ ）。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：x = 2.00000, n = 10
 * 输出：1024.00000
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：x = 2.10000, n = 3
 * 输出：9.26100
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：x = 2.00000, n = -2
 * 输出：0.25000
 * 解释：2^-2 = 1/2^2 = 1/4 = 0.25
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * -100.0 < x < 100.0
 * -2^31 <= n <= 2^31-1
 * -10^4 <= x^n <= 10^4
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /**
    * ## 解题思路
    * * 
    * * f(n) = f(n/2) * f(n/2) * f(n%2)
    */
    double myPow(double x, int n) {
        // x == 0
        if (x <= 0.00001 && x >= -0.00001) {
            return 0;
        }
        // x == 1
        if ((x-1<=0.000001 && x-1>=-0.000001)) {
            return 1;
        }
        // x == -1
        if ((x+1<=0.000001 && x+1>=-0.000001)) {
            return n %2?-1:1;
        }
        if (n == -2147483648) {
            return 0;
        }
        if (n<0) {
            return 1 / myPow(x, -n);
        }
        if (n==0) {
            return 1;
        }
        if (n==1) {
            return x;
        }
        return myPow(x, n/2) * myPow(x, n/2) * myPow(x, n%2);
    }
};
// @lc code=end

