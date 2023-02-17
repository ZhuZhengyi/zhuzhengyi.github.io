/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 *
 * https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number/description/
 *
 * algorithms
 * Medium (52.58%)
 * Likes:    603
 * Dislikes: 0
 * Total Accepted:    84.3K
 * Total Submissions: 159.7K
 * Testcase Example:  '"23"'
 *
 * 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。
 * 
 * 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。
 * 
 * 
 * 
 * 示例:
 * 
 * 输入："23"
 * 输出：["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
 * 
 * 
 * 说明:
 * 尽管上面的答案是按字典序排列的，但是你可以任意选择答案输出的顺序。
 * 
 */


// @lc code=start
const MAPPING: [std::ops::RangeInclusive<u8>; 8] = [
    (b'a'..=b'c'),
    (b'd'..=b'f'),
    (b'g'..=b'i'),
    (b'j'..=b'l'),
    (b'm'..=b'o'),
    (b'p'..=b's'),
    (b't'..=b'v'),
    (b'w'..=b'z'),
];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        digits.as_bytes().iter().fold(
            if digits.is_empty() {
                Vec::new()
            } else {
                vec![String::new()]
            },
            |acc, &x| {
                acc.iter().flat_map(|s| {
                    std::iter::repeat(s)
                        .zip(MAPPING[(x-b'2') as usize].clone())
                        .map(|(s,b)| {
                            s.chars()
                             .chain(std::iter::once(b as char))
                             .collect() 
                        })
                        .collect::<Vec<_>>()
                })
                .collect()
            },
        )
    }
}
// @lc code=end

struct Solution;

#[test]
fn it_works() {
    assert_eq!( Solution::letter_combinations("23".to_string()), 
                vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
}

