from logic import *
from game import *

"""

DESCRIPCIÓN DEL FUNCIONAMIENTO DEL AGENTE
- Tenemos una base de conocimiento donde cada variable es una tupla de coordenadas.
- En ella tenemos conjuntos de celdas donde hay una cantidad n de bombas.
- Cada vez que abrimos una casilla y no es bomba, se añade a la base de conocimiento
- El agente interactúa mediante la terminal, donde nosotros le indicamos que ejecute otra accion
- El primer movimiento siempre es abrir una celda random
- Despues de abrir una celda, si no se pierde, se actualiza la base de conocimiento

"""

#Se genera el tablero donde se va a jugar
tablero = generar_tablero()

#Metodo donde va a jugar el agente
def agent_play():
    None

#Crea una base de conocimiento
def create_knowledge():
    None
