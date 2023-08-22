#include <stdio.h>

int main(){
    int a = 1;
    int b = 2;
    int c = 3;
    int d = 4;
    int e = 5;

    if(a > b){
        if(c > d){
            printf("a > b and c > d\n");
        }else{
            printf("a > b and c <= d\n");
        }
    }else{
        if(e > d){
            printf("a <= b and e > d\n");
        }else{
            printf("a <= b and e <= d\n");
        }
    }
    return 0;
}