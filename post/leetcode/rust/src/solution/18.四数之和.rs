/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 *
 * https://leetcode-cn.com/problems/4sum/description/
 *
 * algorithms
 * Medium (37.20%)
 * Likes:    397
 * Dislikes: 0
 * Total Accepted:    61K
 * Total Submissions: 163.7K
 * Testcase Example:  '[1,0,-1,0,-2,2]\n0'
 *
 * 给定一个包含 n 个整数的数组 nums 和一个目标值 target，判断 nums 中是否存在四个元素 a，b，c 和 d ，使得 a + b + c
 * + d 的值与 target 相等？找出所有满足条件且不重复的四元组。
 *
 * 注意：
 *
 * 答案中不可以包含重复的四元组。
 *
 * 示例：
 *
 * 给定数组 nums = [1, 0, -1, 0, -2, 2]，和 target = 0。
 *
 * 满足要求的四元组集合为：
 * [
 * ⁠ [-1,  0, 0, 1],
 * ⁠ [-2, -1, 1, 2],
 * ⁠ [-2,  0, 0, 2]
 * ]
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 双指针
    /// 1. 先排序；
    /// 2. 先定前面两个进行遍历；
    /// 3. 剩下两个用双指针从两边向中间遍历；
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let len = nums.len();
        if len < 4 {
            return res;
        }
        let mut nums = nums;
        nums.sort();
        for i in 0..len - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..len - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let (mut l, mut r) = (j + 1, len - 1);
                while l < r {
                    let t = nums[i] + nums[j] + nums[l] + nums[r];
                    if t == target {
                        res.push([nums[i], nums[j], nums[l], nums[r]].to_vec());
                        l += 1;
                        r -= 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1;
                        }
                    } else if t < target {
                        l += 1;
                    } else {
                        r -= 1;
                    }
                }
            }
        }
        res
    }
}
// @lc code=end
