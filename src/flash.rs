#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Commands for the module"]
    pub command: COMMAND,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - Configure the wrapper"]
    pub config: CONFIG,
    _reserved1: [u8; 2usize],
    #[doc = "0x08 - Flash status interrupt (masked)"]
    pub irqstat: IRQSTAT,
    _reserved2: [u8; 2usize],
    #[doc = "0x0c - Mask for interrupts"]
    pub irqmask: IRQMASK,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - Status interrupts (unmasked)"]
    pub irqraw: IRQRAW,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - Indicates the last usable address of the main Flash"]
    pub size: SIZE,
    _reserved5: [u8; 2usize],
    #[doc = "0x18 - Address for programming Flash, will auto-increment"]
    pub address: ADDRESS,
    _reserved6: [u8; 8usize],
    #[doc = "0x24 - LFSR register needed for check after MASS READ command"]
    pub lfsrval: LFSRVAL,
    #[doc = "0x28 - Trimming values for Flash erase/modify sequences"]
    pub timetrim1: TIMETRIM1,
    #[doc = "0x2c - Trimming values for Flash erase/modify sequences"]
    pub timetrim2: TIMETRIM2,
    #[doc = "0x30 - Trimming values for Flash wake-up sequence"]
    pub timetrim3: TIMETRIM3,
    _reserved7: [u8; 12usize],
    #[doc = "0x40 - Program cycle data"]
    pub data0: DATA0,
    #[doc = "0x44 - Program cycle data"]
    pub data1: DATA1,
    #[doc = "0x48 - Program cycle data"]
    pub data2: DATA2,
    #[doc = "0x4c - Program cycle data"]
    pub data3: DATA3,
}
#[doc = "Commands for the module"]
pub struct COMMAND {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Commands for the module"]
pub mod command;
#[doc = "Configure the wrapper"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Configure the wrapper"]
pub mod config;
#[doc = "Flash status interrupt (masked)"]
pub struct IRQSTAT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Flash status interrupt (masked)"]
pub mod irqstat;
#[doc = "Mask for interrupts"]
pub struct IRQMASK {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Mask for interrupts"]
pub mod irqmask;
#[doc = "Status interrupts (unmasked)"]
pub struct IRQRAW {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Status interrupts (unmasked)"]
pub mod irqraw;
#[doc = "Indicates the last usable address of the main Flash"]
pub struct SIZE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Indicates the last usable address of the main Flash"]
pub mod size;
#[doc = "Address for programming Flash, will auto-increment"]
pub struct ADDRESS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address for programming Flash, will auto-increment"]
pub mod address;
#[doc = "LFSR register needed for check after MASS READ command"]
pub struct LFSRVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFSR register needed for check after MASS READ command"]
pub mod lfsrval;
#[doc = "Trimming values for Flash erase/modify sequences"]
pub struct TIMETRIM1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trimming values for Flash erase/modify sequences"]
pub mod timetrim1;
#[doc = "Trimming values for Flash erase/modify sequences"]
pub struct TIMETRIM2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trimming values for Flash erase/modify sequences"]
pub mod timetrim2;
#[doc = "Trimming values for Flash wake-up sequence"]
pub struct TIMETRIM3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trimming values for Flash wake-up sequence"]
pub mod timetrim3;
#[doc = "Program cycle data"]
pub struct DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Program cycle data"]
pub mod data0;
#[doc = "Program cycle data"]
pub struct DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Program cycle data"]
pub mod data1;
#[doc = "Program cycle data"]
pub struct DATA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Program cycle data"]
pub mod data2;
#[doc = "Program cycle data"]
pub struct DATA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Program cycle data"]
pub mod data3;
