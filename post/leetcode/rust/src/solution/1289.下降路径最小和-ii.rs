/*
 * @lc app=leetcode.cn id=1289 lang=rust
 *
 * [1289] 下降路径最小和  II
 *
 * https://leetcode-cn.com/problems/minimum-falling-path-sum-ii/description/
 *
 * algorithms
 * Hard (60.63%)
 * Likes:    25
 * Dislikes: 0
 * Total Accepted:    2.8K
 * Total Submissions: 4.6K
 * Testcase Example:  '[[1,2,3],[4,5,6],[7,8,9]]'
 *
 * 给你一个整数方阵 arr ，定义「非零偏移下降路径」为：从 arr 数组中的每一行选择一个数字，且按顺序选出来的数字中，相邻数字不在原数组的同一列。
 *
 * 请你返回非零偏移下降路径数字和的最小值。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：arr = [[1,2,3],[4,5,6],[7,8,9]]
 * 输出：13
 * 解释：
 * 所有非零偏移下降路径包括：
 * [1,5,9], [1,5,7], [1,6,7], [1,6,8],
 * [2,4,8], [2,4,9], [2,6,7], [2,6,8],
 * [3,4,8], [3,4,9], [3,5,7], [3,5,9]
 * 下降路径中数字和最小的是 [1,5,7] ，所以答案是 13 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= arr.length == arr[i].length <= 200
 * -99 <= arr[i][j] <= 99
 *
 *
 */

pub struct Solution{};

// @lc code=start
impl Solution {
    pub fn min_falling_path_sum(arr: Vec<Vec<i32>>) -> i32 {
        //初始化dp矩阵
        let mut dp: Vec<Vec<i32>> = vec![]; //
        for i in 0..arr.len() {
            dp.push(vec![]);
            for j in 0..arr.len() {
                if i == 0 {
                    dp[i].push(arr[i][j]);
                } else {
                    dp[i].push(0);
                }
            }
        }

        //dp[i][j] = arr[i][j] + 上一步不是正上方的最小dp
        for i in 1..arr.len() {
            for j in 0..arr.len() {
                dp[i][j] = arr[i][j] + dp[i - 1]
                    .iter()
                    .enumerate()
                    .filter(|(id, _)| *id != j)
                    .map(|(_, v)| v)
                    .min()
                    .unwrap()
                    .clone()
            }
        }

        dp[dp.len() - 1].iter().min().unwrap().clone()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            13,
            Solution::min_falling_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
    }
}
