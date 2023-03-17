/*
 * @lc app=leetcode.cn id=480 lang=rust
 *
 * [480] 滑动窗口中位数
 *
 * https://leetcode.cn/problems/sliding-window-median/description/
 *
 * algorithms
 * Hard (44.56%)
 * Likes:    410
 * Dislikes: 0
 * Total Accepted:    38.6K
 * Total Submissions: 86.8K
 * Testcase Example:  '[1,3,-1,-3,5,3,6,7]\n3'
 *
 * 中位数是有序序列最中间的那个数。如果序列的长度是偶数，则没有最中间的数；此时中位数是最中间的两个数的平均数。
 *
 * 例如：
 *
 *
 * [2,3,4]，中位数是 3
 * [2,3]，中位数是 (2 + 3) / 2 = 2.5
 *
 *
 * 给你一个数组 nums，有一个长度为 k 的窗口从最左端滑动到最右端。窗口中有 k 个数，每次窗口向右移动 1
 * 位。你的任务是找出每次窗口移动后得到的新窗口中元素的中位数，并输出由它们组成的数组。
 *
 *
 *
 * 示例：
 *
 * 给出 nums = [1,3,-1,-3,5,3,6,7]，以及 k = 3。
 *
 *
 * 窗口位置                      中位数
 * ---------------               -----
 * [1  3  -1] -3  5  3  6  7       1
 * ⁠1 [3  -1  -3] 5  3  6  7      -1
 * ⁠1  3 [-1  -3  5] 3  6  7      -1
 * ⁠1  3  -1 [-3  5  3] 6  7       3
 * ⁠1  3  -1  -3 [5  3  6] 7       5
 * ⁠1  3  -1  -3  5 [3  6  7]      6
 *
 *
 * 因此，返回该滑动窗口的中位数数组 [1,-1,-1,3,5,6]。
 *
 *
 *
 * 提示：
 *
 *
 * 你可以假设 k 始终有效，即：k 始终小于等于输入的非空数组的元素个数。
 * 与真实值误差在 10 ^ -5 以内的答案将被视作正确答案。
 *
 *
 */

// @lc code=start
use std::collections::VecDeque;
impl Solution {
    /// ## 解题思路
    /// - 有序队列
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let mut medians = Vec::new();
        let mut window = VecDeque::with_capacity(k);
        let mut sorted_window = VecDeque::with_capacity(k);
        for n in nums {
            // 将当前元素加入到窗口末尾;
            window.push_back(n);

            // 将当前元素插入到有序窗口的合适位置,
            // 使用二分查找来定位插入的位置.
            sorted_window.insert(sorted_window.binary_search(&n).unwrap_or_else(|i| i), n);

            // 如果窗口未满, 则继续处理下一个元素
            if window.len() < k {
                continue;
            }

            // 如果窗口已满
            if window.len() > k {
                // 移除窗口最左元素
                if let Some(dn) = window.pop_front() {
                    // 删除有序窗口中的已移除元素
                    sorted_window.remove(sorted_window.binary_search(&dn).unwrap());
                }
            }

            // 根据k的奇偶，计算有序窗口中的中位数
            let media = if k % 2 == 0 {
                (sorted_window[k / 2] as f64 + sorted_window[k / 2 - 1] as f64) / 2_f64
            } else {
                sorted_window[k / 2] as f64
            };

            medians.push(media);
        }

        medians
    }
}
// @lc code=end

struct Solution;

mod tests {
    use super::*;
    fn test() {
        assert_eq!(
            Solution::median_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7].to_vec(), 3),
            vec![1, -1, -1, 3, 5, 6].to_vec()
        );
    }
}
