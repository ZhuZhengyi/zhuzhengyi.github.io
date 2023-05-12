/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 * [56] 合并区间
 *
 * https://leetcode.cn/problems/merge-intervals/description/
 *
 * algorithms
 * Medium (49.30%)
 * Likes:    1896
 * Dislikes: 0
 * Total Accepted:    629.3K
 * Total Submissions: 1.3M
 * Testcase Example:  '[[1,3],[2,6],[8,10],[15,18]]'
 *
 * 以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi]
 * 。请你合并所有重叠的区间，并返回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
 * 输出：[[1,6],[8,10],[15,18]]
 * 解释：区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
 *
 *
 * 示例 2：
 *
 *
 * 输入：intervals = [[1,4],[4,5]]
 * 输出：[[1,5]]
 * 解释：区间 [1,4] 和 [4,5] 可被视为重叠区间。
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= intervals.length <= 10^4
 * intervals[i].length == 2
 * 0 <= starti <= endi <= 10^4
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// 1. 将所有区间按首元素排序;
    /// 2. 依次判断排序后的元素是否重叠, 是否重叠根据每个区间的首元素是否<=最后区间的尾元素决定;
    /// 3. 将未重叠的区间直接加入到结果集中;
    /// 4. 重叠的区间,根据尾元素判断是否更新最后区间的尾范围;
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;

        //按首元素对所有区间进行排序
        intervals.sort_by(|a, b| a[0].cmp(&b[0])); 

        let mut res: Vec<Vec<i32>> = vec![];
        //遍历排序后的区间集合
        for val in intervals {
            // 根据当前区间和结果集最后一个区间的关系, 判断当前区间是否重叠
            match res.last_mut() {
                // 如果当前区间头和最后一个区间尾有重叠 
                Some(last_val) if val[0] <= last_val[1] =>  {
                    // 当当前重叠的区间尾部比最后一个区间尾长时
                    if val[1] > last_val[1] { 
                        last_val[1] = val[1]; //更新最后一个区间尾
                    }
                }
                _ => res.push(val), //当前区间和之前区间没有重叠, 则直接将当前区间加入到结果数组尾部
            }
        }

        res
    }
}
// @lc code=end

struct Solution;
