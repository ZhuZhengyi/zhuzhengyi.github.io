/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/longest-common-prefix
@Language: C++
@Datetime: 15-07-21 12:01
*/

class Solution {
public:    
    /**
     * @param strs: A list of strings
     * @return: The longest common prefix
     */
    string longestCommonPrefix(vector<string> &strs) {
        // write your code here
        const int n=strs.size();
        if(n<1)
            return "";
        if(n==1)
            return strs[0];
        
        const int l=strs[0].size();
        
        for(int i=0; i<l; ++i){
            for (int j=1; j<n; ++j ){
                if( i>strs[j].size()||strs[j][i]!=strs[0][i]){
                    return strs[0].substr(0,i);
                }
            }
        }
        
        return strs[0];
    }
};
