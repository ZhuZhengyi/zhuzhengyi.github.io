// @before-stub-for-debug-begin
#include <vector>
#include <string>
#include "commoncppproblem37.h"

using namespace std;
// @before-stub-for-debug-end

/*
 * @lc app=leetcode.cn id=37 lang=cpp
 *
 * [37] 解数独
 *
 * https://leetcode-cn.com/problems/sudoku-solver/description/
 *
 * algorithms
 * Hard (67.36%)
 * Likes:    1234
 * Dislikes: 0
 * Total Accepted:    138K
 * Total Submissions: 204.8K
 * Testcase Example:  '[["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]'
 *
 * 编写一个程序，通过填充空格来解决数独问题。
 * 
 * 数独的解法需 遵循如下规则：
 * 
 * 
 * 数字 1-9 在每一行只能出现一次。
 * 数字 1-9 在每一列只能出现一次。
 * 数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。（请参考示例图）
 * 
 * 
 * 数独部分空格内已填入了数字，空白格用 '.' 表示。
 * 
 * 
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：board =
 * [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
 * 
 * 输出：[["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
 * 解释：输入的数独如上图所示，唯一有效的解决方案如下所示：
 * 
 * 
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * board.length == 9
 * board[i].length == 9
 * board[i][j] 是一位数字或者 '.'
 * 题目数据 保证 输入数独仅有一个解
 * 
 * 
 * 
 * 
 * 
 */

#include<vector>
using namespace std;

// @lc code=start
class Solution {
    bool col[9][9];         //第i行字符c是否已经存在；
    bool row[9][9];         //第j列字符c是否已经存在;
    bool cell[9][9];        //小单元中的字符监测

public:
    /*
    * ## 解题思路
    * * 回溯法
    *   1. 对于每个空位，依次使用1-9字符进行尝试 f(idx)
    *   2. 如果冲突，则尝试用下一个字符尝试；
    *   3. 如果不冲突，继续尝试后续 f(idx+1)
    *      3.1 如果后续尝试合法(当前也合法)，则得到一个解；
    *      3.2 如果不合法，则需要回退当前
    *   4. 如果字符1-9都尝试完，还没有合法的，则false；
    */
    void solveSudoku(vector<vector<char>>& board) {
        // 初始化
        for(int i=0; i<9; i++) {
            for(int j=0; j<9; j++) {
                char c=board[i][j];
                int k=3*(i/3)+j/3;
                if (c!='.') {
                    int ci=c-'1';
                    col[i][ci]=row[j][ci]=cell[k][ci] = true;
                }
            }
        }

        tryFillCharAtIdx(board, 0);
        //tryFillChar(board);
    }

    // 检查i,j位置填写c字符是否有效
    bool isCharValid(int i, int j, char c) {
        int k=3*(i/3)+j/3;
        int ci=c-'1';
        return (!col[i][ci] && !row[j][ci] && !cell[k][ci]);
    }

    bool tryFill(vector<vector<char>>& board) {
        //依次检查每个格子，
        for(int i=0; i<9; i++) {
            for(int j=0; j<9; j++) {
                //格子已有数字，则跳过，继续下一个格子
                if (board[i][j] != '.') {
                    continue;
                }
                //空格

                int k=3*(i/3)+j/3;
                //依次为空格尝试填入1-9
                for(char c='1'; c<='9'; c++) {
                    int ci=c-'1';

                    //检查要填的数字是否有效
                    if (!isCharValid(i, j, c)) { 
                        continue;  //有冲突，则尝试下一个数字
                    }

                    //有效
                    //填入数字
                    board[i][j] = c;
                    col[i][ci]=row[j][ci]=cell[k][ci] = true; //更新已填充的格子

                    //填入后，递归检查后续是否合法
                    if (tryFill(board)) {
                        return true;  //如果后续都能
                    }
                    
                    //否则，后续无法组成，撤回刚才填写的数字
                    col[i][ci]=row[j][ci]=cell[k][ci] = false; //更新已填充的格子
                    board[i][j] = '.';

                    //继续使用下一个字符尝试
                }

                // 尝试完了所有字符，没有合法解，则
                return false;
            }
        }
        //所有格子都检查完，没有无效的情况，则所填的为正确的
        return true;
    }

    /**
    * 往第idx个格子出填充字符
    */
    bool tryFillCharAtIdx(vector<vector<char>>& board, int idx) {
        // 所有格子已经填充完，则填充结束
        if (idx >= 81 ) {
            return true;
        }

        int i = idx / 9;    //所在行
        int j = idx % 9;    //所在列
        int k = 3*(i/3)+j/3; //所在cell

        //当前格子已填充字符
        if (board[i][j] != '.') {   
            return tryFillCharAtIdx(board, idx+1); 
        } 
            
        //当前格子未填充
        // 尝试填充
        for(char c='1'; c<='9'; c++) {
            int ci=c-'1';

            //行,列或者单元格中已经存在c, 则换下一个字符
            if (!isCharValid(i, j, c)) {
                continue;
            }

            //填充当前格子
            board[i][j] = c;  
            col[i][ci]=row[j][ci]=cell[k][ci] = true; //更新已填充的格子
            
            //继续填后面的空格
            if (tryFillCharAtIdx(board, idx+1)) { 
                return true; //如果后面的也合法，为一个有效解, 可以返回；
            }

            //否则，后续无法填到合法的，撤回当前所填的数字
            col[i][ci]=row[j][ci]=cell[k][ci] = false; //恢复
            board[i][j] = '.';

            //重新尝试下一个数字
        }

        // 尝试完了所有字符，没有合法解，则
        return false;
    }
};
// @lc code=end

