/*
Program: Bibliofile
Language: Rustc 1.69.0
ide: CLion
Operating system: Fedora 38/WSL
Purpose: This class is meant to process and return HTML formatted text as strings.
Last edited: 5/20/23
 */
//TODO: I received a warning that soup is going to be incompatible in a future version, as it uses an old version of html5ever.
/*Possible solutions:
 - Convert to html5ever
 - find alternative library/framework
 - convert html module to python script, use python version of soup
 - convert html by hand(absolutely not)
 */
use soup::{Soup};

pub fn main(content: &str) -> String {
    let str_content = content;
    let soup = Soup::new(&str_content);
    let page = soup.text();
    return page;
}