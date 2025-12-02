use std::fs;
use  std::io::Error;


fn string_test(a: String, b: &String, c: &str) {
    println!("string_test : {}, {} , {}", a, b, c);
}

fn extract_errors(text: &str) -> Vec<String> {
    let split_text: Vec<&str> = text.split("\n").collect();
    let mut results = vec![];
    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }
    results
}
fn main() -> Result<(),Error> {
    string_test(String::from("first"), &String::from("second"), &String::from("third"));

    //use match
    /*match  fs::read_to_string("logs.txt") {
        Ok(file) => {
            let error_logs = extract_errors(file.as_str());
            match fs::write("errors.txt", error_logs.join("\n")) {
                Ok(()) => {println!("done");},
                Err(e) => {
                    println!("Error writing to logs.txt file: {}", e);
                }
            }
            println!("Error logs is: {:#?}", error_logs);
        }
        Err(e) => {
            println!("It failed because {}", e)
        }
    }*/

    /*Use expect
    let text = fs::read_to_string("logs.txt")
                        .expect("Failed to read.logs.txt");
    let results = extract_errors(&text);
    fs::write("errors.txt", results.join("\n")).expect("Failed to write errors.txt");*/
    
    //Use ? Operator or try operator
    let text = fs::read_to_string("logs.txt")?;
    let results = extract_errors(text.as_str());
    fs::write("errors.txt", results.join("\n"))?;
    Ok(())

}
