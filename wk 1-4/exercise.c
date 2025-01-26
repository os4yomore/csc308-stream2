#include <stdio.h>


int main()
{
  int age;
  printf("Please Enter your age");
  scanf("%i", &age);
  if (age >=18)
  {
    printf("You are eligible to vote");
  }
  else 
  {
    printf("You cannot vote");
  }

  return 0;

}