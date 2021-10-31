/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/find-the-missing-number
@Language: C++
@Datetime: 15-07-20 12:02
*/

class Solution {
public:
    /**    
     * @param nums: a vector of integers
     * @return: an integer
     */
    int findMissing(vector<int> &nums) {
        // write your code here
        const int N=nums.size();
        int t=0;
        for ( auto n : nums )
            t+=n;
        return N*(N+1)/2-t;
    }
};
