/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/space-replacement
@Language: C++
@Datetime: 15-07-07 22:45
*/

class Solution {
public:
    /**
     * @param string: An array of Char
     * @param length: The true length of the string
     * @return: The true length of new string
     */
    int replaceBlank(char string[], int length) {
        // Write your code here
        int tl = 0;
        for ( int i = 0; i < length; ++i ) {
            if ( string[i] == ' ' )
                tl += 3;
            else
                tl += 1;
        }
        for ( int i = length-1, j = tl-1; i >= 0; --i ) {
            if (string[i] == ' ') {
                string[j--]='0';
                string[j--]='2';
                string[j--]='%';
            } else {
                string[j--]=string[i];
            }
        }
        return tl;
    }
};
