use std::{fs::OpenOptions, io::Seek};

use crate::contants::{
    StatmentType, EMAIL_OFFSET, EMAIL_SIZE, ID_OFFSET, ID_SIZE, ROWS_PER_PAGE, ROW_SIZE,
    TABLE_MAX_PAGES, USERNAME_OFFSET, USERNAME_SIZE,
};

pub struct Row {
    pub id: u32,
    pub username: [u8; USERNAME_SIZE],
    pub email: [u8; EMAIL_SIZE],
}

pub struct Statement {
    pub stype: StatmentType,
    pub row_to_insert: Row,
}

impl Row {
    pub fn new() -> Self {
        Row {
            id: 0,
            username: [0u8; USERNAME_SIZE],
            email: [0u8; EMAIL_SIZE],
        }
    }
}
pub struct Table {
    pub num_rows: usize,
    pub pages: Vec<Vec<u8>>,
}

impl Table {

    pub fn db_open(filename: &str) -> Self {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .unwrap();
        let file_length = file.seek(std::io::SeekFrom::End(0)).unwrap();
        let num_rows = file_length as u32 / ROW_SIZE;
        return Table {
            num_rows: num_rows as usize,
            pages: vec![vec![]; TABLE_MAX_PAGES as usize],
        };
    }
    /// row_num: là số lượng row tại thời điểm hiện tại ví dụ
    /// ta đã thêm 2 row thì trước khi insert ta phải biết số row ở thời điển hiện tại
    /// sau đó xác định nó sẽ phải thêm ở page nào
    pub fn row_slot(&mut self, row_num: u32) -> (u32, u32) {
        let page_num = row_num / ROWS_PER_PAGE;

        let row_offset = row_num % ROWS_PER_PAGE;
        let bytes_offset = row_offset * ROW_SIZE;

        return (page_num, bytes_offset);
    }

    // sau khi đã có dữ liệu từ struct
    // lấy bộ dữ liệu mã hóa sang bytes và lưu vào page theo kiểu tuần tự hóa (Serializable) (append data)

    pub fn serialize_row(&mut self, row: Row, page_num: u32) {
        let id_bytes = row.id.to_ne_bytes();
        let user_name_bytes = row.username;
        let email = row.email;

        self.pages[page_num as usize].extend_from_slice(&id_bytes); // append data
        self.pages[page_num as usize].extend_from_slice(&user_name_bytes);
        self.pages[page_num as usize].extend_from_slice(&email);
    }
}
