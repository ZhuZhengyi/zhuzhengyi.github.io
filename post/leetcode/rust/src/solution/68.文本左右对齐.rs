/*
 * @lc app=leetcode.cn id=68 lang=rust
 *
 * [68] 文本左右对齐
 *
 * https://leetcode.cn/problems/text-justification/description/
 *
 * algorithms
 * Hard (52.22%)
 * Likes:    334
 * Dislikes: 0
 * Total Accepted:    53.9K
 * Total Submissions: 103.2K
 * Testcase Example:  '["This", "is", "an", "example", "of", "text", "justification."]\n16'
 *
 * 给定一个单词数组 words 和一个长度 maxWidth ，重新排版单词，使其成为每行恰好有 maxWidth 个字符，且左右两端对齐的文本。
 *
 * 你应该使用 “贪心算法” 来放置给定的单词；也就是说，尽可能多地往每行中放置单词。必要时可用空格 ' ' 填充，使得每行恰好有 maxWidth
 * 个字符。
 *
 * 要求尽可能均匀分配单词间的空格数量。如果某一行单词间的空格不能均匀分配，则左侧放置的空格数要多于右侧的空格数。
 *
 * 文本的最后一行应为左对齐，且单词之间不插入额外的空格。
 *
 * 注意:
 *
 *
 * 单词是指由非空格字符组成的字符序列。
 * 每个单词的长度大于 0，小于等于 maxWidth。
 * 输入单词数组 words 至少包含一个单词。
 *
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: words = ["This", "is", "an", "example", "of", "text", "justification."],
 * maxWidth = 16
 * 输出:
 * [
 * "This    is    an",
 * "example  of text",
 * "justification.  "
 * ]
 *
 *
 * 示例 2:
 *
 *
 * 输入:words = ["What","must","be","acknowledgment","shall","be"], maxWidth = 16
 * 输出:
 * [
 * "What   must   be",
 * "acknowledgment  ",
 * "shall be        "
 * ]
 * 解释: 注意最后一行的格式应为 "shall be    " 而不是 "shall     be",
 * 因为最后一行应为左对齐，而不是左右两端对齐。
 * ⁠    第二行同样为左对齐，这是因为这行只包含一个单词。
 *
 *
 * 示例 3:
 *
 *
 * 输入:words =
 * ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"]，maxWidth
 * = 20
 * 输出:
 * [
 * "Science  is  what we",
 * ⁠ "understand      well",
 * "enough to explain to",
 * "a  computer.  Art is",
 * "everything  else  we",
 * "do                  "
 * ]
 *
 *
 *
 *
 * 提示:
 *
 *
 * 1 <= words.length <= 300
 * 1 <= words[i].length <= 20
 * words[i] 由小写英文字母和符号组成
 * 1 <= maxWidth <= 100
 * words[i].length <= maxWidth
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res = vec![];
        let mut line_words: Vec<String> = vec![];
        let mut output_lines = vec![];
        for word in words {
            let words_len = line_words.iter().fold(0, |mut len, w| {
                len += w.len();
                len
            });
            if words_len + line_words.len() + word.len() <= max_width as usize {
                line_words.push(word);
            } else {
                output_lines.push(line_words.clone());

                res.push(format_line_words(&mut line_words, max_width));

                line_words.clear();
                line_words.push(word);
            }
        }
        if line_words.len() > 0 {
            let word_len = line_words.iter().fold(0, |mut len, word| {
                len += word.len();
                len
            });
            let last_line = line_words.join(&" ");
            let remain_space = (max_width as usize) - last_line.len();
            if remain_space > 0 {
                res.push(format!("{}{}", last_line, " ".repeat(remain_space)));
            } else {
                res.push(last_line);
            }
        }

        fn format_line_words(line_words: &mut [String], max_width: i32) -> String {
            let word_count = line_words.len();
            if word_count == 0 {
                return String::new();
            }
            let word_len = line_words.iter().fold(0, |mut len, word| {
                len += word.len();
                len
            });
            let need_spaces = (max_width as usize) - word_len;
            if word_count == 1 {
                return format!("{}{}", line_words[0], &" ".repeat(need_spaces));
            } else {
                let mut i = 0;
                while i < need_spaces {
                    line_words[i % (word_count - 1)].push_str(" ");
                    i += 1;
                }
                return format!("{}", line_words.join(&""));
            }
        }

        res
    }
}
// @lc code=end
