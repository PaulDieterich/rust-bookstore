use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Book{
    pub id: usize,
    pub title: String,
    pub author_name: String,
    pub site_count: u16,
    pub is_published: bool,
   
}

