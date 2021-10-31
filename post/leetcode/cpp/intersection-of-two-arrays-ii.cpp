/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/intersection-of-two-arrays-ii
@Language: C++
@Datetime: 17-01-01 12:46
*/


#include <algorithm>

class Solution {
public:
    /**
     * @param nums1 an integer array
     * @param nums2 an integer array
     * @return an integer array
     */
    vector<int> intersection(vector<int>& nums1, vector<int>& nums2) {
        // Write your code here
        
        auto l1 = nums1.size();
        auto l2 = nums2.size();
        
        vector<int> res;
        
        if (l1 <=0 || l2 <= 0) {
            return res;
        }
        
        sort(nums1.begin(), nums1.end());
        sort(nums2.begin(), nums2.end());
        
        int i1 = 0;
        int i2 = 0;
        while (i1<l1 && i2<l2) {
            if (nums1[i1] == nums2[i2]) {
                res.push_back(nums1[i1]);
                i1++;
                i2++;
            } else if (nums1[i1] < nums2[i2]) {
                i1++;
            } else {
                i2++;
            }
        }
        
        return res;
    }
};