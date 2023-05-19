/*
Program: Bibliofile
Language: Rustc 1.69.0
ide: CLion
Operating system: Fedora 38/WSL
Purpose: This class is meant to process and return HTML formatted text as strings.
Last edited: 5/19/23
 */

use soup::{NodeExt, QueryBuilderExt, Soup};

pub fn main(content: String) -> String {
    let mut str_content = content;
    let soup = Soup::new(&str_content);
    let page = soup.text();
    return page;
}