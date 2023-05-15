/*
Program: Bibliofile
Language: Rustc 1.69.0
ide: CLion
Operating system: Fedora 38/WSL
Purpose: ncurses based ereader and library manager for Linux terminal environments. 
Last edited: 5/15/23
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
    epub(filename);

}


//parses epub files
fn epub(epub_file: &str){
    let item_count = 1;
    let doc = EpubDoc::new(&epub_file);
    assert!(doc.is_ok());
    let mut doc = doc.unwrap();
    doc.set_current_page(50);
    let content = doc.get_current_str();
    
    //while true{
        let mut next = String::new();

        io::stdin()
        .read_line(&mut next)
        .expect("Failed to read line");
        //if next == "n"{
            doc.go_next();
            println!("{:?}", content);
        //}

        html_module::main();
        
    //}
    
    
    
}



