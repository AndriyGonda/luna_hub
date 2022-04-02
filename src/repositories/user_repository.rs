use crate::configuration::db::Connection;
use crate::models::device::{Device, DeviceDTO};
use crate::schema::devices::dsl::devices;
use diesel::RunQueryDsl;

struct DeviceRepository<'a> {
    connection: &'a Connection,
}

impl<'a> DeviceRepository<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        DeviceRepository { connection }
    }

    pub fn create_device(&self, device_dto: &DeviceDTO) {
        let query_result = diesel::insert_into(devices)
            .values(device_dto)
            .execute(self.connection);

        match query_result {
            Ok(success) => println!("Device created {:?}", success),
            Err(exc) => println!("Issue happened {:?}", exc),
        }
    }
}
