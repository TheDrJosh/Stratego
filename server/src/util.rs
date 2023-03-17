use std::str::FromStr;

use common::Side;
use rocket::request::FromParam;
use uuid::Uuid;

pub struct UuidGard(pub Uuid);

impl<'a> FromParam<'a> for UuidGard {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match Uuid::from_str(param) {
            Ok(id) => Ok(UuidGard(id)),
            Err(_) => Err(param),
        }
    }
}

pub struct SideGard(pub Side);

impl<'a> FromParam<'a> for SideGard {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match Side::from_str(param) {
            Ok(side) => Ok(SideGard(side)),
            Err(_) => Err(param),
        }
    }
}
