/*
Program: Bibliofile
Language: Rustc 1.68.0
ide: Visual Studio Code
Operating system: Linux Mint 21.1
Purpose: ncurses based ereader and library manager for Linux terminal environments. 
Last edited: 10:18 PM 3/16/23
*/

use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::env;
extern crate ncurses;


//initial function. Reads the ebook passed by argument.
//TODO: add visual library to pull up ebooks.
fn main() {
    //let args: Vec<String> = env::args().collect();
    //let filename = &args[1];
    menu();

}

fn get_terminal_size()-> (i8, i32, i32){

    let mut stdscr: i8 = 0;
    let mut w: i32 = 0; //width
    let mut h: i32 = 0; //height
    ncurses::getmaxyx(&mut stdscr, &mut h, &mut w);

    return (stdscr, h, w);
}



//Gets files in directory and returns string with contents.
fn get_directory() -> String{

    let mut dir: String = "".to_string();
    let mut hold: String;
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        hold = path.unwrap().path().display().to_string();
        dir.push_str(&hold);
        dir.push_str("\n");
    }
    println!("{}", &dir);
    return dir;
}

fn menu(){
 
    let (mut stdscr, height, width): (i8, i32, i32) = get_terminal_size();
    
    ncurses::initscr();
    ncurses::clear();
    let win = ncurses::newwin(100, 100, 5, 5);
    ncurses::wprintw(win, &get_directory());
    ncurses::refresh();
    ncurses::box_(win, 0, 0);
    ncurses::wrefresh(win);
    ncurses::getch();
    ncurses::endwin();
    

}

//screen init function
fn screen(line: &str){

    ncurses::initscr();
    ncurses::clear();
    ncurses::addstr(line);
    ncurses::refresh();
    ncurses::getch();
    ncurses::endwin();

}



//parses UTF8 files. Does not work if it has BOM or CRLF line terminators. Currently searching for fix.
fn text(filename: &str) -> std::io::Result<()>{
    
    
    let mut file = File::open(filename).expect("cannot open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("unreadable file.");

    
    screen(&contents);
    Ok(())


}


//parses pdf files
fn pdf(){


}


//parses mobi files
fn mobi(){



}


//parses epub files
fn epub(){


}



