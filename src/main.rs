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
use std::process::exit;


//initial function. Reads the ebook passed by argument.
//TODO: add visual library to pull up ebooks.
fn main() {
    if env::args().len() == 1 {
        println!("you need to enter a book. Closing program.");
    }
    else {
        let args: Vec<String> = env::args().collect();
        let filename = &args[1];

        epub_func(filename);
    }
}


//parses epub files
fn epub_func(epub_file: &str){

    let doc = EpubDoc::new(&epub_file);
    assert!(doc.is_ok());
    let mut doc = doc.unwrap();

    let mut page_num = 1;
    let is_reading = true;
    while is_reading == true {
        let mut next_or_last = String::new();
        doc.set_current_page(page_num);

        let content = doc.get_current_str();
        let str_content = content.unwrap();
        html_module::main(str_content.0);


        let input_size = std::io::stdin().read_line(&mut next_or_last);
        let input_size_len = input_size.unwrap() - 1;
        if input_size_len == 1{

            /*
            Was not sure how to compare string to input from .read_line
            Instead, I felt that it would be easier to convert it to bytes and
            compare the ASCII values.
             */
            let compare = next_or_last.as_bytes();

            //If user presses n(for next) goes forward one page.
            if compare[0] == 110  {
                page_num = page_num + 1;
            }
            //if user presses b, goes back one page.
            else if compare[0] == 98 {
                //if page number equals one then you are at the beginning of the book.
                if page_num == 1 {
                    println!("at beginning of book.");
                }
                else {
                    page_num = page_num - 1;
                }
            }
                //If user presses q(for quit), exits with status code of 0.
                else if compare[0] == 113 {
                    println!("quitting...");
                    exit(0);
                }
            else {
                println!("did not understand command.");
            }
        }
        else {
            println!("Do not understand input. Try again.");
        }
    }
}



