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


use std::fs::File;

fn get_params() -> Vec<String> {
    use std::env;
    env::args().collect()
}

fn filter_file(pattern: &String, file: File) -> Vec<String>{
    let mut text_lines: Vec<String> = vec![];
    
    use std::io::{self, BufRead};
    
    let file_lines: io::Result<io::Lines<io::BufReader<File>>> = Ok(io::BufReader::new(file).lines());

    if let Ok(lines) = file_lines {
        for line in lines {
            if let Ok(line) = line {
                if line.contains(pattern) {
                    text_lines.push(line);
                }
            }
        }
    }

    text_lines
}

fn show_lines(res: &Vec<String>){
    for line in res {
        println!("{}", line);
    }
}


fn main() {

    //Usamos un vector de strings ya que tenemos varios strings como params.
    let args = get_params();
  
    let file = File::open(&args[2]).expect("ERROR: File not found");

    let res_lines = filter_file(&args[1], file);
    
    show_lines(&res_lines);
}
