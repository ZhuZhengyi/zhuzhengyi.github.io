/*
 * @lc app=leetcode.cn id=384 lang=cpp
 *
 * [384] 打乱数组
 *
 * https://leetcode-cn.com/problems/shuffle-an-array/description/
 *
 * algorithms
 * Medium (61.06%)
 * Likes:    274
 * Dislikes: 0
 * Total Accepted:    98K
 * Total Submissions: 160.5K
 * Testcase Example:  '["Solution","shuffle","reset","shuffle"]\n[[[1,2,3]],[],[],[]]'
 *
 * 给你一个整数数组 nums ，设计算法来打乱一个没有重复元素的数组。打乱后，数组的所有排列应该是 等可能 的。
 * 
 * 实现 Solution class:
 * 
 * 
 * Solution(int[] nums) 使用整数数组 nums 初始化对象
 * int[] reset() 重设数组到它的初始状态并返回
 * int[] shuffle() 返回数组随机打乱后的结果
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入
 * ["Solution", "shuffle", "reset", "shuffle"]
 * [[[1, 2, 3]], [], [], []]
 * 输出
 * [null, [3, 1, 2], [1, 2, 3], [1, 3, 2]]
 * 
 * 解释
 * Solution solution = new Solution([1, 2, 3]);
 * solution.shuffle();    // 打乱数组 [1,2,3] 并返回结果。任何 [1,2,3]的排列返回的概率应该相同。例如，返回
 * [3, 1, 2]
 * solution.reset();      // 重设数组到它的初始状态 [1, 2, 3] 。返回 [1, 2, 3]
 * solution.shuffle();    // 随机返回数组 [1, 2, 3] 打乱后的结果。例如，返回 [1, 3, 2]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= nums.length <= 50
 * -10^6 <= nums[i] <= 10^6
 * nums 中的所有元素都是 唯一的
 * 最多可以调用 10^4 次 reset 和 shuffle
 * 
 * 
 */

#include <vector>

using namespace std;

// @lc code=start
/**
 ## 解题思路
 *  
 **/
class Solution {
    vector<int> nums;           //输出序列
    vector<int> origin;         //原始序列
    default_random_engine seed;  //随机数发生器种子

public:
    Solution(vector<int>& nums):
    nums(nums),
    origin(nums),
    seed(random_device{}())
    {
    }
    
    vector<int> reset() {
        nums = origin;
        return nums;
    }
    
    /*
    ## shuffle算法：
    1. 将原数组分为2部分：
       [  unshuffle nums   | shuffled nums]
    2. 每次使用随机数生成器从 unshuffle nums中选择一个数；
    3. 将其和unshuffle nums 最后一个数交换；
    4. 直到 unshuffle nums 都被交换到 shufflled nums中取；
    */
    vector<int> shuffle() {
        int j = 0;
        for(int i=nums.size()-1; i>0; i--) {
            j = uniform_int_distribution<int>(0, i)(seed);
            if (j!=i) {
                swap(nums[j], nums[i]);
            }
        }
        return nums;
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(nums);
 * vector<int> param_1 = obj->reset();
 * vector<int> param_2 = obj->shuffle();
 */
// @lc code=end

