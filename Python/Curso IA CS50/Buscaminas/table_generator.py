"""

Para generar el tablero, se va a pasar el número de bombas con el que se va a jugar y se van a generar 10
posiciones del tablero diferentes
Tras ello, se actualiza cada posicion del tablero con un valor, referente al numero de bombas que tiene a su
alrededor.

"""

from random import randrange

#Metodo generador de tablero
def generar_tablero():
    #Inicializar tablero
    tablero = []
    
    #Rellenar tablero con 0s y 'o' 
    for x in range(0,10):
        fila = []
        for y in range(0,10):
            fila.append((0, 'o'))
        tablero.append(fila)

    #Obtener las posiciones de las bombas
    bombas = generar_bombas() 

    #Colocar las bombas
    for tupla in bombas:
        tablero[tupla[0]][tupla[1]] = (-1, 'o')
    
    #Actulizar el tablero
    tablero = actualizar_tablero(tablero)
    return tablero

#Generar N posiciones aleatorias
def generar_bombas():
    #Inicializar la lista
    posiciones = []

    #Generar 10 tuplas correspondientes a las posiciones sin que se repita
    for i in range(0,10):
        #Crear la tupla
        pos_tuple = (randrange(10), randrange(10))

        #Comprobar que no esté en la lista
        while posiciones.count(pos_tuple) > 0:
            pos_tuple = (randrange(10), randrange(10))
    
        #Añadirla a la lista
        posiciones.append(pos_tuple)

    #Devolver la lista
    return posiciones

#Actualizar posiciones tablero
def actualizar_tablero(tablero):
    for x in range(0,10):
        for y in range(0,10):
            #Comprobamos que la casilla no sea una bomba
            if tablero[x][y][0] != -1:
                tablero[x][y] = actualizar_celda(tablero, x, y)
    return tablero

def actualizar_celda(tablero, x, y):
    n_bombas = 0
    #Recorremos una matriz de 9x9 empezando en (x-1, y-1) y terminando en (x+1, y+1)
    for i in range(x-1,x+2):
        #Comprobamos que sea una fila válida
        if i >= 0 and i <= 9:
            for j in range(y-1, y+2):
                #Comprobamos que sea una columna válida
                if j >= 0 and j <= 9:
                    #Comprobamos que sea bomba
                    if tablero[i][j][0] == -1:
                        n_bombas += 1
    #Devolvemos el nuevo valor
    return (n_bombas, 'o')
