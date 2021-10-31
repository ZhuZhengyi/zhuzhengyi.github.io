# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/n-queens
@Language: Python
@Datetime: 15-07-30 14:12
'''

class Solution:
    def check(self, board, i, j):
        for x in range(i):
            if board[x]==j or x-board[x]==i-j or x+board[x]==i+j:
                return False
        
        return True
        
    
    def dfs(self, res, board, i):
        n=len(board)
        if i==n:
            res.append(list(board))
        else:
            for j in range(0,n):
                if self.check(board,i,j):
                    board[i]=j
                    self.dfs(res,board, i+1)
                    board[i]=-1
    
    """
    Get all distinct N-Queen solutions
    @param n: The number of queens
    @return: All distinct solutions
    """
    def solveNQueens(self, n):
        # write your code here
        sols=[]
        board=[-1]*n
        
        self.dfs(sols, board, 0)
        
        res=[]
        for sol in sols:
            chess=[]
            for i in range(n):
                s=""
                for j in range(n):
                    if sol[i]==j:
                        s+='Q'
                    else:
                        s+='.'
                chess.append(s)
            res.append(list(chess))
        
        return res

