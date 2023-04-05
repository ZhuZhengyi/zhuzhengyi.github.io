/*
 * @lc app=leetcode.cn id=316 lang=rust
 *
 * [316] 去除重复字母
 *
 * https://leetcode.cn/problems/remove-duplicate-letters/description/
 *
 * algorithms
 * Medium (48.25%)
 * Likes:    906
 * Dislikes: 0
 * Total Accepted:    112.8K
 * Total Submissions: 233.9K
 * Testcase Example:  '"bcabc"'
 *
 * 给你一个字符串 s ，请你去除字符串中重复的字母，使得每个字母只出现一次。需保证 返回结果的字典序最小（要求不能打乱其他字符的相对位置）。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "bcabc"
 * 输出："abc"
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "cbacdcbc"
 * 输出："acdb"
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length <= 10^4
 * s 由小写英文字母组成
 *
 *
 *
 *
 * 注意：该题与 1081
 * https://leetcode-cn.com/problems/smallest-subsequence-of-distinct-characters
 * 相同
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 单调栈
    /// 1. 字符串中, 如果s[i]>s[i+1], 即当前一个字母字典序比后一个字符大，则删除当前字符s[i]后,字符串的字典序将变小;
    /// 2. 所以在重复的字符集中,找到s[i]>s[i+1]的字符,然后有重复的s[i]删掉,即可满足题目;
    /// 3. 为了方便计算s[i]>s[i+1], 可以使用单调栈来保存结果数组;
    /// 4. 在遍历字符串时, 如果当前字符不在栈中,
    ///    且栈顶字符是重复字符且字典序>当前字符,则弹出栈顶字符,直到栈顶字符不满足上述条件;
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut uniq_letters = String::with_capacity(s.len()); //单调栈,记录删除后变为唯一的字符
        let mut counts = vec![0; 26]; //记录各个字符出现的次数
        s.chars()
            .for_each(|c| counts[(c as u8 - b'a') as usize] += 1); //统计字符串中各个字符出现的次数
        let mut existed = vec![false; 26]; //记录遍历时，字母是否在结果字符串中出现过

        for ch in s.chars() {
            let i = (ch as u8 - b'a') as usize;
            counts[i] -= 1;
            // 如果当前字母不在最后结果中
            if !existed[i] {
                // 依次将栈中所有多次出现且字典序>当前字母的所有前置字母移除
                while let Some(letter) = uniq_letters
                    .chars()
                    .last()
                    .filter(|&l| ch < l && counts[(l as u8 - b'a') as usize] > 0)
                {
                    uniq_letters.pop(); //移除uniq_letters中的尾字符(该字符重复出现,且字典序>当前字符)
                    existed[(letter as u8 - b'a') as usize] = false; //
                }

                uniq_letters.push(ch); //将当前字符压入栈顶(结果数组的末尾)
                existed[i] = true; //标记该字符已经已在结果中
            }
        }

        uniq_letters
        //uniq_letters.iter().collect()
    }
}
// @lc code=end

struct Solution;
