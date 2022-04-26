/*
 * @lc app=leetcode.cn id=47 lang=cpp
 *
 * [47] 全排列 II
 *
 * https://leetcode-cn.com/problems/permutations-ii/description/
 *
 * algorithms
 * Medium (64.55%)
 * Likes:    1036
 * Dislikes: 0
 * Total Accepted:    305.4K
 * Total Submissions: 473.1K
 * Testcase Example:  '[1,1,2]'
 *
 * 给定一个可包含重复数字的序列 nums ，按任意顺序 返回所有不重复的全排列。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,1,2]
 * 输出：
 * [[1,1,2],
 * ⁠[1,2,1],
 * ⁠[2,1,1]]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [1,2,3]
 * 输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= nums.length <= 8
 * -10 <= nums[i] <= 10
 * 
 * 
 */

#include<vector>
using namespace std;

// @lc code=start
class Solution {
    vector<vector<int>> result;

public:
    /*
    * ## 解题思路
    * * 回溯法：在46.全排列基础上，增加2个步骤，以处理重复元素；
    * 1. 先对nums进行排序，排序后，重复元素将在nums连续排列；
    * 2. 在递归时，增加重复元素的过滤条件；
    */
    vector<vector<int>> permuteUnique(vector<int>& nums) {
        vector<int> tmp;            //一次排列
        set<int> visited;           //记录已取元素下标
        sort(nums.begin(), nums.end());
        dfs(nums, tmp, visited);

        return result;
    }

    void dfs(vector<int>&nums, vector<int>& tmp, set<int>& visited) {
        if (tmp.size() == nums.size()) {
            result.push_back(tmp);
            return;
        }
        for(int j=0; j<nums.size(); j++) {
            //跳过已取出的元素
            if (visited.find(j)!=visited.end() || (j>0&&nums[j] == nums[j-1]&&visited.find(j-1)==visited.end())) {
                continue;
            }
            //将未取出的元素取出，加入到tmp中
            tmp.push_back(nums[j]);
            visited.insert(j);  //并记录到visited 集合中；
            //递归处理其他元素
            dfs(nums, tmp, visited);
            //递归返回时，放回已取出的元素
            tmp.pop_back();
            visited.erase(j);
        }
    }

};

// @lc code=end

