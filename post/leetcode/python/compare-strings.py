# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/compare-strings
@Language: Python
@Datetime: 15-07-20 01:18
'''

class Solution:
    """
    @param A : A string includes Upper Case letters
    @param B : A string includes Upper Case letters
    @return :  if string A contains all of the characters in B return True else return False
    """
    def compareStrings(self, A, B):
        # write your code here
        a_dic={}
        for a in A:
            if a in a_dic:
                a_dic[a]+=1
            else:
                a_dic[a]=1
                
        for b in B:
            if b not in a_dic:
                return False
            elif b in a_dic:
                if a_dic[b] > 0:
                    a_dic[b]-=1
                else:
                    return False
                
        return True
                
