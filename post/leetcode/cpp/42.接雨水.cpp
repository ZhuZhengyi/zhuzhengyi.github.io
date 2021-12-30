/*
 * @lc app=leetcode.cn id=42 lang=cpp
 *
 * [42] 接雨水
 *
 * https://leetcode-cn.com/problems/trapping-rain-water/description/
 *
 * algorithms
 * Hard (58.97%)
 * Likes:    2959
 * Dislikes: 0
 * Total Accepted:    369.5K
 * Total Submissions: 625.8K
 * Testcase Example:  '[0,1,0,2,1,0,1,3,2,1,2,1]'
 *
 * 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 
 * 
 * 输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
 * 输出：6
 * 解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。 
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：height = [4,2,0,3,2,5]
 * 输出：9
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * n == height.length
 * 1 <= n <= 2 * 10^4
 * 0 <= height[i] <= 10^5
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /*
    * ## 解题思路
    * * 双指针法
    * 1. 分别用l,r指向数组左右边界；
    * 2. 依次判断height[l], height[r]大小，将小的指针向中间移动一格，同时记录小的高度lower
    * 3. 根据当前lower, 更新已经扫描过的(两外侧)柱子的较低高度level；
    * 4. 当前柱子能够接的雨水为level-lower；
    * 5. 总雨水量就是将每个柱子接水量的和；
    */
    int trap(vector<int>& height) {
        int l = 0;
        int r = height.size() - 1;
        int level = 0; //当前桶的高度
        int water = 0; //
        while (l<r) {
            int lower = height[ height[l] < height[r] ? l++: r-- ];
            level = max(level, lower);
            water += level - lower;
        }

        return water;
    }
};
// @lc code=end

