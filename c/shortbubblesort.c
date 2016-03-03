#include <stdio.h>

int bubblesort(int lista[], int n);
void main(void)
 {
  int i, lista[10] = {20,30,40,90,50,60,70,80,100,110};
  bubblesort(lista,10);
  for(i=0;i<10;i++)
   {
    printf("%i ",lista[i]);
   }
 }
int bubblesort(int lista[],int n)
 {
  n=n-1;
  int temp, i, c = 1;
  while(n > 0 && c == 1)
   {
    c = 0;
    for (i=0;i<n;i++)
     {
      if(lista[i]>lista[i+1])
       {
        c = 1;
        temp = lista[i];
        lista[i] = lista[i+1];
        lista[i+1] = temp;
       }
     }
    n = n-1;
   }
 }
