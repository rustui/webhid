use std::{cell::RefCell, rc::Rc};

#[repr(u8)]
enum Type {
    Main = 0,
    Global = 1,
    Local = 2,
    Reserved = 3,
}
#[repr(u8)]
enum MainTag {
    Default = 0x00,       // 0000
    Input = 0x08,         // 1000
    Output = 0x09,        // 1001
    Feature = 0x0B,       // 1011
    Collection = 0x0A,    // 1010
    EndCollection = 0x0C, // 1100
}
#[repr(u8)]
enum GlobalTag {
    UsagePage = 0x00,       // 0000
    LogicalMinimum = 0x01,  // 0001
    LogicalMaximum = 0x02,  // 0010
    PhysicalMinimum = 0x03, // 0011
    PhysicalMaximum = 0x04, // 0100
    UnitExponent = 0x05,    // 0101
    Unit = 0x06,            // 0110
    ReportSize = 0x07,      // 0111
    ReportId = 0x08,        // 1000
    ReportCount = 0x09,     // 1001
    Push = 0x0A,            // 1010
    Pop = 0x0B,             // 1011
}
#[repr(u8)]
enum LocalTag {
    Usage = 0x00,             // 0000
    UsageMinimum = 0x01,      // 0001
    UsageMaximum = 0x02,      // 0010
    DesignatorIndex = 0x03,   // 0011
    DesignatorMinimum = 0x04, // 0100
    DesignatorMaximum = 0x05, // 0101
    StringIndex = 0x07,       // 0111
    StringMinimum = 0x08,     // 1000
    StringMaximum = 0x09,     // 1001
    Delimiter = 0x0A,         // 1010
}
#[repr(u8)]
enum ReservedTag {
    Long = 0xF, // 1111
}
#[repr(u8)]
enum Tag {
    Default = (MainTag::Default as u8) << 2 | Type::Main as u8,
    Input = (MainTag::Input as u8) << 2 | Type::Main as u8,
    Output = (MainTag::Output as u8) << 2 | Type::Main as u8,
    Feature = (MainTag::Feature as u8) << 2 | Type::Main as u8,
    Collection = (MainTag::Collection as u8) << 2 | Type::Main as u8,
    EndCollection = (MainTag::EndCollection as u8) << 2 | Type::Main as u8,
    UsagePage = (GlobalTag::UsagePage as u8) << 2 | Type::Global as u8,
    LogicalMinimum = (GlobalTag::LogicalMinimum as u8) << 2 | Type::Global as u8,
    LogicalMaximum = (GlobalTag::LogicalMaximum as u8) << 2 | Type::Global as u8,
    PhysicalMinimum = (GlobalTag::PhysicalMinimum as u8) << 2 | Type::Global as u8,
    PhysicalMaximum = (GlobalTag::PhysicalMaximum as u8) << 2 | Type::Global as u8,
    UnitExponent = (GlobalTag::UnitExponent as u8) << 2 | Type::Global as u8,
    Unit = (GlobalTag::Unit as u8) << 2 | Type::Global as u8,
    ReportSize = (GlobalTag::ReportSize as u8) << 2 | Type::Global as u8,
    ReportId = (GlobalTag::ReportId as u8) << 2 | Type::Global as u8,
    ReportCount = (GlobalTag::ReportCount as u8) << 2 | Type::Global as u8,
    Push = (GlobalTag::Push as u8) << 2 | Type::Global as u8,
    Pop = (GlobalTag::Pop as u8) << 2 | Type::Global as u8,
    Usage = (LocalTag::Usage as u8) << 2 | Type::Local as u8,
    UsageMinimum = (LocalTag::UsageMinimum as u8) << 2 | Type::Local as u8,
    UsageMaximum = (LocalTag::UsageMaximum as u8) << 2 | Type::Local as u8,
    DesignatorIndex = (LocalTag::DesignatorIndex as u8) << 2 | Type::Local as u8,
    DesignatorMinimum = (LocalTag::DesignatorMinimum as u8) << 2 | Type::Local as u8,
    DesignatorMaximum = (LocalTag::DesignatorMaximum as u8) << 2 | Type::Local as u8,
    StringIndex = (LocalTag::StringIndex as u8) << 2 | Type::Local as u8,
    StringMinimum = (LocalTag::StringMinimum as u8) << 2 | Type::Local as u8,
    StringMaximum = (LocalTag::StringMaximum as u8) << 2 | Type::Local as u8,
    Delimiter = (LocalTag::Delimiter as u8) << 2 | Type::Local as u8,
    Long = (ReservedTag::Long as u8) << 2 | Type::Reserved as u8,
}

struct Header {
    size: usize,
    item_type: u8,
    tag: u8,
}

impl Header {
    fn new(byte: &u8) -> Self {
        Self {
            size: (byte & 0b11) as usize,
            item_type: byte >> 2 & 0b11,
            tag: byte >> 4,
        }
    }
}

pub struct Item {
    tag: u8,
    payload_size: usize,
    previous: Option<Rc<RefCell<Box<Item>>>>,
    next: Option<Rc<RefCell<Box<Item>>>>,
    parent: Option<Rc<RefCell<Box<Item>>>>,
}

impl Item {
    pub fn new(bytes: Vec<u8>, previous: Option<Rc<RefCell<Box<Item>>>>) -> Rc<RefCell<Box<Item>>> {
        let header = Header::new(bytes.first().unwrap());
        let tag: u8 = header.tag << 2 | header.item_type;
        let mut payload_size = 0usize;
        let mut short_data = 0u32;
        if Self::is_long(tag) {
            if let Some(size) = bytes.get(1) {
                payload_size = *size as usize;
            }
        } else {
            payload_size = if header.size == 0x3 { 4 } else { header.size };
            if Self::get_header_size(tag) + payload_size <= bytes.len() {
                // short_data.copy_from_slice(
                //     &bytes[Self::get_header_size(tag)..Self::get_header_size(tag) + payload_size],
                // );
            }
        }
        let this = Self {
            tag,
            payload_size,
            previous: previous.clone(),
            next: None,
            parent: None,
        };
        let this = Rc::new(RefCell::new(Box::new(this)));
        if let Some(prev) = previous {
            let mut prev_mut = prev.borrow_mut();
            prev_mut.next = Some(this.clone());
            if prev_mut.tag == Tag::Collection as u8 {
                let mut this_mut = this.borrow_mut();
                this_mut.parent = Some(prev.clone());
            } else {
                let mut this_mut = this.borrow_mut();
                if this_mut.tag == Tag::EndCollection as u8 {
                    if let Some(parent) = prev_mut.parent.clone() {
                        this_mut.parent = parent.borrow().parent.clone();
                    } else {
                        this_mut.parent = prev_mut.parent.clone();
                    }
                }
            }
        }
        this
    }

    fn get_depth(&self) -> usize {
        if let Some(parent) = &self.parent {
            parent.borrow().get_depth() + 1
        } else {
            0
        }
    }

    fn is_long(tag: u8) -> bool {
        tag == Tag::Long as u8
    }

    fn get_header_size(tag: u8) -> usize {
        if Self::is_long(tag) {
            3
        } else {
            1
        }
    }

    pub fn get_size(&self) -> usize {
        Self::get_header_size(self.tag) + self.payload_size
    }
}
