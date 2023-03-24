/*
 * @lc app=leetcode.cn id=402 lang=rust
 *
 * [402] 移掉 K 位数字
 *
 * https://leetcode.cn/problems/remove-k-digits/description/
 *
 * algorithms
 * Medium (31.97%)
 * Likes:    929
 * Dislikes: 0
 * Total Accepted:    130.8K
 * Total Submissions: 409.9K
 * Testcase Example:  '"1432219"\n3'
 *
 * 给你一个以字符串表示的非负整数 num 和一个整数 k ，移除这个数中的 k 位数字，使得剩下的数字最小。请你以字符串形式返回这个最小的数字。
 *
 *
 * 示例 1 ：
 *
 *
 * 输入：num = "1432219", k = 3
 * 输出："1219"
 * 解释：移除掉三个数字 4, 3, 和 2 形成一个新的最小的数字 1219 。
 *
 *
 * 示例 2 ：
 *
 *
 * 输入：num = "10200", k = 1
 * 输出："200"
 * 解释：移掉首位的 1 剩下的数字为 200. 注意输出不能有任何前导零。
 *
 *
 * 示例 3 ：
 *
 *
 * 输入：num = "10", k = 2
 * 输出："0"
 * 解释：从原数字移除所有的数字，剩余为空就是 0 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * num 仅由若干位数字（0 - 9）组成
 * 除了 0 本身之外，num 不含任何前导零
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 单调栈
    /// 1.
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut ans = String::with_capacity(num.len());
        let mut k = k;
        for ch in num.chars() {
            // 如果k>0, 依次移除栈顶>当前字母的字母
            while k > 0 && ans.chars().last().filter(|&c| c > ch).is_some() {
                ans.pop();
                k -= 1;
            }
            // 前导的'0'不放入到结果中, 跳过
            if ans.is_empty() && ch == '0' {
                continue;
            }
            // 将当前字母加入到结果字符串尾部(压入单调栈顶)
            ans.push(ch);
        }

        // 如果k不为0, 需要继续移除ans中的后续k个字符
        for _ in 0..k {
            ans.pop();
        }

        // 处理空字符
        if ans.is_empty() {
            String::from("0")
        } else {
            ans
        }
    }
}
// @lc code=end

mod tests {
    use super::*;
    fn test() {
        assert_eq!(
            Solution::remove_kdigits("1432219".into(), 3),
            "1219".to_str()
        );
        assert_eq!(Solution::remove_kdigits("10200".into(), 1), "200".to_str());
        assert_eq!(Solution::remove_kdigits("10".into(), 2), "0".to_str());
        assert_eq!(Solution::remove_kdigits("9".into(), 1), "0".to_str());
    }
}
