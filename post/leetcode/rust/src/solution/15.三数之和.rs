/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 *
 * https://leetcode-cn.com/problems/3sum/description/
 *
 * algorithms
 * Medium (25.59%)
 * Likes:    1827
 * Dislikes: 0
 * Total Accepted:    159.8K
 * Total Submissions: 624.2K
 * Testcase Example:  '[-1,0,1,2,-1,-4]'
 *
 * 给定一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0
 * ？找出所有满足条件且不重复的三元组。
 * 
 * 注意：答案中不可以包含重复的三元组。
 * 
 * 
 * 
 * 示例：
 * 
 * 给定数组 nums = [-1, 0, 1, 2, -1, -4]，
 * 
 * 满足要求的三元组集合为：
 * [
 * ⁠ [-1, 0, 1],
 * ⁠ [-1, -1, 2]
 * ]
 * 
 * 
 */

use super::*;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// 1. 对数组进行排序；
    /// 2. 设置3个指针i, l, r；
    /// 3. i从0..size-2进行遍历
    /// 4. l, r为i遍历剩下元素从左右到中间开始遍历；
    /// 
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 3 {
            return vec![]
        }

        let mut nums = nums;
        nums.sort();

        let mut res: Vec<Vec<i32>> = vec![];
        for i in 0..len-2 {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }
            let target = - nums[i];
            let (mut l, mut r) = (i+1, len-1);
            while l < r {
                let s = nums[l] + nums[r]; 
                if s == target {
                    res.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                    while l < r && nums[l] == nums[l-1] {
                        l += 1;
                    }
                    while r > l && nums[r] == nums[r+1] {
                        r -= 1;
                    }
                } else if s < target {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }

        res
    }
}
// @lc code=end

