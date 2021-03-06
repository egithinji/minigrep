use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {

        println!("{}", line);

    }

    Ok(())

}

pub struct Config {

    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,

}

impl Config {

    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let case_sensitive = match args.next(){
            Some(arg) => {
                    if arg == "i" {
                        false
                    }else {
                        env::var("CASE_INSENSITIVE").is_err()
                    }
            },
            None => env::var("CASE_INSENSITIVE").is_err(),

        };

        Ok(Config {query,filename,case_sensitive})

    }

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process;
    
    #[test]
    fn create_config_works() {
        let args = vec![String::from("system"),String::from("apple"),String::from("poem.txt")];
        let sample_config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


        assert_eq!("apple",sample_config.query);

    }

    #[test]
    fn run_works() {

        
        let args = vec![String::from("system"),String::from("apple"),String::from("poem.txt")];
        let sample_config = Config::new(&args).unwrap_or_else(|err| {
         println!("Problem parsing arguments: {}", err);
         process::exit(1);
         });
        
        let mut file_was_read = true;
        let mut specific_error = String::from("none");

        if let Err(e) = run(sample_config) {
            println!("**** error reading file: {}",e);
            file_was_read = false;
            specific_error = String::from(e.to_string());
        }
        
        assert!(file_was_read,specific_error);

    }

    #[test]
    fn case_sensitive() {

        let query = "duct";
        let contents = "\
Rust:\n
safe, fast, productive.\n
Pick three.\n
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));

    }

    #[test]
    fn case_insensitive() {

        let query = "rUsT";
        let contents = "\
Rust:\n
safe, fast, productive.\n
Pick three.\n
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."],
                   search_case_insensitive(query, contents)
                   );

    }

}



