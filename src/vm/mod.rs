use crate::contants::*;
use crate::table::{Row, Statement, Table};

pub fn execute_insert(statement: &Statement, table: &mut Table) {
    let (page_num, _) = table.row_slot(table.num_rows as u32);
    let row = Row {
        id: statement.row_to_insert.id,
        email: statement.row_to_insert.email,
        username: statement.row_to_insert.username,
    };
    table.serialize_row(row, page_num);
    table.num_rows += 1;
    println!("Executed Success");
}
