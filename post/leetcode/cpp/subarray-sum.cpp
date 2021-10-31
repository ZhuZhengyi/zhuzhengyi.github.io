/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/subarray-sum
@Language: C++
@Datetime: 15-08-23 07:18
*/

class Solution {
public:
    /**
     * @param nums: A list of integers
     * @return: A list of integers includes the index of the first number
     *          and the index of the last number
     */
    vector<int> subarraySum(vector<int> nums){
        // write your code here
        const int n=nums.size();
        vector<int> res(2,0);
        for (int i=0; i<n; ++i){
            int sum=nums[i];
            res[0]=i;
            for(int j=i+1; j<n; ++j){
                if (sum+nums[j] == 0){
                    res[1]=j;
                    return res;
                }else{
                    sum+=nums[j];
                }
            }
        }

        return res;

    }
};
