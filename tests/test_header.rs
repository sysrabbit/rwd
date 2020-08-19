use rwd::header::*;
use std::fs::File;

mod common;

#[test]
fn test_loading_header_from_file() {
    for file_name in common::get_files(String::from("rwd")).iter() {
        let mut file = File::open(file_name).expect("An RWD container file");
        let header = Header::load(&mut file);
        assert_eq!(header.is_ok(), true);
        let header = header.unwrap();
        for byte in 0..26 {
            assert_eq!(header.constant_bytes[byte], MAGIC_HEADER_BYTES[byte]);
        }
    }
}

#[test]
fn build_correct_header() {
    // Check the Header.build() method
    let header: Header = Header::builder().build();
    for byte in 0..26 {
        assert_eq!(header.constant_bytes[byte], MAGIC_HEADER_BYTES[byte]);
    }

    // Check the HeaderBuilder methods
    let header = HeaderBuilder::new().build();
    for byte in 0..26 {
        assert_eq!(header.constant_bytes[byte], MAGIC_HEADER_BYTES[byte]);
    }
}

#[test]
fn set_trailing_header_bytes() {
    // Check the Header.build() method
    // TODO: Fuzz bytes
    let bytes: [u8; 4] = [0, 1, 2, 3];
    let header: Header = Header::builder().set_trailing_bytes(bytes).build();
    for byte in 0..4 {
        assert_eq!(header.unknown_bytes[byte], bytes[byte]);
    }
    // Check the HeaderBuilder methods
    let header = HeaderBuilder::new().set_trailing_bytes(bytes).build();
    for byte in 0..4 {
        assert_eq!(header.unknown_bytes[byte], bytes[byte]);
    }
}
