/*
 * @lc app=leetcode.cn id=51 lang=cpp
 *
 * [51] N 皇后
 *
 * https://leetcode-cn.com/problems/n-queens/description/
 *
 * algorithms
 * Hard (74.00%)
 * Likes:    1297
 * Dislikes: 0
 * Total Accepted:    207.5K
 * Total Submissions: 280.4K
 * Testcase Example:  '4'
 *
 * n 皇后问题 研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
 * 
 * 给你一个整数 n ，返回所有不同的 n 皇后问题 的解决方案。
 * 
 * 
 * 
 * 每一种解法包含一个不同的 n 皇后问题 的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 4
 * 输出：[[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
 * 解释：如上图所示，4 皇后问题存在两个不同的解法。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 1
 * 输出：[["Q"]]
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

#include <vector>
#include <string>
using namespace std;

// @lc code=start
class Solution {
    vector<vector<string> > result; //用来保存结果的环境变量；

public:

    /**
    * ## 解题思路
    * * 回溯法
    * 1. 设`f(r)`: 按行在各个棋盘格子中放置'Q';
    * 2. 当`r>n-1`时，结束；
    **/
    vector<vector<string> > solveNQueens(int n) {
        vector<string> chessBoard(n, string(n, '.'));

        trySetQueens(chessBoard, n, 0);

        return result;
    }

    void trySetQueens(vector<string>& chessBoard, int n, int r) {
        if (r > n-1) {
            result.push_back(chessBoard);
            return;
        }
        for(int c=0; c<n; c++) {
            if (!isValid(chessBoard, n, r, c)) {
                continue;
            }

            chessBoard[r][c] = 'Q';
            trySetQueens(chessBoard, n, r+1);
            chessBoard[r][c] = '.';
        }
    }

    // 检测在(r,c)是否能合法放置'Q'
    bool isValid(vector<string>& chessBoard, int n, int r, int c) {
        for(int i=0; i<n; i++) {
            for(int j=0; j<n; j++) {
                if (chessBoard[i][j] == 'Q' && (i==r || j==c || (i-j)==(r-c) || (i+j)==(r+c)) ) {
                    return false;
                }
            }
        }
        return true;
    }

};
// @lc code=end

