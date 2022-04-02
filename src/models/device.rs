use crate::schema::devices;
use diesel::{Identifiable, Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "devices"]
pub struct DeviceDTO {
    pub name: String,
}
