/*
Program: Bibliofile
Language: Rustc 1.69.0
ide: CLion
Operating system: Fedora 38/WSL
Purpose: This class is meant to process and return HTML formatted text as strings.
Last edited: 6/28/23
 */

use scraper::{Html, Selector};

pub fn main(content: String){


  let str_content = Html::parse_document(&content);
  let selector = Selector::parse("html").unwrap();

  let unwrapped_page = str_content.select(&selector).next().unwrap();
  let page = unwrapped_page.text().collect::<Vec<_>>();
  for i in 0..page.len() {
    println!("{}", page[i]);
  }


}