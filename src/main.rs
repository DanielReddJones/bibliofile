/*
Program: Bibliofile
Language: Rustc 1.68.0
ide: Visual Studio Code
Operating system: Linux Mint 21.1
Purpose: ncurses based ereader and library manager for Linux terminal environments. 
Last edited: 10:18 PM 3/16/23
*/

use std::fs::File;
use std::io::prelude::*;
use std::env;
extern crate ncurses;


//initial function. Reads the ebook passed by argument.
//TODO: add visual library to pull up ebooks.
fn main() -> std::io::Result<()>  {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    
    let mut file = File::open(filename).expect("cannot open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("unreadable file.");

    println!("{}", contents);
    screen();
    Ok(())

    
}


//screen init function
fn screen(){

    ncurses::initscr();
    ncurses::addstr("Hello World!");
    ncurses::refresh();
    ncurses::getch();
    ncurses::endwin();

}
