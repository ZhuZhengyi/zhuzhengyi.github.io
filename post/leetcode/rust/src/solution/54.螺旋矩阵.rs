/*
 * @lc app=leetcode.cn id=54 lang=rust
 *
 * [54] 螺旋矩阵
 *
 * https://leetcode.cn/problems/spiral-matrix/description/
 *
 * algorithms
 * Medium (49.26%)
 * Likes:    1375
 * Dislikes: 0
 * Total Accepted:    364.9K
 * Total Submissions: 740.4K
 * Testcase Example:  '[[1,2,3],[4,5,6],[7,8,9]]'
 *
 * 给你一个 m 行 n 列的矩阵 matrix ，请按照 顺时针螺旋顺序 ，返回矩阵中的所有元素。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
 * 输出：[1,2,3,6,9,8,7,4,5]
 *
 *
 * 示例 2：
 *
 *
 * 输入：matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
 * 输出：[1,2,3,4,8,12,11,10,9,5,6,7]
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == matrix.length
 * n == matrix[i].length
 * 1
 * -100
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        let (mut l, mut r, mut u, mut d) = (0, matrix[0].len() - 1, 0, matrix.len() - 1);
        while l <= r && u <= d {
            // 左->右
            for col in l..=r {
                res.push(matrix[u][col]);
            }
            u += 1; // u向下移一行
            if u > d {
                break;
            }

            // 上->下
            for row in u..=d {
                res.push(matrix[row][r]);
            }
            if r == 0 {
                break;
            }
            r -= 1; // r左移一列
            if l > r {
                break;
            }

            // 右->左
            for col in (l..=r).rev() {
                res.push(matrix[d][col]);
            }
            if d == 0 {
                break;
            }
            d -= 1; //
            if d < u {
                break;
            }

            // 下->上
            for row in (u..=d).rev() {
                res.push(matrix[row][l]);
            }
            l += 1; //
            if l > r {
                break;
            }
        }

        res
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(Solution::spiral_order(vec![vec![3], vec![2]]), vec![3, 2]);
    }
}
