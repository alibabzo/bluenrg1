#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AHB up/down converter command register"]
    pub command: COMMAND,
}
#[doc = "AHB up/down converter command register"]
pub struct COMMAND {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "AHB up/down converter command register"]
pub mod command;
