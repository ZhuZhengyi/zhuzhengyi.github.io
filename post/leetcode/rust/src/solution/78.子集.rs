/*
 * @lc app=leetcode.cn id=78 lang=rust
 *
 * [78] 子集
 *
 * https://leetcode.cn/problems/subsets/description/
 *
 * algorithms
 * Medium (81.11%)
 * Likes:    2071
 * Dislikes: 0
 * Total Accepted:    640.9K
 * Total Submissions: 790.1K
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

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 回溯法
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 遍历
        fn dfs(nums: &Vec<i32>, from: usize, tmp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            // 终止条件
            if from >= nums.len() {
                return;
            }
            // 遍历下一个
            for i in from..nums.len() {
                tmp.push(nums[i]);
                res.push(tmp.clone());
                dfs(nums, i + 1, tmp, res);
                tmp.pop();
            }
        }

        let mut tmp = Vec::new();
        let mut res = Vec::new();
        res.push(tmp.clone());
        dfs(&nums, 0, &mut tmp, &mut res);

        res
    }
}
// @lc code=end

struct Solution;
