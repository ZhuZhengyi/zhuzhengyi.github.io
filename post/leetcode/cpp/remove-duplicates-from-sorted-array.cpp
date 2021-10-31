/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/remove-duplicates-from-sorted-array
@Language: C++
@Datetime: 15-07-22 12:45
*/

class Solution {
public:
    /**
     * @param A: a list of integers
     * @return : return an integer
     */
    int removeDuplicates(vector<int> &nums) {
        // write your code here
        //:

        const int n = nums.size();
        if(n<2) return n;

        int t=0;
        for(int i=1; i<n; ++i){
            if(nums[i]!=nums[t])
                nums[++t]=nums[i];
        }
        return t+1;
    }
};
