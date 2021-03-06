use super::Item;

struct ReportDescriptor {}

impl ReportDescriptor {
    fn new(bytes: Vec<u8>) {
        let mut items = vec![];
        let mut header_index = 0usize;
        while header_index < bytes.len() {
            let item = Item::new(bytes[header_index..].to_vec(), None);
            header_index += item.borrow().get_size();
            items.push(item);
        }
        println!("items.len: {}", items.len());
    }
}

#[test]
fn test() {
    let bytes: Vec<u8> = vec![
        5, 1, 9, 5, 161, 1, 133, 1, 9, 48, 9, 49, 9, 50, 9, 53, 9, 51, 9, 52, 21, 0, 38, 255, 0,
        117, 8, 149, 6, 129, 2, 6, 0, 255, 9, 32, 149, 1, 129, 2, 5, 1, 9, 57, 21, 0, 37, 7, 53, 0,
        70, 59, 1, 101, 20, 117, 4, 149, 1, 129, 66, 101, 0, 5, 9, 25, 1, 41, 15, 21, 0, 37, 1,
        117, 1, 149, 15, 129, 2, 6, 0, 255, 9, 33, 149, 13, 129, 2, 6, 0, 255, 9, 34, 21, 0, 38,
        255, 0, 117, 8, 149, 52, 129, 2, 133, 2, 9, 35, 149, 47, 145, 2, 133, 5, 9, 51, 149, 40,
        177, 2, 133, 8, 9, 52, 149, 47, 177, 2, 133, 9, 9, 36, 149, 19, 177, 2, 133, 10, 9, 37,
        149, 26, 177, 2, 133, 32, 9, 38, 149, 63, 177, 2, 133, 33, 9, 39, 149, 4, 177, 2, 133, 34,
        9, 64, 149, 63, 177, 2, 133, 128, 9, 40, 149, 63, 177, 2, 133, 129, 9, 41, 149, 63, 177, 2,
        133, 130, 9, 42, 149, 9, 177, 2, 133, 131, 9, 43, 149, 63, 177, 2, 133, 132, 9, 44, 149,
        63, 177, 2, 133, 133, 9, 45, 149, 2, 177, 2, 133, 160, 9, 46, 149, 1, 177, 2, 133, 224, 9,
        47, 149, 63, 177, 2, 133, 240, 9, 48, 149, 63, 177, 2, 133, 241, 9, 49, 149, 63, 177, 2,
        133, 242, 9, 50, 149, 15, 177, 2, 133, 244, 9, 53, 149, 63, 177, 2, 133, 245, 9, 54, 149,
        3, 177, 2, 192,
    ];
    ReportDescriptor::new(bytes);
}
