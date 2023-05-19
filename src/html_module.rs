/*
Program: Bibliofile
Language: Rustc 1.69.0
ide: CLion
Operating system: Fedora 38/WSL
Purpose: This class is meant to process and return HTML formatted text as strings.
Last edited: 5/18/23
 */


use regex::Regex;
use epub::archive;
use soup::{NodeExt, QueryBuilderExt, Soup};


pub fn main(content: String) -> String{

    println!("IF YOU CAN READ THIS, I HAVE ENTERED THE HTML MODULE");


    let mut str_content = content;
    let soup = Soup::new(&str_content);
    let results = soup.tag(true)
                            .find_all()
                            .map(|tag| tag.name().to_string())
                            .collect::<Vec<_>>();
    assert_eq!(results, vec![
        "html".to_string(),
        "head".to_string(),
        "body".to_string(),
        "p".to_string(),
        "b".to_string(),
    ]);

    return results.join("\n");
}