use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Table {
    id: String,
    size_x: u8,
    size_y: u8,
    data: Vec<Vec<String>>,
}

impl Table {
    pub fn new(id: String, size_y: u8, size_x: u8) -> Table {
        let data: Vec<Vec<String>> = vec![vec![String::from(""); size_x as usize]; size_y as usize];
        Table { id, size_x, size_y, data }
    }

    pub fn size(&self) -> u8 {
        self.size_x * self.size_y
    }

    pub fn get_value(&self, y: u8, x: u8) -> Option<&String> {
        if let Some(v) = self.data.get(y as usize) {
            if let Some(v) = v.get(x as usize) {
                return Some(&v);
            }
        }
        None
    }

    pub fn set_value(&mut self, y: u8, x: u8, new_value: String) -> Result<(), ()> {
        if let Some(v) = self.data.get_mut(y as usize) {
            if let Some(value) = v.get_mut(x as usize) {
                *value = new_value;
                return Ok(());
            }
        }
        Err(())
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "table: {}x{}\n", self.size_x, self.size_y).unwrap();
        self.data.iter().for_each(|vy| {
            write!(f, "|").unwrap();
            vy.to_vec().iter().for_each(|vx| {
                write!(f, "{}|", vx).unwrap();
            });
            write!(f, "\n").unwrap();
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let table = Table::new(String::from("id"),2, 3);
        assert_eq!(table.id, "id");
        assert_eq!(table.size_x, 3);
        assert_eq!(table.size_y, 2);
    }

    #[test]
    fn test_size() {
        let table = Table::new(String::from("id"),3, 3);
        assert_eq!(table.size(), 9);
    }

    #[test]
    fn test_cells_count() {
        let table = Table::new(String::from("id"),2, 3);
        assert_eq!(table.data.len(), 2);
        let data = table.data;
        data.iter().for_each(|vy| {
			assert_eq!(vy.len(), 3);
            vy.iter().for_each(|a| {
                assert_eq!(a.to_owned(), String::from(""));
            });
        });
    }

    #[test]
    fn test_get_value() {
        let table = Table::new(String::from("id"),2, 3);
        if let Some(value) = table.get_value(1, 1) {
            assert_eq!(value.to_owned(), String::from(""));
        } else {
            assert!(false, "It's expected to found the value");
        }
    }

    #[test]
    fn test_set_value() {
        let mut table = Table::new(String::from("id"),2, 3);
        match table.set_value(1, 1, String::from("hello world")) {
            Ok(_) => {}
            Err(_) => {
                assert!(false, "It's expected to set the value");
            }
        }
        let value = table.get_value(1, 1).unwrap();
        assert_eq!(value.to_owned(), String::from("hello world"));
        table.data.iter().enumerate().for_each(|(i, vy)| {
            vy.to_vec().iter().enumerate().for_each(|(k, vx)| {
                if i != 1 && k != 1 {
                    assert_eq!(vx.to_owned(), String::from(""));
                }
            });
        });
    }

    #[test]
    fn test_error_set_value() {
        let mut table = Table::new(String::from("id"),2, 3);
        match table.set_value(4, 4, String::from("hello world")) {
            Ok(_) => {
                assert!(false, "It's expected to set the value");
            }
            Err(_) => {}
        }
    }
}
