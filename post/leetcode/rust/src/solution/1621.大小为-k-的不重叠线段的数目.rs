/*
 * @lc app=leetcode.cn id=1621 lang=rust
 *
 * [1621] 大小为 K 的不重叠线段的数目
 *
 * https://leetcode.cn/problems/number-of-sets-of-k-non-overlapping-line-segments/description/
 *
 * algorithms
 * Medium (47.64%)
 * Likes:    55
 * Dislikes: 0
 * Total Accepted:    3K
 * Total Submissions: 6.3K
 * Testcase Example:  '4\n2'
 *
 * 给你一维空间的 n 个点，其中第 i 个点（编号从 0 到 n-1）位于 x = i 处，请你找到 恰好 k 个不重叠
 * 线段且每个线段至少覆盖两个点的方案数。线段的两个端点必须都是 整数坐标 。这 k 个线段不需要全部覆盖全部 n 个点，且它们的端点 可以 重合。
 * 
 * 请你返回 k 个不重叠线段的方案数。由于答案可能很大，请将结果对 10^9 + 7 取余 后返回。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 4, k = 2
 * 输出：5
 * 解释：
 * 如图所示，两个线段分别用红色和蓝色标出。
 * 上图展示了 5 种不同的方案
 * {(0,2),(2,3)}，{(0,1),(1,3)}，{(0,1),(2,3)}，{(1,2),(2,3)}，{(0,1),(1,2)} 。
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 3, k = 1
 * 输出：3
 * 解释：总共有 3 种不同的方案 {(0,1)}, {(0,2)}, {(1,2)} 。
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：n = 30, k = 7
 * 输出：796297179
 * 解释：画 7 条线段的总方案数为 3796297200 种。将这个数对 10^9 + 7 取余得到 796297179 。
 * 
 * 
 * 示例 4：
 * 
 * 
 * 输入：n = 5, k = 3
 * 输出：7
 * 
 * 
 * 示例 5：
 * 
 * 
 * 输入：n = 3, k = 2
 * 输出：1
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 2 
 * 1 
 * 
 * 
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 dp[i][j]: i个点, j条线段的总方案数;
    /// 2. 如果第j条线段右端点不是i, 则去掉第i个点,组成j条线段的方案数不变, 
    ///    则有: dp[i][j] = dp[i-1][j];
    /// 3. 如果第j条线段右端点是i, 则去掉第j条线段, 所有由前面点组成j-1条线段的总方案数为: 
    ///    dp[i-1][j-1] + dp[i-2][j-1] + .. dp[0][j-1]), 记为sum_dp[i-1][j-1]
    ///    则 sum_dp[i][j] = sum_dp[i-1][j] + dp[i][j]
    /// 4. 所以得到递推关系: 
    ///       dp[i][j] = dp[i-1][j] + sum_dp[i-1][j-1]
    ///       sum_dp[i][j] = sum_dp[i-1][j] + dp[i][j]  
    /// 5. 初始条件: 
    ///       dp[i][1] = dp[i-1][1] + (i - 1)   (i=1..=n)
    ///                      |           |-- 线段右端点为i, 因为总共只有一条线段,所以前面部分不是线段, 
    ///                      |               以i为右端点的线段总共有i-1条
    ///                      |-- 线段右端点不是i, 第i-1个端点组成1条线段的方案数
    ///       sum_dp[i][1] = sum_dp[i-1][1] + dp[i][1]  
    /// 6. 终止条件: dp[n][k]
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        let (n, k) = (n as usize, k as usize);
        const MOD: i32 = 1000_000_000 + 7;
        let mut dp = vec![vec![0_i32; k+1]; n+1];
        let mut sum_dp = vec![vec![0_i32; k+1]; n+1];
        // 初始化, k=1时, 
        for i in 1..=n {
            dp[i][1] = (dp[i-1][1] + ( i as i32 - 1 )) % MOD ;
            sum_dp[i][1] = (sum_dp[i-1][1] + dp[i][1] ) % MOD;
        }

        for i in 1..=n {
            for j in 2..=k {
                dp[i][j] = (dp[i-1][j] + sum_dp[i-1][j-1] ) % MOD;
                sum_dp[i][j] = (sum_dp[i-1][j] + dp[i][j]) % MOD;
            }
        }

        dp[n][k]
    }
}
// @lc code=end

struct Solution;

