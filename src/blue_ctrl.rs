#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Timeout programming register"]
    pub timeout: TIMEOUT,
    _reserved1: [u8; 4usize],
    #[doc = "0x0c - Radio configuration register"]
    pub radio_config: RADIO_CONFIG,
}
#[doc = "Timeout programming register"]
pub struct TIMEOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timeout programming register"]
pub mod timeout;
#[doc = "Radio configuration register"]
pub struct RADIO_CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio configuration register"]
pub mod radio_config;
