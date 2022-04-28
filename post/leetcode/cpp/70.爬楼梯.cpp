/*
 * @lc app=leetcode.cn id=70 lang=cpp
 *
 * [70] 爬楼梯
 *
 * https://leetcode-cn.com/problems/climbing-stairs/description/
 *
 * algorithms
 * Easy (53.23%)
 * Likes:    2083
 * Dislikes: 0
 * Total Accepted:    649.7K
 * Total Submissions: 1.2M
 * Testcase Example:  '2'
 *
 * 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
 * 
 * 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
 * 
 * 注意：给定 n 是一个正整数。
 * 
 * 示例 1：
 * 
 * 输入： 2
 * 输出： 2
 * 解释： 有两种方法可以爬到楼顶。
 * 1.  1 阶 + 1 阶
 * 2.  2 阶
 * 
 * 示例 2：
 * 
 * 输入： 3
 * 输出： 3
 * 解释： 有三种方法可以爬到楼顶。
 * 1.  1 阶 + 1 阶 + 1 阶
 * 2.  1 阶 + 2 阶
 * 3.  2 阶 + 1 阶
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /**
    * ## 解题思路
    * * 动态规划
    * 1. 令f(n): 爬到第n级台阶的方法数；
    * 2. f(n) = f(n-1) + f(n-2)
    */
    int climbStairs(int n) {
        if (n<3) {
            return n;
        }
        int f0 = 1;
        int f1 = 1;
        for (int i=1; i<n; i++) {
            int tmp = f1;
            f1 = f0 + f1;
            f0 = tmp;
        }

        return f1;
    }
};
// @lc code=end

