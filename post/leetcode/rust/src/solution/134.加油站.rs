/*
 * @lc app=leetcode.cn id=134 lang=rust
 *
 * [134] 加油站
 *
 * https://leetcode.cn/problems/gas-station/description/
 *
 * algorithms
 * Medium (51.26%)
 * Likes:    1311
 * Dislikes: 0
 * Total Accepted:    263.1K
 * Total Submissions: 515.3K
 * Testcase Example:  '[1,2,3,4,5]\n[3,4,5,1,2]'
 *
 * 在一条环路上有 n 个加油站，其中第 i 个加油站有汽油 gas[i] 升。
 *
 * 你有一辆油箱容量无限的的汽车，从第 i 个加油站开往第 i+1 个加油站需要消耗汽油 cost[i] 升。你从其中的一个加油站出发，开始时油箱为空。
 *
 * 给定两个整数数组 gas 和 cost ，如果你可以按顺序绕环路行驶一周，则返回出发时加油站的编号，否则返回 -1 。如果存在解，则 保证 它是 唯一
 * 的。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: gas = [1,2,3,4,5], cost = [3,4,5,1,2]
 * 输出: 3
 * 解释:
 * 从 3 号加油站(索引为 3 处)出发，可获得 4 升汽油。此时油箱有 = 0 + 4 = 4 升汽油
 * 开往 4 号加油站，此时油箱有 4 - 1 + 5 = 8 升汽油
 * 开往 0 号加油站，此时油箱有 8 - 2 + 1 = 7 升汽油
 * 开往 1 号加油站，此时油箱有 7 - 3 + 2 = 6 升汽油
 * 开往 2 号加油站，此时油箱有 6 - 4 + 3 = 5 升汽油
 * 开往 3 号加油站，你需要消耗 5 升汽油，正好足够你返回到 3 号加油站。
 * 因此，3 可为起始索引。
 *
 * 示例 2:
 *
 *
 * 输入: gas = [2,3,4], cost = [3,4,3]
 * 输出: -1
 * 解释:
 * 你不能从 0 号或 1 号加油站出发，因为没有足够的汽油可以让你行驶到下一个加油站。
 * 我们从 2 号加油站出发，可以获得 4 升汽油。 此时油箱有 = 0 + 4 = 4 升汽油
 * 开往 0 号加油站，此时油箱有 4 - 3 + 2 = 3 升汽油
 * 开往 1 号加油站，此时油箱有 3 - 3 + 3 = 3 升汽油
 * 你无法返回 2 号加油站，因为返程需要消耗 4 升汽油，但是你的油箱只有 3 升汽油。
 * 因此，无论怎样，你都不可能绕环路行驶一周。
 *
 *
 *
 * 提示:
 *
 *
 * gas.length == n
 * cost.length == n
 * 1 <= n <= 10^5
 * 0 <= gas[i], cost[i] <= 10^4
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 贪心法
    /// 1. 设汽车油箱油量为tank, 如果tank<0, 则汽车无法继续行驶;
    /// 2. 汽车到达i站, 加油后, 油箱油量 tank += gas[i];
    /// 3. 汽车离开i站, 刚到第i+1站, 油箱油量 tank -= cost[i];
    /// 4. 设汽车从站点start出发, 顺利到达i站, 但无法到达i+1站, 则此时汽车剩余油量tank < 0.
    ///    而汽车[start..i]之间的任何站点j, 汽车油箱剩余油量tank均>=0, 所以可以抵达i站;
    ///    而当汽车离开i站点,前往i+1站点后,汽车油箱剩余油量tank必定 < 0,
    ///    那么, 表明汽车从[start..i]之间的任意站点出发, 均无法抵达i+1站点,
    ///    所以[start..=i]之间的任何点都不能作为可能的出发点.
    ///    因此, 必须将出发站点start重置为(i+1)%n;
    /// 5. 最后,如果油箱剩余油量tank<0 (tank=(sum(gas[..]) - sum(cost[..]) = sum(gas[..]-cost[..])),
    ///    则汽车无法绕行一周;
    /// 6. 否则, 汽车可以绕行一周, 起始点为剩余油量最少站点的下一个站点;
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut tank = 0; //当前油箱剩余油量
        let mut min_tank = 0; //最低油箱剩余油量
        let mut start = 0; //起始站点
        for i in 0..n {
            tank += gas[i] - cost[i];
            // 油箱剩余油量不够
            if tank < min_tank {
                min_tank = tank; // 更新最低剩余油量
                start = i + 1; // 重置起始站点
            }
        }
        // 如果总剩余油量<0, 则无法绕行一周
        if tank < 0 {
            return -1;
        } else {
            // 否则, 可以绕行一周, 起始点为最低剩余油量下一个站点
            return (start % n) as i32;
        }
    }

    /// 使用迭代器
    pub fn can_complete_circuit2(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        match gas
            .iter()
            .zip(cost.iter())
            .map(|(&g, &c)| g - c)
            .enumerate()
            .fold(
                (0, (0, 0)),
                |(tank, min_start @ (start, tank_min)), (i, g_c)| match tank + g_c {
                    new_tank if new_tank < tank_min => (new_tank, (i as i32 + 1, new_tank)),
                    new_tank => (new_tank, min_start),
                },
            ) {
            (tank, _) if tank < 0 => -1,
            (_, (start, _)) => start % gas.len() as i32,
        }
    }
}
// @lc code=end

struct Solution;
