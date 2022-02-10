use std::{cell::RefCell, rc::Rc};

use super::{Item, ItemStateTable, Tag};

static MAX_REASONABLE_COLLECTION_DEPTH: i32 = 50;
pub struct Collection {}

impl Collection {
    pub fn new(
        parent: Option<Rc<RefCell<Box<Collection>>>>,
        usage_page: u32,
        usage: u32,
        collection_type: u32,
    ) {
    }
    pub fn build(items: Vec<Item>) {
        let mut state = ItemStateTable::default();
        let mut depth = 0;
        for item in items {
            match Tag::from(item.tag) {
                Tag::Collection => {
                    depth += 1;
                    if depth <= MAX_REASONABLE_COLLECTION_DEPTH {}
                }
                _ => (),
            }
        }
    }
}

fn add_collection(item: &Item, collections: &mut Vec<Collection>, state: &mut ItemStateTable) {
    let usage = state.local.usages.first().unwrap_or(&0);
    let mut usage_page = (usage >> 16) & 0xffff;
    if let Some(last) = state.global_stack.last() {
        if usage_page == 0 {
            usage_page = last.usage_page;
        }
    }
    let collection_type = item.short_data;
}
