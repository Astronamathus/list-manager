use colored::*;
use crate::models::{Item, List};

pub fn create_list(lists: &mut Vec<List>, name: String) {
    let list = List {
        name: name.clone(),
        items: vec![],
    };

    lists.push(list);

    println!("{}", format!("Created list '{}'", name).green());
}

pub fn add_item(lists: &mut Vec<List>, list_name: String, item_text: 
String) {
    for list in lists {
        if list.name == list_name {
            list.items.push(Item {
                text: item_text.clone(),
                completed: false,
            });

            println!("{}", "Item added!".green());
            return;
        }
    }

    println!("{}", "List not found.".red());
}

pub fn view_lists(lists: &Vec<List>) {
    for list in lists {
        println!(
            "\n{}",
            format!("== {} ==", list.name)
                .bright_blue()
                .bold()
        );

        for (i, item) in list.items.iter().enumerate() {
            let status = if item.completed {
                "[x]".green()
            } else {
                "[ ]".yellow()
            };

            println!("{} {} {}", i + 1, status, item.text);
        }
    }
}
