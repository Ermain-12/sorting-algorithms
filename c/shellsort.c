#include <stdio.h>

int shellsort(int lista[], int n);
int insercion(int lista[], int inicio, int incremento, int n);

int main()
 {
  int i, lista[] = {54, 26, 93, 17, 77, 31, 44, 55, 20};
  int n = sizeof(lista)/sizeof(lista[0]);
  shellsort(lista, n);
  for(i=0; i<n; i++)
   {
    printf("%i\n", lista[i], n);
   }
 }
int shellsort(int lista [], int n)
 {
  int i, sn = n / 2;
  while(sn > 0)
   {
    for(i=0; i<sn; i++)
     {
      insercion(lista, i, sn, n);
     }
    sn = sn / 2;
   }
 }
int insercion(int lista[], int inicio, int incremento, int n)
 {
  int i, valor, posicion;
  int x = inicio+incremento;
  for(i=x; i<n; i++)
   {
    valor = lista[i];
    posicion = i;
    while(posicion >= incremento && lista[posicion-incremento] > valor)
     {
      lista[posicion] = lista[posicion-incremento];
      posicion = posicion - incremento;
     }
    lista[posicion] = valor;
   }
 }
