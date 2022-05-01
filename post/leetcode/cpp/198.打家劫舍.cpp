/*
 * @lc app=leetcode.cn id=198 lang=cpp
 *
 * [198] 打家劫舍
 *
 * https://leetcode-cn.com/problems/house-robber/description/
 *
 * algorithms
 * Medium (52.98%)
 * Likes:    2095
 * Dislikes: 0
 * Total Accepted:    538.5K
 * Total Submissions: 1M
 * Testcase Example:  '[1,2,3,1]'
 *
 * 
 * 你是一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金，影响你偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警。
 * 
 * 给定一个代表每个房屋存放金额的非负整数数组，计算你 不触动警报装置的情况下 ，一夜之内能够偷窃到的最高金额。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：[1,2,3,1]
 * 输出：4
 * 解释：偷窃 1 号房屋 (金额 = 1) ，然后偷窃 3 号房屋 (金额 = 3)。
 * 偷窃到的最高金额 = 1 + 3 = 4 。
 * 
 * 示例 2：
 * 
 * 
 * 输入：[2,7,9,3,1]
 * 输出：12
 * 解释：偷窃 1 号房屋 (金额 = 2), 偷窃 3 号房屋 (金额 = 9)，接着偷窃 5 号房屋 (金额 = 1)。
 * 偷窃到的最高金额 = 2 + 9 + 1 = 12 。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 0 
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /**
     * ## 解题思路
     * * 动态规划
     * 1. 设f(n): 到第n个店铺的最大收益；
     * 2. f(n) = max(f(n-2) + n, f(n-1))
     * 3. f(0) = n[0], f(1) = max(n[0], n[1])
    */
    int rob(vector<int>& nums) {
        if (nums.size() < 2) {
            return max(nums);
        } 
        int  fn = 0, fn2=nums[0], fn1=max(nums[1], nums[0]);
        for(int i=2; i<nums.size(); i++) {
            fn = max(fn2+nums[i], fn1);
            fn2 = fn1;
            fn1 = fn;
        }

        return fn;
    }
};
// @lc code=end

