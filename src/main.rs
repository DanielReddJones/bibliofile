/*
Program: Bibliofile
Language: Rustc 1.69.0
ide: CLion
Operating system: Fedora 38/WSL
Purpose: ncurses based ereader and library manager for Linux terminal environments. 
Last edited: 5/19/23
*/


mod html_module;
use epub::doc::EpubDoc; //library for navigating epubs
use std::env;
use std::io;



//initial function. Reads the ebook passed by argument.
//TODO: add visual library to pull up ebooks.
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    epub_func(filename);

}


//parses epub files
fn epub_func(epub_file: &str){
    //let item_count = 1;
    let doc = EpubDoc::new(&epub_file);
    assert!(doc.is_ok());
    let mut doc = doc.unwrap();
    doc.set_current_page(1).expect("end of book");
    let mut content = doc.get_current_str();
    let mut str_content = content.unwrap();



            let page = html_module::main(str_content);
            println!("{}", page);
    
}



