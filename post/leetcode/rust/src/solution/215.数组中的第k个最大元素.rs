/*
 * @lc app=leetcode.cn id=215 lang=rust
 *
 * [215] 数组中的第K个最大元素
 *
 * https://leetcode.cn/problems/kth-largest-element-in-an-array/description/
 *
 * algorithms
 * Medium (64.03%)
 * Likes:    2092
 * Dislikes: 0
 * Total Accepted:    825.4K
 * Total Submissions: 1.3M
 * Testcase Example:  '[3,2,1,5,6,4]\n2'
 *
 * 给定整数数组 nums 和整数 k，请返回数组中第 k 个最大的元素。
 *
 * 请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。
 *
 * 你必须设计并实现时间复杂度为 O(n) 的算法解决此问题。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: [3,2,1,5,6,4], k = 2
 * 输出: 5
 *
 *
 * 示例 2:
 *
 *
 * 输入: [3,2,3,1,2,4,5,5,6], k = 4
 * 输出: 4
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= k <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 *
 *
 */


// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 改进的快排
    /// 1. 快速排序在选择分割点时,分割点会将nums分为2部分,一部分都比基准值小,另一部分大;
    /// 2. 如果分割后基准值的index==k, 则得到结果;
    /// 3. 否则如果<k, 则结果在后一部分,继续新的划分;
    /// 4. 如果>k, 则结果在前一部分
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        fn helper(nums: &mut [i32], k: i32) -> i32 {
            let base = nums[0];
            let (mut l, mut r) = (0, nums.len() - 1);
            while l < r {
                // 从右向左，寻找比基准值小的数的index
                while r > l && nums[r] >= base {
                    r -= 1;
                }
                // 右边找到比基准值小的数
                if r > l && nums[r] < base {
                    nums.swap(l, r); //交换这两个数的位置
                    l += 1;
                }
                // 从左向右，查找比基准值大的数的index
                while l < r && nums[l] <= base {
                    l += 1;
                }
                // 左边找到比基准值大的数
                if l < r && nums[l] > base {
                    nums.swap(l, r);
                    r -= 1;
                }
            }

            if l == k as usize {
                return nums[l];
            } else {
                if l < k as usize {
                    return helper(&mut nums[(l + 1)..], k - (l as i32 + 1));
                } else {
                    return helper(&mut nums[..l], k);
                }
            }
        }

        if nums.len() == 1 {
            return nums[0];
        }

        let k = nums.len() as i32 - k;
        let mut nums = nums;
        helper(&mut nums[..], k)
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4].to_vec(), 2),
            5
        );
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6].to_vec(), 4),
            4
        );
    }
}
