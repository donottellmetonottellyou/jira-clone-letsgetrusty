mod db;
mod io_utils;
mod models;
mod navigator;
mod ui;

use db::*;
use io_utils::*;
use navigator::*;

use std::rc::Rc;

fn main() {
    // TODO: create database and navigator

    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        // 2. render page
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action
    }
}
