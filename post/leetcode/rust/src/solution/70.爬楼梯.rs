/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 *
 * https://leetcode-cn.com/problems/climbing-stairs/description/
 *
 * algorithms
 * Easy (47.88%)
 * Likes:    856
 * Dislikes: 0
 * Total Accepted:    139.1K
 * Total Submissions: 290.2K
 * Testcase Example:  '2'
 *
 * 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
 * 
 * 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
 * 
 * 注意：给定 n 是一个正整数。
 * 
 * 示例 1：
 * 
 * 输入： 2
 * 输出： 2
 * 解释： 有两种方法可以爬到楼顶。
 * 1.  1 阶 + 1 阶
 * 2.  2 阶
 * 
 * 示例 2：
 * 
 * 输入： 3
 * 输出： 3
 * 解释： 有三种方法可以爬到楼顶。
 * 1.  1 阶 + 1 阶 + 1 阶
 * 2.  1 阶 + 2 阶
 * 3.  2 阶 + 1 阶
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1|2 => n,
            _ => {
                let (mut prev, mut current) = (1, 2);
                for _ in 3..n+1  {
                    let old_curr = current;
                    current = prev + current;
                    prev = old_curr;
                }

                current
            }
        }
    }
}
// @lc code=end

