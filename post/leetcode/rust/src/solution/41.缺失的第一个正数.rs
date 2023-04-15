/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] 缺失的第一个正数
 *
 * https://leetcode.cn/problems/first-missing-positive/description/
 *
 * algorithms
 * Hard (42.92%)
 * Likes:    1789
 * Dislikes: 0
 * Total Accepted:    285.3K
 * Total Submissions: 664.5K
 * Testcase Example:  '[1,2,0]'
 *
 * 给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。
 * 请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,2,0]
 * 输出：3
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [3,4,-1,1]
 * 输出：2
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [7,8,9,11,12]
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * -2^31
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// 1. 数组中的数字如果按1,2,3,..排列,则当第i位不位i+1时,第一个缺失的正整数即为i+1;
    /// 2. 依次遍历各个元素,将在[1, n]范围内的元素调整到[0..n-1]的位置;
    /// 3. 最后再自左至右检查, 查找第一个nums[i]!=i+1的i;
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        // 依次调整各个元素至对应的位置
        for i in 0..n {
            let mut ni = nums[i];
            while nums[i] > 0
                && nums[i] <= n as i32
                && nums[i] != (i + 1) as i32
                && nums[(nums[i] - 1) as usize] != nums[i]
            {
                nums.swap(i, (ni - 1) as usize);
                ni = nums[i];
            }
        }
        // 自左至右查找第一个nums[i] != i+1
        for i in 0..n {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }
        return (n + 1) as i32;
    }
}
// @lc code=end
struct Solution;
