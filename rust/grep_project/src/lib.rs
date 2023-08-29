pub fn get_params() -> Vec<String> {
    use std::env;
    env::args().collect()
}

use std::fs::File;
pub fn filter_file(pattern: &String, file: File) -> Vec<String>{
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

pub fn show_lines(res: &Vec<String>){
    for line in res {
        println!("{}", line);
    }
}

pub mod setup {
    use std::fs::File;
    pub fn setup_file() -> File {
        let file = File::create("test.txt").expect("create failed");
        file
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use super::*;
    use setup::setup_file;

    #[test]
    fn filter_test() {
        let mut file = setup_file();
        file.write_all(b"line1\nline2\nline3\n").expect("write failed");
        let pattern = String::from("line");
        let vec = filter_file(&pattern, file);
        assert_eq!(vec[0], "line1");
        assert_eq!(vec[1], "line2");
        assert_eq!(vec[2], "line3");
    }
}

