#[derive(Debug)]
pub struct Table {
    pub num_rows: usize,
    pub pages: Vec<Vec<u8>>,
}

fn main(){
    let mut buffer = vec![];
    buffer.extend_from_slice("123".as_bytes());
    buffer.extend_from_slice("345".as_bytes());
    
    print!("{:?}", Table {num_rows: 3, pages: vec![vec![]; 3]});
}