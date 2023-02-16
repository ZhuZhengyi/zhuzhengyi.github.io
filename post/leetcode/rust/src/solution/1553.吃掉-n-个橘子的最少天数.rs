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

struct Solution;

// @lc code=start

use std::collections::HashMap;
use std::cmp;

impl Solution {
    /// 解题思路：
    /// 
    /// 当剩下i个橘子时，可选择的吃法有如下几种：
    /// a. 吃一个，剩下[i-1]个橘子;
    /// b. 先吃i%2个(一个个吃)，再一次吃剩下的一半，剩下[i/2]个橘子；
    /// c. 先吃i%3个(一个个吃)，再一次次剩下的2/3, 剩下[i/3]个橘子，
    /// 
    /// 最少天数(记为f(n))为上面三种吃法中最小的:
    ///     f(n) = min(1+f(n-1), n%2+1+f(n/2), n%3+1+f(n/3))
    /// 显然,在n>2时，f(n-1) 永远要大于(n%2+f(n/2))和(n%3+f(n/3)),可简化为：
    ///     f(n) = min(n%2+1+f(n/2), n%3+1+f(n/3))
    /// 
    /// 初始条件：
    ///     f(0) = 0;
    ///     f(1) = 1;
    ///     f(2) = 2;
    /// 最后结果： f(n)
    /// 
    /// 实现方法：
    ///     1. 自顶向下递归求解f(n)；为加快速度，可使用hashmap缓存中间结果
    ///     2. 自底向上迭代求取；
    /// 
    /// 递归+hashmap加速
    ///     时间复杂度：O(log(n))
    ///     空间复杂度：O(n)
    pub fn min_days(n: i32) -> i32 {
        let mut record = HashMap::new();
        
        fn eat(n: i32, record: &mut HashMap<i32, i32>) -> i32 {
            match n {
                0 | 1 => n,
                i if record.contains_key(&i) => *record.get(&i).unwrap(),
                _ => {
                    let res = cmp::min(eat(n/2, record) + n%2, eat(n/3, record) + n%3) + 1;
                    record.insert(n, res);
                    res
                }
            }
        }

        eat(n, &mut record)
    }

    // 自底向上迭代 
    //     时间复杂度：O(n)
    //     空间复杂度：O(n)
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


