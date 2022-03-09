/*
 * @lc app=leetcode.cn id=239 lang=cpp
 *
 * [239] 滑动窗口最大值
 *
 * https://leetcode-cn.com/problems/sliding-window-maximum/description/
 *
 * algorithms
 * Hard (49.73%)
 * Likes:    1350
 * Dislikes: 0
 * Total Accepted:    225.3K
 * Total Submissions: 453K
 * Testcase Example:  '[1,3,-1,-3,5,3,6,7]\n3'
 *
 * 给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k
 * 个数字。滑动窗口每次只向右移动一位。
 * 
 * 返回滑动窗口中的最大值。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
 * 输出：[3,3,5,5,6,7]
 * 解释：
 * 滑动窗口的位置                最大值
 * ---------------               -----
 * [1  3  -1] -3  5  3  6  7       3
 * ⁠1 [3  -1  -3] 5  3  6  7       3
 * ⁠1  3 [-1  -3  5] 3  6  7       5
 * ⁠1  3  -1 [-3  5  3] 6  7       5
 * ⁠1  3  -1  -3 [5  3  6] 7       6
 * ⁠1  3  -1  -3  5 [3  6  7]      7
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [1], k = 1
 * 输出：[1]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：nums = [1,-1], k = 1
 * 输出：[1,-1]
 * 
 * 
 * 示例 4：
 * 
 * 
 * 输入：nums = [9,11], k = 2
 * 输出：[11]
 * 
 * 
 * 示例 5：
 * 
 * 
 * 输入：nums = [4,-2], k = 2
 * 输出：[4]
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * -10^4 
 * 1 
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /*
     * ## 解题思路
     * 1. 遍历序列；
     * 2. 使用一个deque来模拟大顶heap，用来记录当前窗口中元素的下标i；
     * 3. curHeap[0]为当前窗口最大元素下标；
     * 4. 遍历时，如果curHeap元素个数>=k(i-curHeap[0]), 则收缩左边元素； 
     * 5. 右侧弹出curHeap中所有<nums[i]的元素, 以保证对
     * 6. 将i加入到curHeap；
     * 7. 从k步开始，依次收集curHeap[0], 即为每个滑窗中的最大元素；
     */
    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        //边界条件
        if(k==0) return {};

        vector<int> res;
        deque<int> curHeap; //当前窗口中,按序排列的值下标

        for(int i=0; i<nums.size(); i++) {
            //窗口中数间隔>=k, 弹出左侧元素，缩小窗口；
            if (!curHeap.empty() && i-curHeap.front() >= k) {
                curHeap.pop_front();
            }
            //弹出窗口右侧所有<nums[i]的元素；
            while(!curHeap.empty() && nums[curHeap.back()] < nums[i]) {
                curHeap.pop_back();
            }
            //将当前坐标加入窗口
            curHeap.push_back(i);

            //i>k开始，窗口
            if (i>k) {
                res.push_back(nums[curHeap.front()]);
            }
        }

        return res;
    }
};
// @lc code=end

