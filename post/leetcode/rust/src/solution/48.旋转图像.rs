/*
 * @lc app=leetcode.cn id=48 lang=rust
 *
 * [48] 旋转图像
 *
 * https://leetcode.cn/problems/rotate-image/description/
 *
 * algorithms
 * Medium (74.65%)
 * Likes:    1613
 * Dislikes: 0
 * Total Accepted:    443.9K
 * Total Submissions: 594.4K
 * Testcase Example:  '[[1,2,3],[4,5,6],[7,8,9]]'
 *
 * 给定一个 n × n 的二维矩阵 matrix 表示一个图像。请你将图像顺时针旋转 90 度。
 *
 * 你必须在 原地 旋转图像，这意味着你需要直接修改输入的二维矩阵。请不要 使用另一个矩阵来旋转图像。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
 * 输出：[[7,4,1],[8,5,2],[9,6,3]]
 *
 *
 * 示例 2：
 *
 *
 * 输入：matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
 * 输出：[[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]
 *
 *
 *
 *
 * 提示：
 *
 *
 * n == matrix.length == matrix[i].length
 * 1 <= n <= 20
 * -1000 <= matrix[i][j] <= 1000
 *
 *
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// 1. 旋转顺时针90度可分为两步:
    ///    a. 先上下对称翻转;
    ///    b. 再沿左上右下对角线翻转;
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // 上下翻转
        matrix.reverse();
        // 沿左上右下对角线翻转
        for i in 0..matrix.len() {
            for j in 0..i {
                // 交换i,j坐标元素
                unsafe {
                    std::ptr::swap(&mut matrix[i][j], &mut matrix[j][i]);
                }
            }
        }
    }
}
// @lc code=end

struct Solution;