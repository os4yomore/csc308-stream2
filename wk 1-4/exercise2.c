#include <stdio.h>


int add()
{
  int a;
  int b;
  int sum;
  printf("Input number 1");
  scanf("%i", &a);
  printf("Input number 2");
  scanf("%i", &b);
  sum = a + b;
  printf("Your sum is:%i", sum);
  return 0;



}
int sub()
{
  int a;
  int b;
  int sum;
  printf("Input number 1");
  scanf("%i", &a);
  printf("Input number 2");
  scanf("%i", &b);
  sum = a - b;
  printf("Your difference is:%i", sum);
  return 0;



}
int mul()
{
  int a;
  int b;
  int sum;
  printf("Input number 1");
  scanf("%i", &a);
  printf("Input number 2");
  scanf("%i", &b);
  sum = a * b;
  printf("Your product is:%i", sum);
  return 0;



}
int div()
{
  int a;
  int b;
  int sum;
  printf("Input number 1");
  scanf("%i", &a);
  printf("Input number 2");
  scanf("%i", &b);
  if (b == 0) {printf("no no no chica");}
  sum = a / b;
  printf("Your sum is:%i", sum);
  return 0;



}

int main()
{
  int choice;
  printf("What would you like to do?");
  printf("1- addition\n 2- subtraction\n 3- multiplication\n 4-division\n");
  scanf("%i", &choice);
  if (choice == 1)
  {
    add();
  }
  if (choice == 2)
  {
    sub();
  }
  if (choice == 3)
  {
    mul();
  }
  if (choice == 4)
  {
    div();
  }
  else
  {
    printf("Invalid input");
  }
 

  return 0;

}