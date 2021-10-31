/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/product-of-array-exclude-itself
@Language: C++
@Datetime: 15-07-22 13:15
*/

class Solution {
public:
    /**
     * @param A: Given an integers array A
     * @return: A long long array B and B[i]= A[0] * ... * A[i-1] * A[i+1] * ... * A[n-1]
     */
    vector<long long> productExcludeItself(vector<int> &nums) {
        // write your code here
        vector<long long> res;
        const int n=nums.size();
        for(int i=0; i<n; ++i){
            long long t=1;
            for(int j=0; j<n; ++j)
                if(j!=i)t*=nums[j];
            res.push_back(t);
        }
        return res;
    }
};
