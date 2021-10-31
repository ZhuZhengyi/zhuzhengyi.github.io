/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/longest-common-substring
@Language: C++
@Datetime: 15-07-21 12:57
*/

class Solution {
public:    
    /**
     * @param A, B: Two string.
     * @return: the length of the longest common substring.
     */
    int longestCommonSubstring(string &A, string &B) {
        // write your code here
        const int sizeA=A.size();
        const int sizeB=B.size();
        int AB[sizeA+1][sizeB+1];
        
        int length=0;
        for(int i=0; i<sizeA; ++i){
            for(int j=0; j<sizeB; ++j){
                if(i==0||j==0) AB[i][j]=0;
                if(A[i]==B[j]){
                    AB[i+1][j+1]=AB[i][j]+1;
                }else{
                    AB[i+1][j+1]=0;
                }
                if (length<AB[i+1][j+1]) {
                    length=AB[i+1][j+1];
                }
            }
        }
        
        return length;
        
    }
};

