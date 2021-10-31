# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/gas-station
@Language: Python
@Datetime: 15-08-30 10:44
'''

class Solution:
    # @param gas, a list of integers
    # @param cost, a list of integers
    # @return an integer
    def canCompleteCircuit(self, gas, cost):
        # write your code here
        
        start=0
        cur=0
        #totalGas=0
        #totalCost=0
        
        for i in range(len(gas)):
            cur+=gas[i]
            cur-=cost[i]
            #totalGas+=gas[i]
            #totalCost+=cost[i]
            if cur<0:
                start=i+1
                cur=0
        
        if sum(gas)>=sum(cost):
            return start
            
        return -1