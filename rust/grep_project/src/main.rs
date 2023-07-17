/*
*
* Vamos a hacer una versión de grep donde se reciben dos parámetros:
* 
*   - String con el que vamos a hacer pattern matching
*   - Path del archivo
*
* El algoritmo va a seguir los siguientes pasos:
*   1. Leer los parámetros de entrada que le pasamos a cargo
*   2. Hacer open del archivo
*   3. Analizar si la línea tiene el string deseado.
*   4. Mostrar una lista de las líneas del archivo que contienen el string.
*
*/

use std::env;

fn main() {

    //Usamos un vector de strings ya que tenemos varios strings como params.
    let args: Vec<String> = env::args().collect();
}
