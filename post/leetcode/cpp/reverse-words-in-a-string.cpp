/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/reverse-words-in-a-string
@Language: C++
@Datetime: 15-07-07 13:43
*/

class Solution {
public:
    /**
     * @param s : A string
     * @return : A string
     */
    string reverseWords(string s) {
        // write your code here
        //std::reverse(s.begin(), s.end());
        bool flag=false;
        auto i=s.begin();
        auto j=i;
        while( j!=s.end()){
            if(*j==' '){
                std::reverse(i,j);
                i=j+1;
            }
            ++j;
        }
        std::reverse(i,j);
        
        std::reverse(s.begin(), s.begin()+s.find_last_not_of(' ')+1);
        
        return s;
    }
};
