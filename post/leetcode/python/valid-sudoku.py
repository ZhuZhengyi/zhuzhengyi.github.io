# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/valid-sudoku
@Language: Python
@Datetime: 17-03-18 00:13
'''

class Solution:
    # @param board, a 9x9 2D array
    # @return a boolean
    def isValidSudoku(self, board):
        n=len(board)
        
        cols=[ [0]*n for _ in range(n) ]
        rows=[ [0]*n for _ in range(n) ]
        cells=[ [0]*n for _ in range(n) ] 
        
        #col
        for i,col in enumerate(board):
            for j,c in enumerate(col):
                if c!='.':
                    c=int(c)-1
                    if cols[i][c] != 0:
                        return False
                    else:
                        cols[i][c]=1
                        
                    if rows[j][c] != 0:
                        return False
                    else:
                        rows[j][c]=1
                        
                    k=(i/3)*3+j/3
                    if cells[k][c] != 0:
                        return False
                    else:
                        cells[k][c]=1
        
        return True

