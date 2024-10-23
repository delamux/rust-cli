use std::fs;
use std::error::Error;


pub fn run(params: Params) -> Result<(), Box<dyn Error>> {
    println!("Searching for: {}\n", params.query);

    let file_content: String = fs::read_to_string(params.file_path)?;
    //find the query in the string file_caontent
    let results = search(&params.query, &file_content);

    print!("We found the quantity of lines that contain the query: {}\n", results.len());

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents.lines()
        .map(|line: &str| line.trim())  
        .filter(|line: &&str| line.contains(query))
        .collect();
}

pub struct Params {
    pub query: String,
    pub file_path: String,
}

impl Params {
    pub fn new(args: &[String]) -> Result<Params, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Params { query, file_path })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive. 
            Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}