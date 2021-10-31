/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/assignment-operator-overloading-c-only
@Language: C++
@Datetime: 15-08-16 06:57
*/

class Solution {
public:
    char *m_pData;
    Solution() {
        this->m_pData = NULL;
    }
    Solution(char *pData) {
        this->m_pData = pData;
    }

    // Implement an assignment operator
    Solution& operator=(const Solution &object) {
        // write your code here
        if (this==&object){
            return *this; 
        }
        
        if (object.m_pData!=NULL){
          
            int n=strlen(object.m_pData);
            char *t=new char[n+1];
            if (t){
                delete [] this->m_pData; 
                this->m_pData=t;
                strcpy(this->m_pData, object.m_pData);
            }
        }
        
        return *this;
    }
};
