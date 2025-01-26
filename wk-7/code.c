#include <stdio.h>

int main()
{
 int num1[3]= {2, 4, 7};
 int num2[3]= {4, 9, 1};
 int sum[3]= {0, 0, 0};
 int *element = num1;
 int *element2 = num2;
 int *great = sum;


 for(int i = 0; i<3; i++)
 {

  *great = *element + *element2;
  printf("%i\n", *great);
  element++;
  element2++;
  great++;


  


  
 }

 return 0;
 


}