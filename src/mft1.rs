#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer / Counter1 register"]
    pub tncnt1: TNCNT1,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - Capture / Reload A register"]
    pub tncra: TNCRA,
    _reserved1: [u8; 2usize],
    #[doc = "0x08 - Capture / Reload B register"]
    pub tncrb: TNCRB,
    _reserved2: [u8; 2usize],
    #[doc = "0x0c - Timer / Counter2 register"]
    pub tncnt2: TNCNT2,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - Clock prescaler register"]
    pub tnprsc: TNPRSC,
    _reserved4: [u8; 3usize],
    #[doc = "0x14 - Clock unit control register"]
    pub tnckc: TNCKC,
    _reserved5: [u8; 3usize],
    #[doc = "0x18 - Timer mode control register"]
    pub tnmctrl: TNMCTRL,
    _reserved6: [u8; 2usize],
    #[doc = "0x1c - Timer interrupt control register"]
    pub tnictrl: TNICTRL,
    _reserved7: [u8; 3usize],
    #[doc = "0x20 - Timer interrupt clear register"]
    pub tniclr: TNICLR,
}
#[doc = "Timer / Counter1 register"]
pub struct TNCNT1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer / Counter1 register"]
pub mod tncnt1;
#[doc = "Capture / Reload A register"]
pub struct TNCRA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture / Reload A register"]
pub mod tncra;
#[doc = "Capture / Reload B register"]
pub struct TNCRB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture / Reload B register"]
pub mod tncrb;
#[doc = "Timer / Counter2 register"]
pub struct TNCNT2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer / Counter2 register"]
pub mod tncnt2;
#[doc = "Clock prescaler register"]
pub struct TNPRSC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clock prescaler register"]
pub mod tnprsc;
#[doc = "Clock unit control register"]
pub struct TNCKC {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clock unit control register"]
pub mod tnckc;
#[doc = "Timer mode control register"]
pub struct TNMCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer mode control register"]
pub mod tnmctrl;
#[doc = "Timer interrupt control register"]
pub struct TNICTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer interrupt control register"]
pub mod tnictrl;
#[doc = "Timer interrupt clear register"]
pub struct TNICLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Timer interrupt clear register"]
pub mod tniclr;
