/*
 * @lc app=leetcode.cn id=877 lang=rust
 *
 * [877] 石子游戏
 *
 * https://leetcode.cn/problems/stone-game/description/
 *
 * algorithms
 * Medium (76.30%)
 * Likes:    455
 * Dislikes: 0
 * Total Accepted:    86.5K
 * Total Submissions: 113.4K
 * Testcase Example:  '[5,3,4,5]'
 *
 * Alice 和 Bob 用几堆石子在做游戏。一共有偶数堆石子，排成一行；每堆都有 正 整数颗石子，数目为 piles[i] 。
 *
 * 游戏以谁手中的石子最多来决出胜负。石子的 总数 是 奇数 ，所以没有平局。
 *
 * Alice 和 Bob 轮流进行，Alice 先开始 。 每回合，玩家从行的 开始 或 结束 处取走整堆石头。
 * 这种情况一直持续到没有更多的石子堆为止，此时手中 石子最多 的玩家 获胜 。
 *
 * 假设 Alice 和 Bob 都发挥出最佳水平，当 Alice 赢得比赛时返回 true ，当 Bob 赢得比赛时返回 false 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：piles = [5,3,4,5]
 * 输出：true
 * 解释：
 * Alice 先开始，只能拿前 5 颗或后 5 颗石子 。
 * 假设他取了前 5 颗，这一行就变成了 [3,4,5] 。
 * 如果 Bob 拿走前 3 颗，那么剩下的是 [4,5]，Alice 拿走后 5 颗赢得 10 分。
 * 如果 Bob 拿走后 5 颗，那么剩下的是 [3,4]，Alice 拿走后 4 颗赢得 9 分。
 * 这表明，取前 5 颗石子对 Alice 来说是一个胜利的举动，所以返回 true 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：piles = [3,7,2,3]
 * 输出：true
 *
 *
 *
 *
 * 提示：
 *
 *
 * 2 <= piles.length <= 500
 * piles.length 是 偶数
 * 1 <= piles[i] <= 500
 * sum(piles[i]) 是 奇数
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// * 动态规划
    /// 1. 设dp[i][j]表示剩下piles[i..j]石子时，当前玩家所得石子的最大差；
    /// 2. i>j时, 无意义，dp[i][j] = 0;
    /// 3. i==j时, 只剩下这个,dp[i][j] = piles[i];
    /// 4. i<j时, 当前玩家有2种取法，分别取piles[i], piles[j], 则剩下的被对家选:
    ///     dp[i][j] = max(piles[i] - dp[i+1][j], piles[j] - dp[i][j-1])
    /// 5. return dp[0][n-1] > 0
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();
        let mut dp = vec![vec![0_i32; n]; n];

        for i in (0..n - 1).rev() {
            for j in i..n {
                dp[i][j] = if i == j {
                    piles[i]
                } else {
                    std::cmp::max(piles[i] - dp[i + 1][j], piles[j] - dp[i][j - 1])
                };
            }
        }

        dp[0][n - 1] > 0
    }
}
// @lc code=end
