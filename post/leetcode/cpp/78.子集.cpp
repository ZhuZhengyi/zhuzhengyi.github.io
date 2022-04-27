/*
 * @lc app=leetcode.cn id=78 lang=cpp
 *
 * [78] 子集
 *
 * https://leetcode-cn.com/problems/subsets/description/
 *
 * algorithms
 * Medium (80.45%)
 * Likes:    1604
 * Dislikes: 0
 * Total Accepted:    431.2K
 * Total Submissions: 535.9K
 * Testcase Example:  '[1,2,3]'
 *
 * 给你一个整数数组 nums ，数组中的元素 互不相同 。返回该数组所有可能的子集（幂集）。
 * 
 * 解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,2,3]
 * 输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [0]
 * 输出：[[],[0]]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * -10 
 * nums 中的所有元素 互不相同
 * 
 * 
 */

#include <vector>

using namespace std;

// @lc code=start
class Solution {
    vector<vector<int>> result;
public:
    /*
    * ## 解题思路
    * * 深度遍历
    */
    vector<vector<int>> subsets(vector<int>& nums) {
        vector<int> tmp;
        result.push_back(tmp);
        dfs(nums, tmp, 0);
        return result;
    }

    void dfs(vector<int>&nums, vector<int>&tmp, int s) {
        if(s >= nums.size()) {
            return;
        }
        for(int i=s; i<nums.size(); i++ ) {
            tmp.push_back(nums[i]);
            result.push_back(tmp);
            dfs(nums, tmp, i+1);
            tmp.pop_back();
        }
    }
};
// @lc code=end

