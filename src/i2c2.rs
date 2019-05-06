#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Control register"]
    pub cr: CR,
    #[doc = "0x04 - I2C Slave Control register"]
    pub scr: SCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - I2C master control register"]
    pub mcr: MCR,
    #[doc = "0x10 - I2C transmit FIFO register"]
    pub tfr: TFR,
    _reserved1: [u8; 3usize],
    #[doc = "0x14 - I2C status register"]
    pub sr: SR,
    #[doc = "0x18 - I2C receive FIFO register"]
    pub rfr: RFR,
    _reserved2: [u8; 3usize],
    #[doc = "0x1c - I2C transmit FIFO threshold register"]
    pub tftr: TFTR,
    _reserved3: [u8; 2usize],
    #[doc = "0x20 - I2C receive FIFO threshold register"]
    pub rftr: RFTR,
    _reserved4: [u8; 2usize],
    #[doc = "0x24 - I2C DMA register"]
    pub dmar: DMAR,
    _reserved5: [u8; 2usize],
    #[doc = "0x28 - I2C Baud-rate counter register"]
    pub brcr: BRCR,
    _reserved6: [u8; 2usize],
    #[doc = "0x2c - I2C interrupt mask set/clear register"]
    pub imscr: IMSCR,
    #[doc = "0x30 - I2C raw interrupt status register"]
    pub risr: RISR,
    #[doc = "0x34 - I2C masked interrupt status register"]
    pub misr: MISR,
    #[doc = "0x38 - I2C interrupt clear register"]
    pub icr: ICR,
    _reserved7: [u8; 16usize],
    #[doc = "0x4c - I2C hold time data"]
    pub thddat: THDDAT,
    _reserved8: [u8; 2usize],
    #[doc = "0x50 - I2C hold time start condition F/S"]
    pub thdsta_fst_std: THDSTA_FST_STD,
    _reserved9: [u8; 4usize],
    #[doc = "0x58 - I2C setup time start condition F/S"]
    pub tsusta_fst_std: TSUSTA_FST_STD,
}
#[doc = "I2C Control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Control register"]
pub mod cr;
#[doc = "I2C Slave Control register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave Control register"]
pub mod scr;
#[doc = "I2C master control register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C master control register"]
pub mod mcr;
#[doc = "I2C transmit FIFO register"]
pub struct TFR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2C transmit FIFO register"]
pub mod tfr;
#[doc = "I2C status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C status register"]
pub mod sr;
#[doc = "I2C receive FIFO register"]
pub struct RFR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "I2C receive FIFO register"]
pub mod rfr;
#[doc = "I2C transmit FIFO threshold register"]
pub struct TFTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "I2C transmit FIFO threshold register"]
pub mod tftr;
#[doc = "I2C receive FIFO threshold register"]
pub struct RFTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "I2C receive FIFO threshold register"]
pub mod rftr;
#[doc = "I2C DMA register"]
pub struct DMAR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "I2C DMA register"]
pub mod dmar;
#[doc = "I2C Baud-rate counter register"]
pub struct BRCR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "I2C Baud-rate counter register"]
pub mod brcr;
#[doc = "I2C interrupt mask set/clear register"]
pub struct IMSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C interrupt mask set/clear register"]
pub mod imscr;
#[doc = "I2C raw interrupt status register"]
pub struct RISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C raw interrupt status register"]
pub mod risr;
#[doc = "I2C masked interrupt status register"]
pub struct MISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C masked interrupt status register"]
pub mod misr;
#[doc = "I2C interrupt clear register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C interrupt clear register"]
pub mod icr;
#[doc = "I2C hold time data"]
pub struct THDDAT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "I2C hold time data"]
pub mod thddat;
#[doc = "I2C hold time start condition F/S"]
pub struct THDSTA_FST_STD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C hold time start condition F/S"]
pub mod thdsta_fst_std;
#[doc = "I2C setup time start condition F/S"]
pub struct TSUSTA_FST_STD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C setup time start condition F/S"]
pub mod tsusta_fst_std;
