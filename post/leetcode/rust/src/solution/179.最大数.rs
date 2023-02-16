/*
 * @lc app=leetcode.cn id=179 lang=rust
 *
 * [179] 最大数
 *
 * https://leetcode-cn.com/problems/largest-number/description/
 *
 * algorithms
 * Medium (40.96%)
 * Likes:    846
 * Dislikes: 0
 * Total Accepted:    130.2K
 * Total Submissions: 317.8K
 * Testcase Example:  '[10,2]'
 *
 * 给定一组非负整数 nums，重新排列每个数的顺序（每个数不可拆分）使之组成一个最大的整数。
 *
 * 注意：输出结果可能非常大，所以你需要返回一个字符串而不是整数。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [10,2]
 * 输出："210"
 *
 * 示例 2：
 *
 *
 * 输入：nums = [3,30,34,5,9]
 * 输出："9534330"
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [1]
 * 输出："1"
 *
 *
 * 示例 4：
 *
 *
 * 输入：nums = [10]
 * 输出："10"
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * 0
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        // nums.into_iter()
        //     .map(|n| n.to_string())
        //     .collect()
        //     .sort_by(|&x, &y| x.chars().nth(0))
        //     .iter()
        //     .fold(String::new(), |mut s, t| {
        //         s.push_str(&t);
        //         s
        //     })
        todo!()
    }
}
// @lc code=end
