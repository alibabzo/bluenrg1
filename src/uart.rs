#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Register"]
    pub dr: DR,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - Receive Status Register"]
    pub rsr: RSR,
    _reserved1: [u8; 4usize],
    #[doc = "0x0c - Timeout Register"]
    pub timeout: TIMEOUT,
    _reserved2: [u8; 8usize],
    #[doc = "0x18 - Flag Register"]
    pub fr: FR,
    _reserved3: [u8; 2usize],
    #[doc = "0x1c - Receive Line Control Register"]
    pub lcrh_rx: LCRH_RX,
    _reserved4: [u8; 7usize],
    #[doc = "0x24 - Integer Baud Rate Register"]
    pub ibrd: IBRD,
    _reserved5: [u8; 2usize],
    #[doc = "0x28 - Fractional Baud Rate Register"]
    pub fbrd: FBRD,
    _reserved6: [u8; 3usize],
    #[doc = "0x2c - Transmit Line Control Register"]
    pub lcrh_tx: LCRH_TX,
    _reserved7: [u8; 3usize],
    #[doc = "0x30 - Control Register"]
    pub cr: CR,
    #[doc = "0x34 - Interrupt FIFO level select register"]
    pub ifls: IFLS,
    _reserved8: [u8; 3usize],
    #[doc = "0x38 - Interrupt Mask Set/Clear Register"]
    pub imsc: IMSC,
    _reserved9: [u8; 2usize],
    #[doc = "0x3c - Raw Interrupt Status Register"]
    pub ris: RIS,
    _reserved10: [u8; 2usize],
    #[doc = "0x40 - Masked Interrupt Status Register"]
    pub mis: MIS,
    _reserved11: [u8; 2usize],
    #[doc = "0x44 - Interrupt Clear Register"]
    pub icr: ICR,
    _reserved12: [u8; 2usize],
    #[doc = "0x48 - DMA control register"]
    pub dmacr: DMACR,
    _reserved13: [u8; 7usize],
    #[doc = "0x50 - XON/XOFF Control Register"]
    pub xfcr: XFCR,
    _reserved14: [u8; 3usize],
    #[doc = "0x54 - Register used to store the Xon1 character used for software flow control"]
    pub xon1: XON1,
    _reserved15: [u8; 3usize],
    #[doc = "0x58 - Register used to store the Xon2 character used for software flow control"]
    pub xon2: XON2,
    _reserved16: [u8; 3usize],
    #[doc = "0x5c - Register used to store the Xoff1 character used for software flow control"]
    pub xoff1: XOFF1,
    _reserved17: [u8; 3usize],
    #[doc = "0x60 - Register used to store the Xoff2 character used for software flow control"]
    pub xoff2: XOFF2,
}
#[doc = "Data Register"]
pub struct DR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Data Register"]
pub mod dr;
#[doc = "Receive Status Register"]
pub struct RSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "Error Clear Register. A write to this register clears the framing (FE), parity (PE), break (BE), and overrun (OE) errors."]
pub struct ECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Clear Register. A write to this register clears the framing (FE), parity (PE), break (BE), and overrun (OE) errors."]
pub mod ecr;
#[doc = "Timeout Register"]
pub struct TIMEOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timeout Register"]
pub mod timeout;
#[doc = "Flag Register"]
pub struct FR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Flag Register"]
pub mod fr;
#[doc = "Receive Line Control Register"]
pub struct LCRH_RX {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Receive Line Control Register"]
pub mod lcrh_rx;
#[doc = "Integer Baud Rate Register"]
pub struct IBRD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Integer Baud Rate Register"]
pub mod ibrd;
#[doc = "Fractional Baud Rate Register"]
pub struct FBRD {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Fractional Baud Rate Register"]
pub mod fbrd;
#[doc = "Transmit Line Control Register"]
pub struct LCRH_TX {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Transmit Line Control Register"]
pub mod lcrh_tx;
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Interrupt FIFO level select register"]
pub struct IFLS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt FIFO level select register"]
pub mod ifls;
#[doc = "Interrupt Mask Set/Clear Register"]
pub struct IMSC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt Mask Set/Clear Register"]
pub mod imsc;
#[doc = "Raw Interrupt Status Register"]
pub struct RIS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Raw Interrupt Status Register"]
pub mod ris;
#[doc = "Masked Interrupt Status Register"]
pub struct MIS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Masked Interrupt Status Register"]
pub mod mis;
#[doc = "Interrupt Clear Register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "DMA control register"]
pub struct DMACR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DMA control register"]
pub mod dmacr;
#[doc = "XON/XOFF Control Register"]
pub struct XFCR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "XON/XOFF Control Register"]
pub mod xfcr;
#[doc = "Register used to store the Xon1 character used for software flow control"]
pub struct XON1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Register used to store the Xon1 character used for software flow control"]
pub mod xon1;
#[doc = "Register used to store the Xon2 character used for software flow control"]
pub struct XON2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Register used to store the Xon2 character used for software flow control"]
pub mod xon2;
#[doc = "Register used to store the Xoff1 character used for software flow control"]
pub struct XOFF1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Register used to store the Xoff1 character used for software flow control"]
pub mod xoff1;
#[doc = "Register used to store the Xoff2 character used for software flow control"]
pub struct XOFF2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Register used to store the Xoff2 character used for software flow control"]
pub mod xoff2;
