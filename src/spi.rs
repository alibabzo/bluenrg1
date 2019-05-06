#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register 0"]
    pub cr0: CR0,
    #[doc = "0x04 - Control Register 1"]
    pub cr1: CR1,
    #[doc = "0x08 - Data Register"]
    pub dr: DR,
    #[doc = "0x0c - Status Register"]
    pub sr: SR,
    _reserved0: [u8; 3usize],
    #[doc = "0x10 - Clock prescale register"]
    pub cpsr: CPSR,
    _reserved1: [u8; 3usize],
    #[doc = "0x14 - Interrupt mask set or clear register"]
    pub imsc: IMSC,
    _reserved2: [u8; 3usize],
    #[doc = "0x18 - Raw interrupt status register"]
    pub ris: RIS,
    _reserved3: [u8; 3usize],
    #[doc = "0x1c - Masked Interrupt Status Register"]
    pub mis: MIS,
    _reserved4: [u8; 3usize],
    #[doc = "0x20 - Interrupt clear register"]
    pub icr: ICR,
    _reserved5: [u8; 3usize],
    #[doc = "0x24 - SPI DMA control register"]
    pub dmacr: DMACR,
    _reserved6: [u8; 3usize],
    #[doc = "0x28 - SPI Receive Frame register. Indicates the number of frames to receive from the slave."]
    pub rxfrm: RXFRM,
    _reserved7: [u8; 2usize],
    #[doc = "0x2c - Dummy character register"]
    pub chn: CHN,
    #[doc = "0x30 - SPI transmit FIFO receive frame number. Indicates the number of frames to receive from the transmit FIFO."]
    pub wdtxf: WDTXF,
    _reserved8: [u8; 78usize],
    #[doc = "0x80 - Integration test control register"]
    pub itcr: ITCR,
    _reserved9: [u8; 11usize],
    #[doc = "0x8c - FIFO Test Data Register"]
    pub tdr: TDR,
}
#[doc = "Control Register 0"]
pub struct CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register 0"]
pub mod cr0;
#[doc = "Control Register 1"]
pub struct CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register 1"]
pub mod cr1;
#[doc = "Data Register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Register"]
pub mod dr;
#[doc = "Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Clock prescale register"]
pub struct CPSR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clock prescale register"]
pub mod cpsr;
#[doc = "Interrupt mask set or clear register"]
pub struct IMSC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt mask set or clear register"]
pub mod imsc;
#[doc = "Raw interrupt status register"]
pub struct RIS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Raw interrupt status register"]
pub mod ris;
#[doc = "Masked Interrupt Status Register"]
pub struct MIS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Masked Interrupt Status Register"]
pub mod mis;
#[doc = "Interrupt clear register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt clear register"]
pub mod icr;
#[doc = "SPI DMA control register"]
pub struct DMACR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SPI DMA control register"]
pub mod dmacr;
#[doc = "SPI Receive Frame register. Indicates the number of frames to receive from the slave."]
pub struct RXFRM {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "SPI Receive Frame register. Indicates the number of frames to receive from the slave."]
pub mod rxfrm;
#[doc = "Dummy character register"]
pub struct CHN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dummy character register"]
pub mod chn;
#[doc = "SPI transmit FIFO receive frame number. Indicates the number of frames to receive from the transmit FIFO."]
pub struct WDTXF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "SPI transmit FIFO receive frame number. Indicates the number of frames to receive from the transmit FIFO."]
pub mod wdtxf;
#[doc = "Integration test control register"]
pub struct ITCR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Integration test control register"]
pub mod itcr;
#[doc = "FIFO Test Data Register"]
pub struct TDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Test Data Register"]
pub mod tdr;
