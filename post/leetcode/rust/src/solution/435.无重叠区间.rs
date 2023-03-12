/*
 * @lc app=leetcode.cn id=435 lang=rust
 *
 * [435] 无重叠区间
 *
 * https://leetcode-cn.com/problems/non-overlapping-intervals/description/
 *
 * algorithms
 * Medium (50.73%)
 * Likes:    529
 * Dislikes: 0
 * Total Accepted:    102.4K
 * Total Submissions: 202K
 * Testcase Example:  '[[1,2],[2,3],[3,4],[1,3]]'
 *
 * 给定一个区间的集合，找到需要移除区间的最小数量，使剩余区间互不重叠。
 *
 * 注意:
 *
 *
 * 可以认为区间的终点总是大于它的起点。
 * 区间 [1,2] 和 [2,3] 的边界相互“接触”，但没有相互重叠。
 *
 *
 * 示例 1:
 *
 *
 * 输入: [ [1,2], [2,3], [3,4], [1,3] ]
 *
 * 输出: 1
 *
 * 解释: 移除 [1,3] 后，剩下的区间没有重叠。
 *
 *
 * 示例 2:
 *
 *
 * 输入: [ [1,2], [1,2], [1,2] ]
 *
 * 输出: 2
 *
 * 解释: 你需要移除两个 [1,2] 来使剩下的区间没有重叠。
 *
 *
 * 示例 3:
 *
 *
 * 输入: [ [1,2], [2,3] ]
 *
 * 输出: 0
 *
 * 解释: 你不需要移除任何区间，因为它们已经是无重叠的了。
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 贪心法
    /// 1. 要需移除区间最小数量 = 总量 - 最多非重叠区间数量 ；
    /// 2. 最多非重叠区间数量为所有区间从左到右排序，首尾不重叠时最长的排列；
    /// 3. 先将区间按右端点从小到大排序；
    /// 4. 取第一个区间的右端点为上一个重叠区标记端点last_end；
    /// 4. 从左到右依次比较区间v[0]和标记端点last_end;
    ///     4.1. 如果v[0]>=last_end, 表明发现一个新的非重叠区间，则将非重叠区间计数+1，同时更新标记端点last_end为v[1]
    ///     4.2. 否则未重叠区间，不做处理；
    /// 5. 遍历结束后，将区间总数 - 非重叠区间计数
    /// * 注：由于预先对区间右端点从小到大排序，last_end更新时，该非重叠区域内最小的，
    ///     因此最后的非重叠区域计数是最大值，此处即贪心；
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals.to_vec();
        intervals.sort_by_key(|v| v[1]);

        let mut last_end = intervals[0][1];
        let mut skip_count = 1;
        intervals[1..].iter().for_each(|v| {
            if v[0] >= last_end {
                last_end = v[1];
                skip_count += 1;
            }
        });

        intervals.len() as i32 - skip_count
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
            2
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
            0
        );
    }
}
