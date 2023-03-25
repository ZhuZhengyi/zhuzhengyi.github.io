/*
 * @lc app=leetcode.cn id=31 lang=rust
 *
 * [31] 下一个排列
 *
 * https://leetcode-cn.com/problems/next-permutation/description/
 *
 * algorithms
 * Medium (37.32%)
 * Likes:    1391
 * Dislikes: 0
 * Total Accepted:    224.9K
 * Total Submissions: 603.2K
 * Testcase Example:  '[1,2,3]'
 *
 * 实现获取 下一个排列 的函数，算法需要将给定数字序列重新排列成字典序中下一个更大的排列（即，组合出下一个更大的整数）。
 *
 * 如果不存在下一个更大的排列，则将数字重新排列成最小的排列（即升序排列）。
 *
 * 必须 原地 修改，只允许使用额外常数空间。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,2,3]
 * 输出：[1,3,2]
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [3,2,1]
 * 输出：[1,2,3]
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [1,1,5]
 * 输出：[1,5,1]
 *
 *
 * 示例 4：
 *
 *
 * 输入：nums = [1]
 * 输出：[1]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 100
 * 0 <= nums[i] <= 100
 *
 *
 */


// @lc code=start
impl Solution {
    /// ## 解题思路：
    /// - 交换
    /// 1. 从右往左，找到第一个nums[i-1]<nums[i]的i;
    /// 2. 然后再从[i+1..]中查找比nums[i-1]大的最小数nums[j];
    /// 3. 交换nums[i-1],nums[j];
    /// 4. 交换后，重新按从小到大排列nums[i..]；
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        if len < 2 {
            return;
        }

        let mut resort_i = 0;  //要重新排序的部分起始index
        // 从右向左遍历
        for i in (1..len).rev() {
            // 找到第一个nums[i-1]<nums[i]
            if nums[i - 1] < nums[i] {
                // 在[i..]中, 查找比nums[i-1]大的最小数
                let bigger = nums[i..]
                    .iter()
                    .enumerate()
                    .filter(|(_, &n)| n > nums[i - 1])
                    .min_by(|(_, &n1), (_, &n2)| n1.cmp(&n2))
                    .map(|(j, _)| j + i)
                    .unwrap_or(i);
                // 交换nums[i-1], nums[bigger]
                nums.swap(i - 1, bigger);
                resort_i = i; //标记交换点
                break;
            }
        }

        //重新排序nums[i..]
        nums[resort_i..].sort();
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }
}
