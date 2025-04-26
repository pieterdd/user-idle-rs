mod linux_dbus_impl;
mod linux_x11_impl;

use linux_dbus_impl::{
    get_idle_time_from_mutter, get_idle_time_from_screensaver,
};
use linux_x11_impl::get_idle_time as get_idle_time_from_x11;

use crate::Error;
use std::time::Duration;

pub fn get_idle_time() -> Result<Duration, Error> {
    match get_idle_time_from_mutter() {
        Ok(duration) => return Ok(duration),
        Err(_) => {}
    }
    match get_idle_time_from_x11() {
        Ok(duration) => return Ok(duration),
        Err(_) => {}
    }
    match get_idle_time_from_screensaver() {
        Ok(duration) => return Ok(duration),
        Err(error) => Err(error),
    }
}
