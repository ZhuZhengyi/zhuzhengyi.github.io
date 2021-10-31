/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/first-missing-positive
@Language: C++
@Datetime: 15-07-23 14:30
*/

class Solution {
public:
    /**    
     * @param A: a vector of integers
     * @return: an integer
     */
    int firstMissingPositive(vector<int> A) {
        // write your code here
        const int n=A.size();
        if(n<1) return 1;
        
        for(int i=0; i<n; ){
            if(A[i]>0&&A[i]<n&&A[i]!=A[A[i]-1]){
                swap(A[i], A[A[i]-1]);
            }else{
                ++i;
            }
        }
        
        int i=0;
        int r=0;
        for(i=0; i<n; ++i){
            if(A[i]!=i+1)
                return i+1;
        }

        return n+1;
    }
};
