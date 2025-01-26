#include <stdio.h>

#define ROWS 3
#define COLS 3

int main() {


    int matrix1[ROWS][COLS];
    int matrix2[ROWS][COLS];
    

    int result[ROWS][COLS];
    int index1;
    int index2;
    
    for (int i = 0; i < ROWS; i++) {
        for (int j = 0; j < COLS; j++) {
          printf("Input matrix element index [%i], [%i]\n", i, j);
            scanf("%i", &index1);
            matrix1[i][j] = index1;
        }}

        for (int i = 0; i < ROWS; i++) {
        for (int j = 0; j < COLS; j++) {
            printf("Input matrix element index [%i], [%i]\n", i, j);
            scanf("%i", &index2);
            matrix2[i][j] = index2;
        }}

         for (int i = 0; i < ROWS; i++) {
        for (int j = 0; j < COLS; j++) {
            result[i][j] = matrix1[i][j] + matrix2[i][j];
        }
    }
    




    printf("The sum of the two matrices is:\n");
    for (int i = 0; i < ROWS; i++) {
        for (int j = 0; j < COLS; j++) {
            printf("%d ", result[i][j]);
        }
        printf("\n");
    }

    return 0;
}



