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

struct Solution;

// @lc code=start
impl Solution {
    /// 思路：
    /// 1. 从右往左，找到第一个小于其右边的数，将两数交换；
    /// 2. 交换后，将该数右侧的子序列重新从左到右增序排列；
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let l = nums.len();
        if l < 2 {
            return
        }

        let mut i = l-1;
        while i > 0 {
            // 从右向左，找到第一个小于其右侧元素的元素
            if nums[i-1] < nums[i] {
                let mut bigger = i;
                for j in i+1..l {
                    //查找其右侧只比该元素大一点的元素
                    if nums[j] > nums[i-1] && nums[j] < nums[bigger] {
                        bigger = j;
                    }
                }
                nums.swap(i-1, bigger);
                break;
            }
            i-=1;
        }

        // 
        nums[i..].sort();
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums = vec![1,2,3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1,3,2]);
    }
}
