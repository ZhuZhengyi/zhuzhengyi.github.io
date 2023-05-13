/*
 * @lc app=leetcode.cn id=60 lang=rust
 *
 * [60] 排列序列
 *
 * https://leetcode.cn/problems/permutation-sequence/description/
 *
 * algorithms
 * Hard (53.40%)
 * Likes:    769
 * Dislikes: 0
 * Total Accepted:    127.5K
 * Total Submissions: 238.8K
 * Testcase Example:  '3\n3'
 *
 * 给出集合 [1,2,3,...,n]，其所有元素共有 n! 种排列。
 *
 * 按大小顺序列出所有排列情况，并一一标记，当 n = 3 时, 所有排列如下：
 *
 *
 * "123"
 * "132"
 * "213"
 * "231"
 * "312"
 * "321"
 *
 *
 * 给定 n 和 k，返回第 k 个排列。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 3, k = 3
 * 输出："213"
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 4, k = 9
 * 输出："2314"
 *
 *
 * 示例 3：
 *
 *
 * 输入：n = 3, k = 1
 * 输出："123"
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 递归
    /// 1. k = c1 * (n-1)! + k'
    /// 2. k' = c2 * (n-2)!
    ///    ..
    ///    结果为 c1c2..cn
    pub fn get_permutation(n: i32, k: i32) -> String {
        let factor = (1..n).fold(vec![1], |mut acc, i| {
            acc.push(acc.last().unwrap() * i);
            acc
        });
        let mut nums = String::from("123456789").as_bytes()[..(n as usize)].to_vec();
        let mut res = vec![];
        let mut k = k - 1;
        let mut n = n;
        while k > 0 {
            let c = (k / factor[(n - 1) as usize]) as usize;
            res.push(nums.remove(c as usize));
            k %= factor[(n - 1) as usize];
            n -= 1;
        }

        String::from_utf8([res, nums].concat()).unwrap_or(String::new())
    }
}
// @lc code=end
