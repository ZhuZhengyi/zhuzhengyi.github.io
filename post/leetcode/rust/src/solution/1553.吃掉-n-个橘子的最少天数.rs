/*
 * @lc app=leetcode.cn id=1553 lang=rust
 *
 * [1553] 吃掉 N 个橘子的最少天数
 *
 * https://leetcode-cn.com/problems/minimum-number-of-days-to-eat-n-oranges/description/
 *
 * algorithms
 * Hard (26.35%)
 * Likes:    96
 * Dislikes: 0
 * Total Accepted:    7.8K
 * Total Submissions: 26.3K
 * Testcase Example:  '10'
 *
 * 厨房里总共有 n 个橘子，你决定每一天选择如下方式之一吃这些橘子：
 * 
 * 
 * 吃掉一个橘子。
 * 如果剩余橘子数 n 能被 2 整除，那么你可以吃掉 n/2 个橘子。
 * 如果剩余橘子数 n 能被 3 整除，那么你可以吃掉 2*(n/3) 个橘子。
 * 
 * 
 * 每天你只能从以上 3 种方案中选择一种方案。
 * 
 * 请你返回吃掉所有 n 个橘子的最少天数。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 输入：n = 10
 * 输出：4
 * 解释：你总共有 10 个橘子。
 * 第 1 天：吃 1 个橘子，剩余橘子数 10 - 1 = 9。
 * 第 2 天：吃 6 个橘子，剩余橘子数 9 - 2*(9/3) = 9 - 6 = 3。（9 可以被 3 整除）
 * 第 3 天：吃 2 个橘子，剩余橘子数 3 - 2*(3/3) = 3 - 2 = 1。
 * 第 4 天：吃掉最后 1 个橘子，剩余橘子数 1 - 1 = 0。
 * 你需要至少 4 天吃掉 10 个橘子。
 * 
 * 
 * 示例 2：
 * 
 * 输入：n = 6
 * 输出：3
 * 解释：你总共有 6 个橘子。
 * 第 1 天：吃 3 个橘子，剩余橘子数 6 - 6/2 = 6 - 3 = 3。（6 可以被 2 整除）
 * 第 2 天：吃 2 个橘子，剩余橘子数 3 - 2*(3/3) = 3 - 2 = 1。（3 可以被 3 整除）
 * 第 3 天：吃掉剩余 1 个橘子，剩余橘子数 1 - 1 = 0。
 * 你至少需要 3 天吃掉 6 个橘子。
 * 
 * 
 * 示例 3：
 * 
 * 输入：n = 1
 * 输出：1
 * 
 * 
 * 示例 4：
 * 
 * 输入：n = 56
 * 输出：6
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= n <= 2*10^9
 * 
 * 
 */

// @lc code=start

use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn min_days(n: i32) -> i32 {
        let mut record = HashMap::new();

        Self::eat(n, &mut record)
    }

    pub fn eat(n: i32, record: &mut HashMap<i32, i32>) -> i32 {
        match n {
            0 | 1 => n,
            i if record.contains_key(&i) => *record.get(&i).unwrap(),
            _ => {
                let res = cmp::min(Self::eat(n/2, record) + n%2, Self::eat(n/3, record) + n%3) + 1;
                record.insert(n, res);
                res
            }
        }
    }
    // pub fn min_days(n: i32) -> i32 {
    //     if n < 2 {
    //         return n;
    //     }
    //     let mut res: Vec<i32> = vec![0 , 1];
    //     for i in 2..n+1 {
    //         res.push(cmp::min(res[(i/2) as usize] + i%2, res[(i/3) as usize] + i%3) + 1);
    //     }

    //     return res[n as usize];
    // }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(Solution::min_days(10), 4);
    }
}


