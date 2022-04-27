/*
 * @lc app=leetcode.cn id=77 lang=cpp
 *
 * [77] 组合
 *
 * https://leetcode-cn.com/problems/combinations/description/
 *
 * algorithms
 * Medium (77.02%)
 * Likes:    958
 * Dislikes: 0
 * Total Accepted:    336.5K
 * Total Submissions: 436.8K
 * Testcase Example:  '4\n2'
 *
 * 给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。
 * 
 * 你可以按 任何顺序 返回答案。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 4, k = 2
 * 输出：
 * [
 * ⁠ [2,4],
 * ⁠ [3,4],
 * ⁠ [2,3],
 * ⁠ [1,2],
 * ⁠ [1,3],
 * ⁠ [1,4],
 * ]
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 1, k = 1
 * 输出：[[1]]
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 1 
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
    vector<vector<int>> combine(int n, int k) {
        vector<int> tmp;
        dfs(tmp, n, k, 1);
        return result;
    }

    void dfs(vector<int>& tmp, int total, int left, int start) {
        if (left == 0 ) {
            result.push_back(tmp);
            return;
        }

        // 如果当前数字小于tmp中已存在的最大值，则跳过，继续下一个字符
        for(int i=start; i<=total; i++) {
            // 将当前数字加入tmp
            tmp.push_back(i);
            // 从下一个数开始，深度遍历下一个
            dfs(tmp, total, left-1, i+1);
            // 回退当前所选数
            tmp.pop_back();
        }
    }
};
// @lc code=end

