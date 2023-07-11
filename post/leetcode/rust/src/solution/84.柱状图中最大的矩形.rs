/*
 * @lc app=leetcode.cn id=84 lang=rust
 *
 * [84] 柱状图中最大的矩形
 *
 * https://leetcode.cn/problems/largest-rectangle-in-histogram/description/
 *
 * algorithms
 * Hard (45.00%)
 * Likes:    2481
 * Dislikes: 0
 * Total Accepted:    349.3K
 * Total Submissions: 776K
 * Testcase Example:  '[2,1,5,6,2,3]'
 *
 * 给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。
 *
 * 求在该柱状图中，能够勾勒出来的矩形的最大面积。
 *
 *
 *
 * 示例 1:
 *
 *
 *
 *
 * 输入：heights = [2,1,5,6,2,3]
 * 输出：10
 * 解释：最大的矩形为图中红色区域，面积为 10
 *
 *
 * 示例 2：
 *
 *
 *
 *
 * 输入： heights = [2,4]
 * 输出： 4
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * 0
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 单调栈(单调递增栈)
    /// 1. 对于heights中的每一个heights[i], 最大面积矩形为: `heights[i] * ( right[i] - left[i] + 1 )`,
    ///    题目需要找出所有这些面积中最大的, 即 `max(heights[i] * (right[i] - left[i] + 1) )`.
    ///    其中:
    ///    - right[i]为当前柱子向右找到的第一个小于heights[i]的柱子的下标;
    ///    - left[i]为当前柱子向左找到的第一个小于heights[i]的柱子的下标;
    /// 2. 为了求取right[i], left[i], 可使用单调栈;
    /// 3. 单调栈inc_stack: 用于记录从左->右遍历时,已经访问过的最大heights[i];
    /// 4. 左->右遍历heights时, 如果栈顶元素高度<当前矩形高度heights[i], 则直接将heights[i]入栈;
    /// 5. 否则,如果栈顶元素高度 > 当前元素高度heights[i],
    ///    则以栈顶元素大小为高的最大面积矩形, 右边界r为i, 左边界l为栈顶下一个元素,
    ///    该矩形最大面积为: h * (i - l - 1)
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights = heights;
        heights.push(0); // heights左右各增加一个0, 以方便统一处理遍历边界;
        heights.insert(0, 0);
        let mut inc_stack: Vec<usize> = Vec::with_capacity(heights.len());
        let mut max_area = 0;
        for (i, &hs) in heights.iter().enumerate() {
            // 如果当前矩形高度 < 栈顶矩形高度
            while !inc_stack.is_empty() && hs < heights[*inc_stack.last().unwrap()] {
                //
                if let Some(cur) = inc_stack.pop() {
                    let h = heights[cur]; // 栈顶元素高度
                    let l = inc_stack.last().unwrap(); // 栈顶元素左边界为栈次顶元素id
                    let r = i; // 栈顶元素右边界为当前元素id;
                    max_area = max_area.max(h * ((r - l - 1) as i32));
                }
            }
            // 此时栈中(左边)高度>当前的矩形都已经出栈, 计算完了
            inc_stack.push(i); // 将当前矩形id入栈
        }

        max_area
    }
}
// @lc code=end

struct Solution;
