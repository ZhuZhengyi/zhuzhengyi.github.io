# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/string-to-integer-ii
@Language: Python
@Datetime: 15-08-30 05:00
'''

class Solution:
    # @param str: a string
    # @return an integer
    def atoi(self, str):
        # write your code here
        line = [2,1,4,7,4,8,3,6,4,7]
        validletter='0123456789+-'
        startfu = '+-'
        limit =[2147483647, -2147483648]
        ret = 0
        nev = False
        filter = ''
        
        for i in range(0, len(str)):
            if(not str[i] in validletter):
                if(len(filter) == 0):
                    continue
                else:
                    break
            elif(str[i] in startfu):
                if(len(filter) != 0):
                    break
                else:
                    filter += str[i]
            else:
                filter += str[i]
            
        if(len(filter) == 0 or filter == '+' or filter == '-'):
            return 0
            
        if(filter[0] == '-'):
            nev = True
        pos = 0
        if(nev):
            pos = 1
            valid = filter[1:len(filter)]
        elif(filter[0] == '+'):
            valid = filter[1:len(filter)]
        else:
            valid = filter
            
        if(len(valid) > len(line)):
            return limit[pos]
            
        if(len(valid) == len(line)):
            for i in range(0, len(valid)):
                if(i == len(valid) - 1):
                    line[i] += 1
                if(valid[i] < line[i]):
                    break
                elif(valid[i] == line[i]):
                    continue
                else:
                    return limit[pos]
            
        integer = int(valid)
        if(nev):
            integer = 0 - integer
            
        return integer
            