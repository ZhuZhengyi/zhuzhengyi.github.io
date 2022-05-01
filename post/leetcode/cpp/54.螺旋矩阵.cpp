/*
 * @lc app=leetcode.cn id=54 lang=cpp
 *
 * [54] 螺旋矩阵
 *
 * https://leetcode-cn.com/problems/spiral-matrix/description/
 *
 * algorithms
 * Medium (48.67%)
 * Likes:    1079
 * Dislikes: 0
 * Total Accepted:    256.4K
 * Total Submissions: 526.5K
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
class Solution {
public:
    /**
     * ## 解题思路
     * * 设置4个变量，l,r,u,d；
     * * 分别表示遍历螺旋的左，右，上，下4个边界；
     * * 按照 先向左，再向下，向右，向上 4种方向进行遍历；
    */
    vector<int> spiralOrder(vector<vector<int>>& matrix) {
        vector<int> res;
        int m = matrix.size();          //depth
        int n = m?matrix[0].size():0;   //width

        int l=0, r=n-1, u=0, d=m-1;
        while(l<=r&&u<=d) {
            // to right, increase col
            for(int col = l; col <= r; ++col) {
                res.push_back(matrix[u][col]);
            }
            // 向下移动一行
            if (++u>d) {
                break;
            }
            // to down, increase row
            for(int row = u; row <= d; ++row) {
                res.push_back(matrix[row][r]);
            }
            // 向左移动一列
            if (--r<l) {
                break;
            }

            // to left, sub col
            for(int col = r; col >= l; --col) {
                res.push_back(matrix[d][col]);
            }
            // 向上移动一行
            if (--d<u) {
                break;
            }
            // to up, sub row
            for(int row = d; row >= u; --row) {
                res.push_back(matrix[row][l]);
            }
            // 向右移动一列
            if (++l>r) {
                break;
            }
        }

        return res;
    }
};
// @lc code=end