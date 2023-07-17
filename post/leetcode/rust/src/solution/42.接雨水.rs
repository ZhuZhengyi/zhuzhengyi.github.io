/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 *
 * https://leetcode.cn/problems/trapping-rain-water/description/
 *
 * algorithms
 * Hard (62.52%)
 * Likes:    4252
 * Dislikes: 0
 * Total Accepted:    665.2K
 * Total Submissions: 1.1M
 * Testcase Example:  '[0,1,0,2,1,0,1,3,2,1,2,1]'
 *
 * 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
 *
 *
 *
 * 示例 1：
 *
 *
 *
 *
 * 输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
 * 输出：6
 * 解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。
 *
 *
 * 示例 2：
 *
 *
 * 输入：height = [4,2,0,3,2,5]
 * 输出：9
 *
 *
 *
 *
 * 提示：
 *
 *
 * n == height.length
 * 1 <= n <= 2 * 10^4
 * 0 <= height[i] <= 10^5
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 双指针
    /// 1. 对于每一列height[i],其盛水的量由左右两边中较低的边沿确定,而每边的边沿为这边中所有边界的最大值,用公式表示
    ///    - 桶边界为: h_edge[i] = max(height[i], min(max(height[..i]), max(height[i+1..])))
    ///    - 盛水量为:  water[i] = h_edge[i] - height[i]
    ///    - 总盛水量:  waters = sum(water[..])
    /// 2. 为了计算每列的桶边界, 设置双指针l,r=0, n-1;
    /// 3. 如果height[l] < height[r], 则对于i=l的列, 其桶边界必定在左边即:
    ///      h_edge = max(height[..l], height[l])
    ///    否则对与i=r的列, 桶边界为:
    ///      h_edge = max(height[r+1..], height[r])
    /// 4. 综合以上2种情况,当l,r都从外向内移动时,可以用一个变量来记录h_edge;
    pub fn trap2(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len() - 1);
        let mut h_edge = 0; // 当前列的桶边界
        let mut waters = 0; // 总水量
        while l < r {
            let hi = if height[l] < height[r] {
                l += 1;
                height[l - 1]
            } else {
                r -= 1;
                height[r + 1]
            };
            h_edge = h_edge.max(hi);
            waters += h_edge - hi;
        }

        waters
    }

    /// ## 解题思路2
    /// - 单调栈
    /// 1. 设置单调递减栈desc_wall, 用于保存height中;
    /// 2. 从左->右遍历height;
    /// 3. 如果栈为空或当前柱子height[i] < 栈顶柱子高度, 则当前柱子可能会积水. 将当前柱子索引i入栈;
    /// 4. 如果当前柱子高度height[i] > 栈顶柱子高度, 则
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut desc_wall = Vec::with_capacity(height.len());
        for (i, &h) in height.iter().enumerate() {
            // 如果栈不为空, 且当前柱子高度 > 栈顶柱子高度, 则可能会积水
            while !desc_wall.is_empty() && h > height[*desc_wall.last().unwrap()] {
                let bottom_i = desc_wall.pop().unwrap(); // 栈顶为积水底部

                // 如果左侧有更高的柱子, 则可以积水
                if let Some(&left_i) = desc_wall.last() {
                    let width = (i - left_i - 1) as i32; // 积水区宽度为:当前柱子 到 左边高柱子之间的部分
                    let min_wall = h.min(height[left_i]); // 积水区高度为: 左右两侧高度中较低的
                    total += width * (min_wall - height[bottom_i]);
                }
            }
            // 栈中所有低于当前高度的柱子已经处理完
            // 栈顶柱子高度 > 当前柱子高度, 则可能会积水(当后面有柱子高于当前柱子时)
            desc_wall.push(i); // 入栈, 等后续出现高于当前高度的柱子时处理
        }

        total
    }
}
// @lc code=end

struct Solution;
