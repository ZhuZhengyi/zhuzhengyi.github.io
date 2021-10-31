# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/cosine-similarity
@Language: Python
@Datetime: 15-09-25 14:40
'''

class Solution:
    """
    @param A: An integer array.
    @param B: An integer array.
    @return: Cosine similarity.
    """
    def cosineSimilarity(self, A, B):
        # write your code here
        if A==[] or B==[]:
            return 2.0
        
        if sum([i*i for i in A]) == 0 or sum([i*i for i in B]) == 0:
            return 2.0
            
        return sum([x*y for x,y in zip(A,B)])/(math.sqrt(sum([i*i for i in A]))*math.sqrt(sum([i*i for i in B])))
