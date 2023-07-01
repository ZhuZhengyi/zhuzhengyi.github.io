/*
 * @lc app=leetcode.cn id=76 lang=rust
 *
 * [76] 最小覆盖子串
 *
 * https://leetcode.cn/problems/minimum-window-substring/description/
 *
 * algorithms
 * Hard (45.19%)
 * Likes:    2551
 * Dislikes: 0
 * Total Accepted:    432.5K
 * Total Submissions: 956.9K
 * Testcase Example:  '"ADOBECODEBANC"\n"ABC"'
 *
 * 给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 ""
 * 。
 *
 *
 *
 * 注意：
 *
 *
 * 对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
 * 如果 s 中存在这样的子串，我们保证它是唯一的答案。
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "ADOBECODEBANC", t = "ABC"
 * 输出："BANC"
 * 解释：最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "a", t = "a"
 * 输出："a"
 * 解释：整个字符串 s 是最小覆盖子串。
 *
 *
 * 示例 3:
 *
 *
 * 输入: s = "a", t = "aa"
 * 输出: ""
 * 解释: t 中两个字符 'a' 均应包含在 s 的子串中，
 * 因此没有符合条件的子字符串，返回空字符串。
 *
 *
 *
 * 提示：
 *
 *
 * ^m == s.length
 * ^n == t.length
 * 1 <= m, n <= 10^5
 * s 和 t 由英文字母组成
 *
 *
 *
 * 进阶：你能设计一个在 o(m+n) 时间内解决此问题的算法吗？
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 滑动窗口 + hashmap(可用固定数组代替)
    /// 1. 窗口右边界滑动时,检查当前字符在窗口内出现的次数是否达到目标字符串;
    /// 2. 如果达到,则增加有效计数;
    /// 3. 检查窗口左边界字符出现次数是否超过目标字符串中字符次数, 如果超过,则右滑左边界;
    /// 4. 检查有效计数是否达到目标字符串大小,如果达到,则更新结果字符串;
    pub fn min_window(s: String, t: String) -> String {
        let mut min_substr = "";

        let mut t_stat = vec![0; 60]; // 目标字符串字符频次统计
        let mut w_stat = vec![0; 60]; // 滑动窗口内字符频次统计

        // 统计目标字符串中各字符次数
        t.bytes().map(|b| (b - b'A') as usize).for_each(|id| {
            t_stat[id] += 1;
        });

        let mut sb = s.as_bytes();
        let mut valid_count = 0; //窗口内有效字符计数
        let mut l = 0; // 滑动窗口左右边界
        for r in 0..sb.len() {
            let ri = (sb[r] - b'A') as usize;
            // 增加滑动窗口内部右边界字符频次统计
            w_stat[ri] += 1;
            // 如果当前字符不在目标字符串内
            if t_stat[ri] == 0 {
                continue;
            }
            // 如果当前字符频次未超过目标字符串内字符频次
            if w_stat[ri] <= t_stat[ri] {
                valid_count += 1; // 有效字符计数递增
            }
            // 处理滑窗左边界
            while l < r {
                let li = (sb[l] - b'A') as usize;
                // 如果窗口左边界字符频次 超过 目标字符频次
                if w_stat[li] > t_stat[li] {
                    l += 1; // 滑动左边界
                    w_stat[li] -= 1; // 滑窗中的字符次数递减
                } else {
                    break;
                }
            }

            // 如果当前窗口内有效字符数 == 目标字符数
            if valid_count == t.len() {
                // 如果
                if min_substr.is_empty() || r - l + 1 < min_substr.len() {
                    min_substr = &s[l..=r];
                }
            }
        }

        min_substr.to_string()
    }
}
// @lc code=end

struct Solution;
