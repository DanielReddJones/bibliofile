use std::fs::File;
use std::io::prelude::*;
use std::env;
extern crate ncurses;

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

fn screen(){

    ncurses::initscr();
    ncurses::printw("Hello World!");
    ncurses::refresh();
    ncurses::getch();
    ncurses::endwin();

}
