/*
 * @lc app=leetcode.cn id=97 lang=rust
 *
 * [97] 交错字符串
 *
 * https://leetcode.cn/problems/interleaving-string/description/
 *
 * algorithms
 * Medium (44.67%)
 * Likes:    882
 * Dislikes: 0
 * Total Accepted:    118.4K
 * Total Submissions: 265.3K
 * Testcase Example:  '"aabcc"\n"dbbca"\n"aadbbcbcac"'
 *
 * 给定三个字符串 s1、s2、s3，请你帮忙验证 s3 是否是由 s1 和 s2 交错 组成的。
 *
 * 两个字符串 s 和 t 交错 的定义与过程如下，其中每个字符串都会被分割成若干 非空 子字符串：
 *
 *
 * s = s1 + s2 + ... + sn
 * t = t1 + t2 + ... + tm
 * |n - m| <= 1
 * 交错 是 s1 + t1 + s2 + t2 + s3 + t3 + ... 或者 t1 + s1 + t2 + s2 + t3 + s3 +
 * ...
 *
 *
 * 注意：a + b 意味着字符串 a 和 b 连接。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
 * 输出：true
 *
 *
 * 示例 2：
 *
 *
 * 输入：s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
 * 输出：false
 *
 *
 * 示例 3：
 *
 *
 * 输入：s1 = "", s2 = "", s3 = ""
 * 输出：true
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0 <= s1.length, s2.length <= 100
 * 0 <= s3.length <= 200
 * s1、s2、和 s3 都由小写英文字母组成
 *
 *
 *
 *
 * 进阶：您能否仅使用 O(s2.length) 额外的内存空间来解决它?
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 dp[i][j]: 表示 s1[..i],s2[..j]是否为s3[..(i+j)]的交错字符串
    ///    则 题目等价于: dp[s1.len()][s2.len()] 是否为 true
    /// 3. 递推关系: dp[i][j] = (s1[i-1] == s3[i+j-1] && dp[i-1][j])
    ///                       || (s2[j-1] == s3[i+j-1] && dp[i][j-1])
    /// 4. 初始条件: dp[0][0] = true, s1,s2, s3均为空
    ///             dp[i][0] = (s1[i-1] == s3[i-1] && dp[i-1][0]) ( 1<i<=s1.len() )
    ///             dp[0][j] = (s2[j-1] == s3[j-1] && dp[0][j-1]) ( 1<j<=s2.len() )
    /// 5. 优化:
    ///    a. 由递推公式可知, dp[i][j] 仅和 dp[i-1][j], dp[i][j-1] 相关,
    ///       故可使用旋转数组来取代 dp[m][n] 二维矩阵, 降低内存开销;
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        // 存在""的情况
        if (s1 == "" && s2 == s3) || (s2 == "" && s1 == s3) {
            return true;
        }

        let (l1, l2) = (s1.len(), s2.len());
        // 排除长度不对的
        if s3.len() != l1 + l2 {
            return false;
        }

        let mut dp = vec![vec![false; l2 + 1]; l1 + 1];
        dp[0][0] = true;
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        (0..l1).for_each(|i| dp[i + 1][0] = s1[i] == s3[i] && dp[i][0]);
        (0..l2).for_each(|i| dp[0][i + 1] = s2[i] == s3[i] && dp[0][i]);
        for i in 0..l1 {
            for j in 0..l2 {
                dp[i + 1][j + 1] = (s1[i] == s3[i + j + 1] && dp[i][j + 1])
                    || (s2[j] == s3[i + j + 1] && dp[i + 1][j]);
            }
        }

        dp[l1][l2]
    }
}
// @lc code=end

struct Solution;
