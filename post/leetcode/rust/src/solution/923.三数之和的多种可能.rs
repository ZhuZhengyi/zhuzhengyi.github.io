/*
 * @lc app=leetcode.cn id=923 lang=rust
 *
 * [923] 三数之和的多种可能
 *
 * https://leetcode.cn/problems/3sum-with-multiplicity/description/
 *
 * algorithms
 * Medium (36.28%)
 * Likes:    111
 * Dislikes: 0
 * Total Accepted:    8.9K
 * Total Submissions: 24.6K
 * Testcase Example:  '[1,1,2,2,3,3,4,4,5,5]\n8'
 *
 * 给定一个整数数组 arr ，以及一个整数 target 作为目标值，返回满足 i < j < k 且 arr[i] + arr[j] + arr[k]
 * == target 的元组 i, j, k 的数量。
 * 
 * 由于结果会非常大，请返回 10^9 + 7 的模。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：arr = [1,1,2,2,3,3,4,4,5,5], target = 8
 * 输出：20
 * 解释：
 * 按值枚举(arr[i], arr[j], arr[k])：
 * (1, 2, 5) 出现 8 次；
 * (1, 3, 4) 出现 8 次；
 * (2, 2, 4) 出现 2 次；
 * (2, 3, 3) 出现 2 次。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：arr = [1,1,2,2,2,2], target = 5
 * 输出：12
 * 解释：
 * arr[i] = 1, arr[j] = arr[k] = 2 出现 12 次：
 * 我们从 [1,1] 中选择一个 1，有 2 种情况，
 * 从 [2,2,2,2] 中选出两个 2，有 6 种情况。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 3 <= arr.length <= 3000
 * 0 <= arr[i] <= 100
 * 0 <= target <= 300
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {

        

    }
}
// @lc code=end

