from os import close
from sys import argv

#Guardamos en $text_file el nombre del archivo
text_file = argv[1]

#Abrimos el archivo
file = open(text_file)

#Creamos un array de vocales donde 0 -> a, 1 -> e, 2 -> i, 3 -> o y 4 -> u
vocales = [0, 0, 0, 0, 0]

#Recorremos linea a linea el fichero
for line in file:

    #Recorremos la linea caracter a caracter
    for letter in line:

        if letter == 'a':
            vocales[0] += 1
        elif letter == 'e':
            vocales[1] += 1
        elif letter == 'i':
            vocales[2] += 1
        elif letter == 'o':
            vocales[3] += 1
        elif letter == 'u':
            vocales[4] += 1
print(vocales)

#Cerramos el archivo
file.close()

#Buscamos las vocales que más se repiten
mas_reps = [0]
for i in range(1,5,1):
    if vocales[i] > vocales[mas_reps[0]]:
        mas_reps.clear()
        mas_reps.append(i)
    elif vocales[i] == vocales[mas_reps[0]]:
        mas_reps.append(i)

#Printeamos la lista
print(mas_reps)



