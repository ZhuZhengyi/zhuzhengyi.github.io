/*
 * @lc app=leetcode.cn id=1893 lang=cpp
 *
 * [1893] 检查是否区域内所有整数都被覆盖
 *
 * https://leetcode-cn.com/problems/check-if-all-the-integers-in-a-range-are-covered/description/
 *
 * algorithms
 * Easy (59.07%)
 * Likes:    112
 * Dislikes: 0
 * Total Accepted:    32.5K
 * Total Submissions: 55.1K
 * Testcase Example:  '[[1,2],[3,4],[5,6]]\n2\n5'
 *
 * 给你一个二维整数数组 ranges 和两个整数 left 和 right 。每个 ranges[i] = [starti, endi] 表示一个从
 * starti 到 endi 的 闭区间 。
 * 
 * 如果闭区间 [left, right] 内每个整数都被 ranges 中 至少一个 区间覆盖，那么请你返回 true ，否则返回 false 。
 * 
 * 已知区间 ranges[i] = [starti, endi] ，如果整数 x 满足 starti i ，那么我们称整数x 被覆盖了。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：ranges = [[1,2],[3,4],[5,6]], left = 2, right = 5
 * 输出：true
 * 解释：2 到 5 的每个整数都被覆盖了：
 * - 2 被第一个区间覆盖。
 * - 3 和 4 被第二个区间覆盖。
 * - 5 被第三个区间覆盖。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：ranges = [[1,10],[10,20]], left = 21, right = 21
 * 输出：false
 * 解释：21 没有被任何一个区间覆盖。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 1 i i 
 * 1 
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /*
    * ## 解题思路
    * 1. 使用一个数组nums来记录整个区间内所有数据的占用情况；
    * 2. 数组nums建立方法：
    *    * 第一次遍历每个区间（前闭，后开）：
    *      * 在区间开始，将数组nums对应位置的数+1，表示范围开始；
    *      * 在区间结束，将数组nums对应位置的数-1，表示范围结束；
    *    * 第二次遍历数组nums:
    *      * 将nums每个数加上前一个数，将变化率转换为最终的区间占用；
    *  3. 检查nums[left, right+1] 区间内是否都覆盖；    
    * 
    *               |------------|            | -------
    *    -----------|            | -----------|
    *             +1(上升)    -1(下降)
    */
    bool isCovered(vector<vector<int>>& ranges, int left, int right) {
        vector<int> nums(52, 0);   //
        for(auto& range: ranges) {
            nums[range[0]]++;      //区间开始，向上
            nums[range[1]+1]--;    //区间结束，往下
        }
        for(auto i=1; i<nums.size(); i++) {
            nums[i] += nums[i-1];
        }
        // 
        for(auto i=left; i<right+1; i++) {
            if (nums[i] == 0) {
                return false;
            }
        }
        
        return true;
    }
};
// @lc code=end

