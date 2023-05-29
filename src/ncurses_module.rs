/*
Program: Bibliofile
Language: Rustc 1.69.0
ide: CLion
Operating system: Fedora 38/WSL
Purpose: This module is meant to render the book text within ncurses.
Last edited: 5/20/23
 */


use ncurses;
use ncurses::{initscr, WINDOW};

pub fn main(mut text: String) {

    ncurses::clear();
    initscr();
    ncurses::addstr(&*text);

    ncurses::refresh();
    println!("If you can read this, you are in the ncurses_module");
}