/*
 * @lc app=leetcode.cn id=63 lang=cpp
 *
 * [63] 不同路径 II
 *
 * https://leetcode-cn.com/problems/unique-paths-ii/description/
 *
 * algorithms
 * Medium (39.93%)
 * Likes:    766
 * Dislikes: 0
 * Total Accepted:    241.7K
 * Total Submissions: 604.2K
 * Testcase Example:  '[[0,0,0],[0,1,0],[0,0,0]]'
 *
 * 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为 “Start” ）。
 * 
 * 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish”）。
 * 
 * 现在考虑网格中有障碍物。那么从左上角到右下角将会有多少条不同的路径？
 * 
 * 网格中的障碍物和空位置分别用 1 和 0 来表示。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
 * 输出：2
 * 解释：3x3 网格的正中间有一个障碍物。
 * 从左上角到右下角一共有 2 条不同的路径：
 * 1. 向右 -> 向右 -> 向下 -> 向下
 * 2. 向下 -> 向下 -> 向右 -> 向右
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：obstacleGrid = [[0,1],[0,0]]
 * 输出：1
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * m == obstacleGrid.length
 * n == obstacleGrid[i].length
 * 1 <= m, n <= 100
 * obstacleGrid[i][j] 为 0 或 1
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /*
    ## 解题思路
    * 动态规划
    1. f(m,n)代表到达m,n的方法数；
    2. f(m,n) = f(m-1,n) + f(m, n-1)
    3. 如果obstracleGrid[m][n]==1, 则f(m,n) = 0, 否则f(m,n) > 0;
    4. f(0,n),f(m,0)作为边界情况，需要单独处理；
    */
    int uniquePathsWithObstacles(vector<vector<int>>& obstacleGrid) {
        int m = obstacleGrid.size();
        int n = obstacleGrid[0].size();
        vector<vector<int>> f(m, vector<int>(n, 0));
        for (int i=0; i <m; i++) {
            for(int j=0; j<n; j++) {
                if (obstacleGrid[i][j] == 1) {
                    f[i][j] = 0;
                } else if (i == 0 || j==0) {
                    if (i==0 && j==0) {
                        f[i][j] = 1-obstacleGrid[i][j];
                    } else if (j>0) {
                        f[i][j] = f[i][j-1];
                    } else if(i>0) {
                        f[i][j] = f[i-1][j];
                    }
                } else {
                    f[i][j] = f[i-1][j] + f[i][j-1];
                }
            }
        }

        return f[m-1][n-1];
    }
    /*
    * 优化：
    5. 可以考虑扩展一行和一列，将第0行第0列变为第1行，第1列, 第0行第0列初始化为0，进行统一处理；
    6. 设置[0,1]为起始点,即f(0,1)=1;
    */
    int uniquePathsWithObstacles2(vector<vector<int>>& obstacleGrid) {
        int m = obstacleGrid.size();
        int n = obstacleGrid[0].size();
        vector<vector<int>> f(m+1, vector<int>(n+1, 0));
        f[0][1]=1; //从扩展的[0,1]格开始出发，[1,0]也可以，但不能同时
        for (int i=1; i <=m; i++) {
            for(int j=1; j<=n; j++) {
                if (obstacleGrid[i-1][j-1] == 0) { //obstacleGrid没有扩展
                    f[i][j] = f[i-1][j] + f[i][j-1];
                }
            }
        }

        return f[m][n];
    }
};
// @lc code=end

