/*
 * @lc app=leetcode.cn id=1909 lang=rust
 *
 * [1909] 删除一个元素使数组严格递增
 *
 * https://leetcode.cn/problems/remove-one-element-to-make-the-array-strictly-increasing/description/
 *
 * algorithms
 * Easy (30.00%)
 * Likes:    37
 * Dislikes: 0
 * Total Accepted:    10.5K
 * Total Submissions: 35K
 * Testcase Example:  '[1,2,10,5,7]'
 *
 * 给你一个下标从 0 开始的整数数组 nums ，如果 恰好 删除 一个 元素后，数组 严格递增 ，那么请你返回 true ，否则返回 false
 * 。如果数组本身已经是严格递增的，请你也返回 true 。
 * 
 * 数组 nums 是 严格递增 的定义为：对于任意下标的 1 <= i < nums.length 都满足 nums[i - 1] < nums[i]
 * 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 输入：nums = [1,2,10,5,7]
 * 输出：true
 * 解释：从 nums 中删除下标 2 处的 10 ，得到 [1,2,5,7] 。
 * [1,2,5,7] 是严格递增的，所以返回 true 。
 * 
 * 
 * 示例 2：
 * 
 * 输入：nums = [2,3,1,2]
 * 输出：false
 * 解释：
 * [3,1,2] 是删除下标 0 处元素后得到的结果。
 * [2,1,2] 是删除下标 1 处元素后得到的结果。
 * [2,3,2] 是删除下标 2 处元素后得到的结果。
 * [2,3,1] 是删除下标 3 处元素后得到的结果。
 * 没有任何结果数组是严格递增的，所以返回 false 。
 * 
 * 示例 3：
 * 
 * 输入：nums = [1,1,1]
 * 输出：false
 * 解释：删除任意元素后的结果都是 [1,1] 。
 * [1,1] 不是严格递增的，所以返回 false 。
 * 
 * 
 * 示例 4：
 * 
 * 输入：nums = [1,2,3]
 * 输出：true
 * 解释：[1,2,3] 已经是严格递增的，所以返回 true 。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 2 <= nums.length <= 1000
 * 1 <= nums[i] <= 1000
 * 
 * 
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 爆力枚举
    /// 1. 从左至右依次遍历nums, 找到nums[i]>=nums[i+1];
    /// 2. 如果之前已经删除过,则返回false;
    /// 3. 否则第一次找到递减元素, 必须删除nums[i], nums[i+1]中的一个;
    /// 4. 如果i==0，说明nums[i]在开头，删除nums[i]。
    /// 5. 如果i+1==n-1, 说明小数在末尾, 删除nums[i+1];
    /// 6. 如果nums[i+1] > nums[i-1]，说明nums[i-1]<<nums[i]>nums[i+1], nums[i]为局部最大值，删除nums[i]。
    /// 7. 如果nums[i] < nums[i + 2]，说明nums[i]>nums[i+1]<<nums[i+2], nums[i+1]为局部最小值，删除nums[i+1]。
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let mut deleted = false;
        for i in 0..(nums.len()-1) {
            if nums[i] >= nums[i+1] {   // 发现递减元素
                if deleted {            // 如果已经删除过,则无法生成递增序列
                    return false;
                } else if i==0 || i+1 == nums.len()-1 || nums[i+1]>nums[i-1] || nums[i]<nums[i+2] {
                    deleted = true;
                } else {
                    return false;
                }
            }
        }

        return true;
    }
}
// @lc code=end

struct Solution;
