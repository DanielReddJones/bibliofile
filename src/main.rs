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

    
    screen(&contents);
    Ok(())

    
}


//screen init function
fn screen(line: &str){
    
    let mut stdscr: i8 = 0;
    let mut w: i32 = 0; //width
    let mut h: i32 = 0; //height
    ncurses::getmaxyx(&mut stdscr, &mut h, &mut w);

    ncurses::initscr();
    ncurses::clear();
    ncurses::addstr(line);
    ncurses::refresh();
    ncurses::getch();
    ncurses::endwin();

}
