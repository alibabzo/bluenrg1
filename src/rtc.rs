#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clockwatch Data Register"]
    pub cwdr: CWDR,
    #[doc = "0x04 - Clockwatch Data Match Register"]
    pub cwdmr: CWDMR,
    #[doc = "0x08 - Clockwatch Data Load Register"]
    pub cwdlr: CWDLR,
    #[doc = "0x0c - Clockwatch Year Register"]
    pub cwyr: CWYR,
    _reserved0: [u8; 2usize],
    #[doc = "0x10 - Clockwatch Year Match Register"]
    pub cwymr: CWYMR,
    _reserved1: [u8; 2usize],
    #[doc = "0x14 - Clockwatch Year Load Register"]
    pub cwylr: CWYLR,
    _reserved2: [u8; 2usize],
    #[doc = "0x18 - Control Trim and Counter Register"]
    pub ctcr: CTCR,
    #[doc = "0x1c - RTC interrupt mask register"]
    pub imsc: IMSC,
    _reserved3: [u8; 3usize],
    #[doc = "0x20 - RTC raw interrupt status register"]
    pub ris: RIS,
    _reserved4: [u8; 3usize],
    #[doc = "0x24 - RTC masked interrupt status register"]
    pub mis: MIS,
    _reserved5: [u8; 3usize],
    #[doc = "0x28 - RTC interrupt clear register"]
    pub icr: ICR,
    _reserved6: [u8; 3usize],
    #[doc = "0x2c - RTC timer load value"]
    pub tdr: TDR,
    #[doc = "0x30 - RTC timer control register"]
    pub tcr: TCR,
    _reserved7: [u8; 2usize],
    #[doc = "0x34 - RTC Timer first Load Register"]
    pub tlr1: TLR1,
    #[doc = "0x38 - RTC Timer second Load Register"]
    pub tlr2: TLR2,
    #[doc = "0x3c - RTC Timer Pattern Register (pattern\\[31:0\\])"]
    pub tpr1: TPR1,
    #[doc = "0x40 - RTC Timer Pattern Register (pattern\\[63:32\\])"]
    pub tpr2: TPR2,
    #[doc = "0x44 - RTC Timer Pattern Register (pattern\\[95:64\\])"]
    pub tpr3: TPR3,
    #[doc = "0x48 - RTC Timer Pattern Register (pattern\\[127:96\\])"]
    pub tpr4: TPR4,
    #[doc = "0x4c - RTC Timer Interrupt Number Register"]
    pub tin: TIN,
}
#[doc = "Clockwatch Data Register"]
pub struct CWDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clockwatch Data Register"]
pub mod cwdr;
#[doc = "Clockwatch Data Match Register"]
pub struct CWDMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clockwatch Data Match Register"]
pub mod cwdmr;
#[doc = "Clockwatch Data Load Register"]
pub struct CWDLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clockwatch Data Load Register"]
pub mod cwdlr;
#[doc = "Clockwatch Year Register"]
pub struct CWYR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Clockwatch Year Register"]
pub mod cwyr;
#[doc = "Clockwatch Year Match Register"]
pub struct CWYMR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Clockwatch Year Match Register"]
pub mod cwymr;
#[doc = "Clockwatch Year Load Register"]
pub struct CWYLR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Clockwatch Year Load Register"]
pub mod cwylr;
#[doc = "Control Trim and Counter Register"]
pub struct CTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Trim and Counter Register"]
pub mod ctcr;
#[doc = "RTC interrupt mask register"]
pub struct IMSC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "RTC interrupt mask register"]
pub mod imsc;
#[doc = "RTC raw interrupt status register"]
pub struct RIS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "RTC raw interrupt status register"]
pub mod ris;
#[doc = "RTC masked interrupt status register"]
pub struct MIS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "RTC masked interrupt status register"]
pub mod mis;
#[doc = "RTC interrupt clear register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "RTC interrupt clear register"]
pub mod icr;
#[doc = "RTC timer load value"]
pub struct TDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC timer load value"]
pub mod tdr;
#[doc = "RTC timer control register"]
pub struct TCR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "RTC timer control register"]
pub mod tcr;
#[doc = "RTC Timer first Load Register"]
pub struct TLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Timer first Load Register"]
pub mod tlr1;
#[doc = "RTC Timer second Load Register"]
pub struct TLR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Timer second Load Register"]
pub mod tlr2;
#[doc = "RTC Timer Pattern Register (pattern\\[31:0\\])"]
pub struct TPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Timer Pattern Register (pattern\\[31:0\\])"]
pub mod tpr1;
#[doc = "RTC Timer Pattern Register (pattern\\[63:32\\])"]
pub struct TPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Timer Pattern Register (pattern\\[63:32\\])"]
pub mod tpr2;
#[doc = "RTC Timer Pattern Register (pattern\\[95:64\\])"]
pub struct TPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Timer Pattern Register (pattern\\[95:64\\])"]
pub mod tpr3;
#[doc = "RTC Timer Pattern Register (pattern\\[127:96\\])"]
pub struct TPR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Timer Pattern Register (pattern\\[127:96\\])"]
pub mod tpr4;
#[doc = "RTC Timer Interrupt Number Register"]
pub struct TIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Timer Interrupt Number Register"]
pub mod tin;
