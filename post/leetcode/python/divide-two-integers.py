# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/divide-two-integers
@Language: Python
@Datetime: 15-08-29 14:56
'''

class Solution:
    # @param {int} dividend the dividend
    # @param {int} divisor the divisor
    # @return {int} the result
    def divide(self, dividend, divisor):
        # Write your code here

        if divisor==0:
            return None

        if dividend==0:
            return 0

        if divisor<0:
            res=-self.divide(dividend, -divisor)
            if res>2147483647:
                res=2147483647
            elif res<-2147483647:
                res=-2147483648
            return res

        if dividend<0:
            res=-self.divide(-dividend, divisor)
            if res>2147483647:
                res=2147483647
            elif res<-2147483647:
                res=-2147483648
            return res

        if divisor==1:
            return dividend

        if dividend<divisor:
            return 0

        if dividend==divisor:
            return 1

        res=dividend-divisor
        res=0
        while dividend>=divisor:
            tmp=divisor
            cnt=1
            while dividend>=tmp :
                dividend-=tmp
                res+=cnt
                tmp=tmp<<1
                cnt=cnt<<1

        return res


