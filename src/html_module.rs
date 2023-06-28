/*
Program: Bibliofile
Language: Rustc 1.69.0
ide: CLion
Operating system: Fedora 38/WSL
Purpose: This class is meant to process and return HTML formatted text as strings.
Last edited: 6/28/23
 */

use scraper::{Html, Selector};

pub fn main(content: String) -> String {
    let str_content = content;

    println!("{}", str_content);

    let page = str_content;

    return page;
}