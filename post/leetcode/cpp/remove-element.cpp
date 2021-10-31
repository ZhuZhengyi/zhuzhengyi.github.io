/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/remove-element
@Language: C++
@Datetime: 15-07-21 14:29
*/

class Solution {
public:
    /** 
     *@param A: A list of integers
     *@param elem: An integer
     *@return: The new length after remove
     */
    int removeElement(vector<int> &A, int elem) {
        // write your code here
        const int n=A.size();
        if (n<1) return 0;
        
        int count=0;
        //A.push_back(A[n-1]);
        //int t=0,h=0;
        for(int i=0;i<n;++i){
            if(A[i]==elem) count++;
            else{
                A[i-count]=A[i];
            }
        }
        
        A.erase(A.end()-count,A.end());
        
        return n-count;
    }
};
