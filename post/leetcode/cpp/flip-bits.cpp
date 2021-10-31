/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/flip-bits
@Language: C++
@Datetime: 15-08-06 11:32
*/

class Solution {
public:
    /**
     *@param a, b: Two integer
     *return: An integer
     */
    int bitSwapRequired(int a, int b) {
        // write your code here  
        unsigned int  flag = 1;  
        int i =0;  
        int j = 0;  
        int c= 0;  
        while (flag) {  
            i = a & flag;  
            j = b & flag;  
           if (i ^ j) {  
               c++;  
           }  
            flag = flag << 1;  
        }  
        return c;  
    }
};
