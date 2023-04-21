/*
 * @lc app=leetcode.cn id=47 lang=rust
 *
 * [47] 全排列 II
 *
 * https://leetcode.cn/problems/permutations-ii/description/
 *
 * algorithms
 * Medium (65.46%)
 * Likes:    1344
 * Dislikes: 0
 * Total Accepted:    446.3K
 * Total Submissions: 681.5K
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

// @lc code=start
impl Solution {
    /// ## 解题思路
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        let mut res: Vec<Vec<i32>> = vec![];
        nums.sort();

        /// 回溯
        fn _permute(nums: &[i32], tmp: &[i32], res: &mut Vec<Vec<i32>>) {
            // 终止条件
            if nums.is_empty() {
                res.push(tmp.to_vec());
                return;
            }

            // 遍历
            for (i, &val) in nums.iter().enumerate() {
                if i < 1 || nums[i - 1] != val {
                    let (mut nums_v, mut tmp_v) = (nums.to_vec(), tmp.to_vec());
                    nums_v.remove(i);
                    tmp_v.push(val);
                    //
                    _permute(&nums_v, &tmp_v, res);
                }
            }
        }

        _permute(&nums, &vec![], &mut res);

        res
    }
}
// @lc code=end
