/*
 * @lc app=leetcode.cn id=812 lang=rust
 *
 * [812] 最大三角形面积
 *
 * https://leetcode.cn/problems/largest-triangle-area/description/
 *
 * algorithms
 * Easy (68.65%)
 * Likes:    177
 * Dislikes: 0
 * Total Accepted:    34.6K
 * Total Submissions: 50.4K
 * Testcase Example:  '[[0,0],[0,1],[1,0],[0,2],[2,0]]'
 *
 * 给定包含多个点的集合，从其中取三个点组成三角形，返回能组成的最大三角形的面积。
 *
 *
 * 示例:
 * 输入: points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
 * 输出: 2
 * 解释:
 * 这五个点如下图所示。组成的橙色三角形是最大的，面积为2。
 *
 *
 *
 *
 * 注意:
 *
 *
 * 3 <= points.length <= 50.
 * 不存在重复的点。
 * -50 <= points[i][j] <= 50.
 * 结果误差值在 10^-6 以内都认为是正确答案。
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 公式
    /// 1. 平面三角形面积公式:
    ///    ` S = |x1*(y2 - y3) + x2*(y3-y1) + x3*(y1 - y2)| / 2`
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut s = 0_f64;
        for (i, p1) in points.iter().enumerate() {
            for (j, p2) in points.iter().skip(i).enumerate() {
                for (_, p3) in points.iter().skip(j).enumerate() {
                    if let [x1, y1] = p1[..] {
                        if let [x2, y2] = p2[..] {
                            if let [x3, y3] = p3[..] {
                                s = s.max(
                                    (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs() as f64
                                        / 2_f64,
                                );
                            }
                        }
                    }
                }
            }
        }

        s
    }
}
// @lc code=end
//

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::largest_triangle_area(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![0, 2],
                vec![2, 0]
            ]),
            2.0_f64
        );
    }
}
