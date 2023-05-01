/*
Program: Bibliofile
Language: Rustc 1.69.0
ide: Visual Studio Code
Operating system: Fedora 38/WSL
Purpose: ncurses based ereader and library manager for Linux terminal environments. 
Last edited: 5/1/23
*/

use epub::doc::EpubDoc;
use std::fs;
use std::io::prelude::*;
use std::env;
extern crate ncurses; //display framework
extern crate termsize; //makes sure ncurses border maches term size


//initial function. Reads the ebook passed by argument.
//TODO: add visual library to pull up ebooks.
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    epub(filename);

}


//parses epub files
fn epub(epub_file: &str){
    let item_count = 0;
    println!("{}", epub_file);
    let doc = EpubDoc::new(&epub_file);
    assert!(doc.is_ok());
    let doc = doc.unwrap();
    assert_eq!(105, doc.spine.len());
    let page = &doc.spine[item_count];
    let text = doc.resources.get(page);
    println!("{:?}", text);
}



