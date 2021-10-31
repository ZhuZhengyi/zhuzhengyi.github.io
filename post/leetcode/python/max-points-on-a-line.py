# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/max-points-on-a-line
@Language: Python
@Datetime: 15-07-19 09:34
'''

# Definition for a point.
# class Point:
#     def __init__(self, a=0, b=0):
#         self.x = a
#         self.y = b

class Solution:
    # @param {int[]} points an array of point
    # @return {int} an integer
    def maxPoints(self, points):
        # Write your code here
        l = len(points)
        if l<=2:
            return l
        
        max_count=1
        for i in range(l):
            args=[]
            if max_count > l-i:
                break
            same_p=0;
            for j in range(i+1, l):
                if points[j].x == points[i].x :
                    if points[j].y == points[i].y:
                        same_p+=1;
                    else:
                        args.append(float("inf"))    
                else:
                    args.append(float(points[j].y-points[i].y)/float(points[j].x-points[i].x))

            if len(args) == 0:
                max_count=max(max_count, same_p)
            else:
                count=same_p+1
                args.sort()    
                for k in range(1, len(args)):
                    if args[k]==args[k-1] :
                        count+=1
                    else:
                        count=same_p+1;
                    max_count=max(max_count,count)
               
        return max_count+1
