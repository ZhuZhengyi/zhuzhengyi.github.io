# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/n-queens-ii
@Language: Python
@Datetime: 15-08-08 14:06
'''

class Solution:
    """
    Calculate the total number of distinct N-Queen solutions.
    @param n: The number of queens.
    @return: The total number of distinct solutions.
    """
    def totalNQueens(self, n):
        # write your code here
        res=0
        board=[-1]*n
        
        res=dfs(res, board, 0)
        
        return res

def check(board, i, j):
    for x in range(i):
        if board[x]==j or x-board[x]==i-j or x+board[x]==i+j:
            return False
    
    return True
    

def dfs(res, board, i):
    n=len(board)
    if i==n:
        #res+=1
        return res+1
        #res.append(list(board))
    else:
        for j in range(0,n):
            if check(board,i,j):
                board[i]=j
                res=dfs(res,board, i+1)
                board[i]=-1

    return res