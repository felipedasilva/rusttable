pub mod model;

use model::Table;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct TableService {
    tables: HashMap<String, Table>,
}

impl TableService {
    pub fn new() -> TableService {
        TableService {
            tables: HashMap::default(),
        }
    }

    pub fn create_table(&mut self, dto: CreateTableBody) -> Result<&Table, ()> {
        self.tables.insert(
            dto.id.clone(),
            Table::new(dto.id.clone(), dto.size_y, dto.size_x),
        );
        self.get_table(dto.id)
    }

    pub fn get_table(&self, id: String) -> Result<&Table, ()> {
        if let Some(table) = self.tables.get(&id) {
            Ok(table)
        } else {
            Err(())
        }
    }

    pub fn get_mut_table(&mut self, id: String) -> Result<&mut Table, ()> {
        if let Some(table) = self.tables.get_mut(&id) {
            Ok(table)
        } else {
            Err(())
        }
    }

    pub fn change_table(&mut self, dto: ChangeTableDTO) -> Result<(), ()> {
        let table = self.get_mut_table(dto.id)?;
        table.set_value(dto.y, dto.x, dto.value)?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateTableBody {
    pub id: String,
    pub size_y: u8,
    pub size_x: u8,
}

#[derive(Deserialize, Serialize)]
pub struct ChangeTableDTO {
    pub id: String,
    pub x: u8,
    pub y: u8,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_table() {
        let dto = CreateTableBody {
            id: String::from("test"),
            size_y: 1,
            size_x: 1,
        };
        let mut service = TableService::new();
        assert_eq!(
            service.create_table(dto),
            Ok(&Table::new(String::from("test"), 1, 1))
        );
    }

    #[test]
    fn test_get_table() {
        let dto = CreateTableBody {
            id: String::from("test"),
            size_y: 1,
            size_x: 1,
        };
        let mut service = TableService::new();
        service.create_table(dto).unwrap();
        assert_eq!(
            service.get_table(String::from("test")),
            Ok(&Table::new(String::from("test"), 1, 1))
        );
    }

    #[test]
    fn test_get_mut_table() {
        let dto = CreateTableBody {
            id: String::from("test"),
            size_y: 1,
            size_x: 1,
        };
        let mut service = TableService::new();
        service.create_table(dto).unwrap();
        assert_eq!(
            service.get_mut_table(String::from("test")),
            Ok(&mut Table::new(String::from("test"), 1, 1))
        );
    }

    #[test]
    fn test_change_table_error_when_not_found_table() {
        let dto = CreateTableBody {
            id: String::from("test"),
            size_y: 1,
            size_x: 1,
        };
        let mut service = TableService::new();
        service.create_table(dto).unwrap();
        let dto = ChangeTableDTO {
            id: String::from("notfound"),
            y: 0,
            x: 1,
            value: String::from("new_value"),
        };
        assert_eq!(service.change_table(dto), Err(()));
    }

    #[test]
    fn test_change_table_error_when_x_out_of_range() {
        let dto = CreateTableBody {
            id: String::from("test"),
            size_y: 0,
            size_x: 1,
        };
        let mut service = TableService::new();
        service.create_table(dto).unwrap();
        let dto = ChangeTableDTO {
            id: String::from("test"),
            y: 0,
            x: 1,
            value: String::from("new_value"),
        };
        assert_eq!(service.change_table(dto), Err(()));
    }

    #[test]
    fn test_change_table_error_when_y_out_of_range() {
        let dto = CreateTableBody {
            id: String::from("test"),
            size_y: 0,
            size_x: 1,
        };
        let mut service = TableService::new();
        service.create_table(dto).unwrap();
        let dto = ChangeTableDTO {
            id: String::from("test"),
            y: 1,
            x: 0,
            value: String::from("new_value"),
        };
        assert_eq!(service.change_table(dto), Err(()));
    }

    #[test]
    fn test_change_table() {
        let dto = CreateTableBody {
            id: String::from("test"),
            size_y: 1,
            size_x: 1,
        };
        let mut service = TableService::new();
        service.create_table(dto).unwrap();
        let dto = ChangeTableDTO {
            id: String::from("test"),
            y: 0,
            x: 0,
            value: String::from("new_value"),
        };
        assert_eq!(service.change_table(dto), Ok(()));
        let table = service.get_table(String::from("test")).unwrap();
        let value = table.get_value(0, 0);
        assert_eq!(value, Some(&String::from("new_value")));
    }
}
