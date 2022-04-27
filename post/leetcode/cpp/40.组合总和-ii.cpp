/*
 * @lc app=leetcode.cn id=40 lang=cpp
 *
 * [40] 组合总和 II
 *
 * https://leetcode-cn.com/problems/combination-sum-ii/description/
 *
 * algorithms
 * Medium (60.80%)
 * Likes:    935
 * Dislikes: 0
 * Total Accepted:    282.6K
 * Total Submissions: 464.9K
 * Testcase Example:  '[10,1,2,7,6,1,5]\n8'
 *
 * 给定一个候选人编号的集合 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。
 * 
 * candidates 中的每个数字在每个组合中只能使用 一次 。
 * 
 * 注意：解集不能包含重复的组合。 
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: candidates = [10,1,2,7,6,1,5], target = 8,
 * 输出:
 * [
 * [1,1,6],
 * [1,2,5],
 * [1,7],
 * [2,6]
 * ]
 * 
 * 示例 2:
 * 
 * 
 * 输入: candidates = [2,5,2,1,2], target = 5,
 * 输出:
 * [
 * [1,2,2],
 * [5]
 * ]
 * 
 * 
 * 
 * 提示:
 * 
 * 
 * 1 <= candidates.length <= 100
 * 1 <= candidates[i] <= 50
 * 1 <= target <= 30
 * 
 * 
 */

#include "include/solution.h"

// @lc code=start
class Solution {
    vector<vector<int>> result;
public:
    /**
    * ## 解题思路
    * * 回溯法，在第39的基础上，改变递归时的判断条件
    */
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {

        vector<int> tmp;
        sort(candidates.begin(), candidates.end());
        dfs(candidates, tmp, target, 0);
        return result;
    }

    void dfs(vector<int>& candidates, vector<int>& tmp, int target, int s) {
        if (target < 0) {
            return;
        }
        if (target==0) {
            result.push_back(tmp);
        }
        for (int i=s; i<candidates.size(); i++) {
            // 当前元素大于剩余target, 或
            // 当前相等元素已经去过，则跳过
            if (candidates[i]>target || (i>s && candidates[i] == candidates[i-1])) {
                continue;
            }
            //
            tmp.push_back(candidates[i]);
            //当前元素只取一次，下次取后一个
            dfs(candidates, tmp, target-candidates[i], i+1);
            tmp.pop_back();
        }
    }
};
// @lc code=end

