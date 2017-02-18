def counting(lista):
    min = lista[0]
    max = lista[0]
    for i in range(len(lista)):
        if lista[i] > max:
            max = lista[i]
        if lista[i] < min:
            min = lista[i]
    aux = [0] * (max + 1)
    for x in range(min, max+1):
        for y in range(len(lista)):
            if lista[y] == x:
                aux[x] = aux[x] + 1
    ord = []
    for m in range(min, max+1):
        ord += [m] * aux[m]
    print("Lista ordenada: ", ord)
lista = [2, 5, 3, 2, 7, 5, 3, 2, 2]
print("Lista sin ordenar: ", lista)
counting(lista)
