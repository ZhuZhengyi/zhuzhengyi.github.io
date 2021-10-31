# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/flatten-list
@Language: Python
@Datetime: 17-02-05 08:18
'''

class Solution(object):

    # @param nestedList a list, each element in the list 
    # can be a list or integer, for example [1,2,[1,2]]
    # @return {int[]} a list of integer
    def flatten(self, nestedList):
        # Write your code here
        res = []
        
        if type(nestedList) == int:
            return [nestedList]
        
        for a in nestedList:
            res.extend(self.flatten(a))
                
        return res