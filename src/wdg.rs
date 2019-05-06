#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Load Register"]
    pub lr: LR,
    #[doc = "0x04 - Watchdog Value Register"]
    pub val: VAL,
    #[doc = "0x08 - Watchdog Control Register"]
    pub cr: CR,
    _reserved0: [u8; 3usize],
    #[doc = "0x0c - Watchdog Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x10 - Watchdog Raw Interrupt Status Register"]
    pub ris: RIS,
    _reserved1: [u8; 3usize],
    #[doc = "0x14 - Watchdog Masked Interrupt Status Register"]
    pub mis: MIS,
    _reserved2: [u8; 3051usize],
    #[doc = "0xc00 - Watchdog Lock Register"]
    pub lock: LOCK,
}
#[doc = "Watchdog Load Register"]
pub struct LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Load Register"]
pub mod lr;
#[doc = "Watchdog Value Register"]
pub struct VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Value Register"]
pub mod val;
#[doc = "Watchdog Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Watchdog Control Register"]
pub mod cr;
#[doc = "Watchdog Interrupt Clear Register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Interrupt Clear Register"]
pub mod icr;
#[doc = "Watchdog Raw Interrupt Status Register"]
pub struct RIS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Watchdog Raw Interrupt Status Register"]
pub mod ris;
#[doc = "Watchdog Masked Interrupt Status Register"]
pub struct MIS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Watchdog Masked Interrupt Status Register"]
pub mod mis;
#[doc = "Watchdog Lock Register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Lock Register"]
pub mod lock;
