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
use grep_project::*;
use std::fs::File;
fn main() {

    //Usamos un vector de strings ya que tenemos varios strings como params.
    let args = get_params();
  
    let file = File::open(&args[2]).expect("ERROR: File not found");

    let res_lines = filter_file(&args[1], file);
    
    show_lines(&res_lines);
}
