#include <stdio.h>

int counting(int lista[], int n);
void main(void)
 {
  int i, lista[] = {2, 5, 3, 2, 7, 5, 3, 2, 2};
  int n = sizeof(lista)/sizeof(lista[0]);
  counting(lista, n);
 }
int counting(int lista[], int n)
 {
  int i, j, x, y, m;
  int min = lista[0];
  int max = lista[0];
  int aux[max];
  for(i = 0; i < n; i++)
   {
    if(lista[i] > max)
     {
      max = lista[i];
     }
    if(lista[i] < min)
     {
      min = lista[i];
     }
   }
  for(j = 0; j <= max; j++)
   {
    aux[j] = 0;
   }
  for(x = min; x <= max; x++)
   {
    for(y = 0; y < n; y++)
     {
      if(lista[y] == x)
       {
        aux[x] = aux[x] + 1;
       }
     }
   }
  int r = 1, o = 0, ord[n];
  for(m = min; m <= max; m++)
   {
    printf("%i", aux[m]);
    if(aux[m] > 0)
     {
      while(r <= aux[m])
       {
        ord[o] = m;
        r = r + 1;
        o = o + 1;
       }
     }
   }
  i = 0;
  for(i=0;i<n;i++)
   {
    //printf("%i ",ord[i]);
   }
 }
