/*
Program: Bibliofile
Purpose: This class is meant to process and return HTML formatted text as strings.
Last edited: 6/28/23
 */

use scraper::{Html, Selector};

pub fn main(content: String){


  let str_content = Html::parse_document(&content);

  //Selector is HTML tag. Can be <br> or <p> or anything else. To parse entire page, set selector to <html>
  let selector = Selector::parse("html").unwrap();
  let unwrapped_page = str_content.select(&selector).next().unwrap();
  let page = unwrapped_page.text().collect::<Vec<_>>();

  //every line in document is an entry into the vector. For loop iterates through every entry and displays it.
  for i in 0..page.len() {
    println!("{}", page[i]);
  }


}