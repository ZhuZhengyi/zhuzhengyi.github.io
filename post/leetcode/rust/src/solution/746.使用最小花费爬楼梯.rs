/*
 * @lc app=leetcode.cn id=746 lang=rust
 *
 * [746] 使用最小花费爬楼梯
 *
 * https://leetcode-cn.com/problems/min-cost-climbing-stairs/description/
 *
 * algorithms
 * Easy (58.96%)
 * Likes:    691
 * Dislikes: 0
 * Total Accepted:    142.3K
 * Total Submissions: 240.6K
 * Testcase Example:  '[10,15,20]'
 *
 * 数组的每个下标作为一个阶梯，第 i 个阶梯对应着一个非负数的体力花费值 cost[i]（下标从 0 开始）。
 * 
 * 每当你爬上一个阶梯你都要花费对应的体力值，一旦支付了相应的体力值，你就可以选择向上爬一个阶梯或者爬两个阶梯。
 * 
 * 请你找出达到楼层顶部的最低花费。在开始时，你可以选择从下标为 0 或 1 的元素作为初始阶梯。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：cost = [10, 15, 20]
 * 输出：15
 * 解释：最低花费是从 cost[1] 开始，然后走两步即可到阶梯顶，一共花费 15 。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：cost = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1]
 * 输出：6
 * 解释：最低花费方式是从 cost[0] 开始，逐个经过那些 1 ，跳过 cost[3] ，一共花费 6 。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * cost 的长度范围是 [2, 1000]。
 * cost[i] 将会是一个整型数据，范围为 [0, 999] 。
 * 
 * 
 */

use super::*;

// @lc code=start
use std::cmp;

impl Solution {
    /// 解题思路：
    ///     本题的题目描述存在一定的歧义，
    /// 注意：
    /// 1. cost[i]代表从第i级台阶往上跨时，需要消耗的体力
    /// 2. 在每级台阶上，消耗该台阶对应的体力cost[i]后，可以选择跨1个或2个台阶；
    /// 3. 最后一个还需要跨一步，因为最后一个台阶cost[i]需要消耗体力
    /// 示意图：
    /// 
    ///                    _top_
    ///                _3_| 0
    ///            _2_|20
    ///        _1_|15
    ///  _____|10
    ///   0
    /// 
    /// 登上第i级台阶有两种方式：
    /// 1. 先登上第i-1级台阶，再跨1个台阶, 要消耗cost[i-1]点体力；
    /// 2. 先登上第i-2级台阶，再跨2个台阶，要消耗cost[i-2]点体力；
    /// 其最小花费f(i)为两种花费方式中最小的那个，递推公式如下：
    ///     f(i) = min(f(i-1)+cost[i-1], f(i-2) + cost [i-2]) 
    /// 初始条件：
    ///     f(0) = 0  ;地面往上跨需要消耗的体力为0
    ///     f(1) = 0  ;直接跨2步到顶上
    /// 最终: f(len(cost))
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let l = cost.len();
        match l {
            0 | 1 => {
                return 0;
            }
            _ => {}
        }

        let mut f0 = 0;
        let mut f1 = 0;
        let mut f2 = 0; 
        for i in 2..=l {
            f2 = cmp::min(f1+cost[i-1], f0+cost[i-2]);
            f0 = f1;
            f1 = f2;
        }

        f2
    }
}
// @lc code=end

