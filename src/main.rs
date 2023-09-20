mod db;
mod io_utils;
mod models;
mod navigator;
mod ui;

use db::*;
use io_utils::*;
use models::Action::NavigateToPreviousPage;
use navigator::*;

use anyhow::anyhow;

use std::rc::Rc;

fn main() {
    let db = Rc::new(JiraDatabase::new("data/db.json".to_string()));
    let mut navigator = Navigator::new(db);

    while let Some(page) = navigator.get_current_page() {
        clearscreen::clear().unwrap();

        if let Err(error) = page.draw_page() {
            println!("Error rendering page:\n{error}");
            println!("Press enter to continue...");
            wait_for_key_press();

            navigator.handle_action(NavigateToPreviousPage).ok();
            continue;
        }

        match page.handle_input(&get_user_input()) {
            Ok(Some(action)) => navigator.handle_action(action),
            Ok(None) => Err(anyhow!("Invalid input.")),
            Err(e) => Err(e),
        }
        .map_err(|e| {
            println!("Error handling input:\n{e}");
            println!("Press enter to continue...");
            wait_for_key_press();
        })
        .ok();
    }
}
