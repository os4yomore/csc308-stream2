int main()
{
  int csc201;
  int csc205;
  int sta205;
  printf("Enter CSC201 score");
  scanf("%i", &csc201);
  printf("Enter CSC205 score");
  scanf("%i", &csc205);
  printf("Enter STA205 score");
  scanf("%i", &sta205);

  int sum = csc201+csc205+sta205;
  int avg = sum/3;

  printf("Your sum is: %i", sum);
  printf("Your average is: %i", avg);

  return 0;
  
  

}