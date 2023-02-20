/*
 * @lc app=leetcode.cn id=300 lang=rust
 *
 * [300] 最长递增子序列
 *
 * https://leetcode-cn.com/problems/longest-increasing-subsequence/description/
 *
 * algorithms
 * Medium (51.75%)
 * Likes:    1963
 * Dislikes: 0
 * Total Accepted:    371.6K
 * Total Submissions: 717.9K
 * Testcase Example:  '[10,9,2,5,3,7,101,18]'
 *
 * 给你一个整数数组 nums ，找到其中最长严格递增子序列的长度。
 *
 * 子序列是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余元素的顺序。例如，[3,6,2,7] 是数组 [0,3,1,6,2,2,7]
 * 的子序列。
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [10,9,2,5,3,7,101,18]
 * 输出：4
 * 解释：最长递增子序列是 [2,3,7,101]，因此长度为 4 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [0,1,0,3,2,3]
 * 输出：4
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [7,7,7,7,7,7,7]
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * -10^4
 *
 *
 *
 *
 * 进阶：
 *
 *
 * 你可以设计时间复杂度为 O(n^2) 的解决方案吗？
 * 你能将算法的时间复杂度降低到 O(n log(n)) 吗?
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// 假设nums[0:n]的最长递增子序列为lis[],
    /// 则nums[0:n+1]的lis有一些情况：
    /// a. nums[n] > lis[-1] => lis.push(nums[n])
    /// b. nums[n] > lis[i-1] => lis[i] = nums[n] (i为第一个大于nums[n]的数)
    ///
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() < 1 {
            return 0;
        }
        let mut lis = vec![nums[0]]; //最长递增子序列
        nums[1..].iter().for_each(|n| {
            //顺序遍历序列
            if n > lis.last().unwrap() {
                // 如果当前元素大于lis最后元素
                lis.push(*n); //将当前元素追加到lis尾部
            } else {
                // 否则，lis中的一定存在元素大于当前元素
                // 查找lis中第一个大于当前元素值的元素
                // 由于lis为有序数组，可以用二分法加速查找
                let (mut l, mut r) = (0, lis.len());
                //let mut i = 0;
                while l < r {
                    let i = l + (r - l) / 2;
                    if lis[i] >= *n && (i == 0 || lis[i - 1] < *n) {
                        lis[i] = *n;
                        break;
                    } else if lis[i] < *n {
                        l = i + 1;
                    } else {
                        r = i;
                    }
                }
            }
        });
        return lis.len() as i32;
    }
}
// @lc code=end
