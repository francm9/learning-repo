from table_generator import generar_tablero

#Se crea el tablero
tablero = generar_tablero()

#Metodo que abre una celda
def abrir_celda(x, y, counter):
    #Si es una bomba pierde la partida
    if tablero[x][y][0] == -1:
        tablero[x][y] = (-1, 'x')

    #Si no es bomba abre las casillas que correspondan
    else:
        if tablero[x][y][1] != "ob":
            tablero[x][y] = (tablero[x][y][0], 'x')
            counter += 1
            if tablero[x][y][0] == 0:
                for i in range(x-1,x+2):
                    #Comprobamos que sea una fila válida
                    if i >= 0 and i <= 9:
                        for j in range(y-1, y+2):
                            #Comprobamos que sea una columna válida
                            if j >= 0 and j <= 9:
                                #Comprobamos que la celda no se haya abierto y que no sea una bandera
                                if tablero[i][j][1] == "o":
                                    counter = abrir_celda(i, j, counter)  
    return counter
#Metodo que muestra el tablero
def mostrar_tablero():
    #Mostrar el numero de columna
    print("  0 1 2 3 4 5 6 7 8 9")

    #Mostrar separacion
    print("+---------------------+")
    for x in range(0,10):
        print_line = "| "
        for y in range(0,10):
            if tablero[x][y][1] == 'x':
                if tablero[x][y][0] > 0:
                    print_line += (str(tablero[x][y][0]) + " ")
                elif tablero[x][y][0] == 0:
                    print_line += "  "
                else:
                    print_line += "b "
            elif tablero[x][y][1] == "ob":
                print_line += "> "
            else:
                print_line += "x "
        print_line += "| " + str(x)
        print(print_line)
    print("+---------------------+")

def play():
    celdas_counter = 0
    fin = False
    while not fin:

        #Introduce una accion correcta
        action = int(input("0: Abrir celda\n1: Colocar bandera\nElige la acción a realizar: "))
        while action > 1 or action < 0:
            action = int(input("El dato que has introducido en el mundo es incorrecto. Introducelo de nuevo: "))

        #Introduce una fila correcta    
        x = int(input("Introduce la fila: "))
        while x > 9 or action < 0:
            x = int(input("El dato que has introducido en el mundo es incorrecto. Introducelo de nuevo: "))

        #Introduce una columna correcta
        y = int(input("Introduce la columna: "))
        while y > 9 or action < 0:
            y = int(input("El dato que has introducido en el mundo es incorrecto. Introducelo de nuevo: "))
        
        #Se ejecuta la accion introducida    
        if action:
            colocar_bandera(x, y)
        else:
            celdas_counter = abrir_celda(x, y, celdas_counter)

        #Se muestra el tablero    
        mostrar_tablero()

        #Se comprueba si ha finalizado la partida
        fin = check_fin(x, y, celdas_counter)


#Metodo que se utiliza para colocar una bandera
def colocar_bandera(x, y):
    tablero[x][y] = (tablero[x][y][0], "ob")

#Comprueba cuando acaba la partida, es decir, que se hayan abierto todas las celdas menos las de las bombas o que 
#se haya abierto una celda con bomba
def check_fin(x, y, counter):
    fin = (tablero[x][y][0] == -1 and tablero[x][y][1] == 'x') or counter == 90
    if counter == 90:
        print("You win")
    return fin

