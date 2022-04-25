/*
 * @lc app=leetcode.cn id=52 lang=cpp
 *
 * [52] N皇后 II
 *
 * https://leetcode-cn.com/problems/n-queens-ii/description/
 *
 * algorithms
 * Hard (82.31%)
 * Likes:    355
 * Dislikes: 0
 * Total Accepted:    90K
 * Total Submissions: 109.4K
 * Testcase Example:  '4'
 *
 * n 皇后问题 研究的是如何将 n 个皇后放置在 n × n 的棋盘上，并且使皇后彼此之间不能相互攻击。
 * 
 * 给你一个整数 n ，返回 n 皇后问题 不同的解决方案的数量。
 * 
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 4
 * 输出：2
 * 解释：如上图所示，4 皇后问题存在两个不同的解法。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 1
 * 输出：1
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= n <= 9
 * 
 * 
 * 
 * 
 */

#include<vector>
using namespace std;

// @lc code=start
class Solution {
    int result;
public:
    /*
    * ## 解题思路
    * * 回溯法
    */
    int totalNQueens(int n) {
        vector<vector<bool>> chessBoard(n, vector(n, false));

        dfs(chessBoard, 0);

        return result;
    }
    void dfs(vector<vector<bool>>& chessBoard, int r) {
        int n = chessBoard.size();
        if (r>n-1) {
            result += 1;
            return;
        }
        for(int c=0; c<n; c++) {
            if (!isValid(chessBoard, r, c)) {
                continue;
            }

            chessBoard[r][c] = true;
            dfs(chessBoard, r+1);
            chessBoard[r][c] = false;
        }
    }
    bool isValid(vector<vector<bool>>& chessBoard, int r, int c) {
        int n = chessBoard.size();
        for(int i=0; i<n; i++) {
            for(int j=0; j<n; j++) {
                if (chessBoard[i][j] && ( i==r || (j==c) || (i-j)==(r-c) || (i+j) == (r+c) )) {
                    return false;
                }
            }
        }
        return true;
    }
};
// @lc code=end

