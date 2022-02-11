use std::{cell::RefCell, rc::Rc};

use super::{Item, ItemStateTable, Tag};

static MAX_REASONABLE_COLLECTION_DEPTH: i32 = 50;
pub struct Collection {
    parent: Option<Rc<RefCell<Box<Collection>>>>,
    children: Vec<Rc<RefCell<Box<Collection>>>>,
}

impl Collection {
    pub fn new(
        parent: Option<Rc<RefCell<Box<Collection>>>>,
        usage_page: u32,
        usage: u32,
        collection_type: u32,
    ) -> Rc<RefCell<Box<Collection>>> {
        let collection = Collection {
            parent,
            children: Vec::new(),
        };
        Rc::new(RefCell::new(Box::new(collection)))
    }
    pub fn build(items: Vec<Item>) {
        let mut collections: Vec<Rc<RefCell<Box<Collection>>>> = Vec::new();
        let mut state = ItemStateTable::default();
        let mut depth = 0;
        for item in items {
            match Tag::from(item.tag) {
                Tag::Collection => {
                    depth += 1;
                    if depth <= MAX_REASONABLE_COLLECTION_DEPTH {
                        add_collection(&item, &mut collections, &mut state);
                    }
                    state.local.reset();
                }
                Tag::EndCollection => {
                    if depth <= MAX_REASONABLE_COLLECTION_DEPTH {
                        if let Some(state_collection) = state.collection.clone() {
                            state.collection = state_collection.borrow().parent.clone();
                        }
                    }
                    state.local.reset();
                    if depth>0{
                        depth-=1;
                    }
                }
                Tag::Input|Tag::Output|Tag::Feature=>{

                }
                _ => (),
            }
        }
    }
}

fn add_collection(
    item: &Item,
    collections: &mut Vec<Rc<RefCell<Box<Collection>>>>,
    state: &mut ItemStateTable,
) {
    let usage = *state.local.usages.first().unwrap_or(&0);
    let mut usage_page = (usage >> 16) & 0xffff;
    if let Some(last) = state.global_stack.last() {
        if usage_page == 0 {
            usage_page = last.usage_page;
        }
    }
    let collection_type = item.short_data;
    let collection = Collection::new(state.collection.clone(), usage_page, usage, collection_type);
    if let Some(state_collection) = state.collection.clone() {
        let mut collection_mut = state_collection.borrow_mut();
        collection_mut.children.push(collection.clone());
    } else {
        collections.push(collection.clone());
    }
    state.collection = Some(collection.clone());
}

fn add_report_item(tag:Tag,report_info:u32,state:&ItemStateTable){
    
}