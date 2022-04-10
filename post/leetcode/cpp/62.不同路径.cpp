/*
 * @lc app=leetcode.cn id=62 lang=cpp
 *
 * [62] 不同路径
 *
 * https://leetcode-cn.com/problems/unique-paths/description/
 *
 * algorithms
 * Medium (66.84%)
 * Likes:    1361
 * Dislikes: 0
 * Total Accepted:    413.2K
 * Total Submissions: 617.3K
 * Testcase Example:  '3\n7'
 *
 * 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为 “Start” ）。
 * 
 * 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish” ）。
 * 
 * 问总共有多少条不同的路径？
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：m = 3, n = 7
 * 输出：28
 * 
 * 示例 2：
 * 
 * 
 * 输入：m = 3, n = 2
 * 输出：3
 * 解释：
 * 从左上角开始，总共有 3 条路径可以到达右下角。
 * 1. 向右 -> 向下 -> 向下
 * 2. 向下 -> 向下 -> 向右
 * 3. 向下 -> 向右 -> 向下
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：m = 7, n = 3
 * 输出：28
 * 
 * 
 * 示例 4：
 * 
 * 
 * 输入：m = 3, n = 3
 * 输出：6
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 题目数据保证答案小于等于 2 * 10^9
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /*
    ## 解题思路
    * 动态规划
    f(m,n): 代表走到(m,n)这个方格的步数；
    走到(m,n)方格有两种走法：
        1. 先走到(m-1,n)方格处，再往下走一格；
        2. 先走到(m,n-1)方格处，再往右走一格；
    递推公式：f(m,n) = f(m-1, n) + f(m, n-1)
    初始条件: 
            f(i,0) = 1
            f(0,j) = 1
    */
    int uniquePaths(int m, int n) {
        vector<vector<int>> f(m, vector<int>(n, 1));
        for(int i=1; i<m; i++) {
            for(int j=1; j<n; j++) {
                f[i][j]=f[i-1][j]+f[i][j-1];
            }
        }
        return f[m-1][n-1];
    }
};
// @lc code=end

