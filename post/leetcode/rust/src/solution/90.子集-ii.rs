/*
 * @lc app=leetcode.cn id=90 lang=rust
 *
 * [90] 子集 II
 *
 * https://leetcode.cn/problems/subsets-ii/description/
 *
 * algorithms
 * Medium (63.60%)
 * Likes:    1117
 * Dislikes: 0
 * Total Accepted:    306.5K
 * Total Submissions: 481.7K
 * Testcase Example:  '[1,2,2]'
 *
 * 给你一个整数数组 nums ，其中可能包含重复元素，请你返回该数组所有可能的子集（幂集）。
 *
 * 解集 不能 包含重复的子集。返回的解集中，子集可以按 任意顺序 排列。
 *
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,2,2]
 * 输出：[[],[1],[1,2],[1,2,2],[2],[2,2]]
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
 *
 *
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 回溯法(dfs)
    /// 1. 首先对所有的整数排序;
    /// 2. 遍历访问时, 跳过和前一元素相等的重复元素;
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(nums: &Vec<i32>, from: usize, tmp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            // 递归终止条件
            if from >= nums.len() {
                return;
            }
            for i in from..nums.len() {
                // 跳过重复出现过的数字
                if i > from && nums[i] == nums[i - 1] {
                    continue;
                }
                tmp.push(nums[i]);
                res.push(tmp.clone());
                dfs(nums, i + 1, tmp, res);
                tmp.pop();
            }
        }

        let mut tmp = Vec::new();
        let mut res = Vec::new();

        res.push(tmp.clone());
        let mut nums = nums;
        nums.sort(); // 将原序列排序
        dfs(&nums, 0, &mut tmp, &mut res);

        res
    }
}
// @lc code=end

struct Solution;
