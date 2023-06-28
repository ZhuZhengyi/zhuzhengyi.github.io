/*
 * @lc app=leetcode.cn id=74 lang=rust
 *
 * [74] 搜索二维矩阵
 *
 * https://leetcode.cn/problems/search-a-2d-matrix/description/
 *
 * algorithms
 * Medium (48.71%)
 * Likes:    811
 * Dislikes: 0
 * Total Accepted:    316K
 * Total Submissions: 648.7K
 * Testcase Example:  '[[1,3,5,7],[10,11,16,20],[23,30,34,60]]\n3'
 *
 * 编写一个高效的算法来判断 m x n 矩阵中，是否存在一个目标值。该矩阵具有如下特性：
 *
 *
 * 每行中的整数从左到右按升序排列。
 * 每行的第一个整数大于前一行的最后一个整数。
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
 * 输出：true
 *
 *
 * 示例 2：
 *
 *
 * 输入：matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
 * 输出：false
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
 * -10^4
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 二分查找
    /// 1. 由于该矩阵的特性,
    /// 2. 二分法依次比较target和第0列大小, 找到target处在那一行;
    /// 3. 在该行中, 通过二分法查找target是否处于该行中;
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut l, mut r) = (0, m);
        let mut row = 0;
        while l < r {
            row = (l + r) / 2;
            if matrix[row][0] == target || matrix[row][n - 1] == target {
                return true;
            } else if matrix[row][0] < target {
                if matrix[row][n - 1] > target {
                    break;
                }
                l = row + 1;
            } else {
                r = row;
            }
        }
        let (mut l, mut r) = (0, n);
        while l < r {
            let col = (l + r) / 2;
            if matrix[row][col] == target {
                return true;
            } else if matrix[row][col] < target {
                l = col + 1;
            } else {
                r = col;
            }
        }

        return false;
    }
}
// @lc code=end

struct Solution;
