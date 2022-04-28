/*
 * @lc app=leetcode.cn id=39 lang=cpp
 *
 * [39] 组合总和
 *
 * https://leetcode-cn.com/problems/combination-sum/description/
 *
 * algorithms
 * Medium (72.75%)
 * Likes:    1922
 * Dislikes: 0
 * Total Accepted:    484.6K
 * Total Submissions: 666K
 * Testcase Example:  '[2,3,6,7]\n7'
 *
 * 给你一个 无重复元素 的整数数组 candidates 和一个目标整数 target ，找出 candidates 中可以使数字和为目标数 target
 * 的 所有 不同组合 ，并以列表形式返回。你可以按 任意顺序 返回这些组合。
 * 
 * candidates 中的 同一个 数字可以 无限制重复被选取 。如果至少一个数字的被选数量不同，则两种组合是不同的。 
 * 
 * 对于给定的输入，保证和为 target 的不同组合数少于 150 个。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：candidates = [2,3,6,7], target = 7
 * 输出：[[2,2,3],[7]]
 * 解释：
 * 2 和 3 可以形成一组候选，2 + 2 + 3 = 7 。注意 2 可以使用多次。
 * 7 也是一个候选， 7 = 7 。
 * 仅有这两种组合。
 * 
 * 示例 2：
 * 
 * 
 * 输入: candidates = [2,3,5], target = 8
 * 输出: [[2,2,2,2],[2,3,3],[3,5]]
 * 
 * 示例 3：
 * 
 * 
 * 输入: candidates = [2], target = 1
 * 输出: []
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= candidates.length <= 30
 * 1 <= candidates[i] <= 200
 * candidate 中的每个元素都 互不相同
 * 1 <= target <= 500
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
    * * 深度优先搜索
    */
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        vector<int> tmp;
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
            // 当前元素大于剩余target, 剪支 
            if (candidates[i]>target) {
                continue;
            }
            //
            tmp.push_back(candidates[i]);
            // 允许重复取当前元素
            dfs(candidates, tmp, target-candidates[i], i);
            tmp.pop_back();
        }

    }
};
// @lc code=end

