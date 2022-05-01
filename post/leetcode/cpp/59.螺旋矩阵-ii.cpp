/*
 * @lc app=leetcode.cn id=59 lang=cpp
 *
 * [59] 螺旋矩阵 II
 *
 * https://leetcode-cn.com/problems/spiral-matrix-ii/description/
 *
 * algorithms
 * Medium (76.50%)
 * Likes:    696
 * Dislikes: 0
 * Total Accepted:    191.5K
 * Total Submissions: 250.5K
 * Testcase Example:  '3'
 *
 * 给你一个正整数 n ，生成一个包含 1 到 n^2 所有元素，且元素按顺时针顺序螺旋排列的 n x n 正方形矩阵 matrix 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 3
 * 输出：[[1,2,3],[8,9,4],[7,6,5]]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 1
 * 输出：[[1]]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    vector<vector<int>> generateMatrix(int n) {
        vector<vector<int>> res(n, vector<int>(n, 0));

        int l=0, r=n-1, u=0, d=n-1;
        int i=1;
        while(l<=r&&u<=d) {
            // to right, increase col
            for(int col = l; col <= r; ++col) {
                res[u][col]=i++;
            }
            // 向下移动一行
            if (++u>d) {
                break;
            }
            // to down, increase row
            for(int row = u; row <= d; ++row) {
                res[row][r]=i++;
            }
            // 向左移动一列
            if (--r<l) {
                break;
            }

            // to left, sub col
            for(int col = r; col >= l; --col) {
                res[d][col]=i++;
            }
            // 向上移动一行
            if (--d<u) {
                break;
            }
            // to up, sub row
            for(int row = d; row >= u; --row) {
                res[row][l]=i++;
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

