#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Level selection for wakeup IO (1 bit for IO) IO\\[13:9\\].<ul><li>0: The system wakes up when IO is low.</li><li>1: The system wakes up when IO is high.</li></ul>"]
    pub wkp_io_is: WKP_IO_IS,
    _reserved0: [u8; 3usize],
    #[doc = "0x04 - Enables the IO that wakes up the device (1 bit for IO) IO\\[13:9\\].<ul><li>0: The wakes up feature on the IO is disabled.</li><li>1: The wakes up feature on the IO is enabled.</li></ul>"]
    pub wkp_io_ie: WKP_IO_IE,
    _reserved1: [u8; 3usize],
    #[doc = "0x08 - XO frequency indication to provide by the application"]
    pub ctrl: CTRL,
}
#[doc = "Level selection for wakeup IO (1 bit for IO) IO\\[13:9\\].<ul><li>0: The system wakes up when IO is low.</li><li>1: The system wakes up when IO is high.</li></ul>"]
pub struct WKP_IO_IS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Level selection for wakeup IO (1 bit for IO) IO\\[13:9\\].<ul><li>0: The system wakes up when IO is low.</li><li>1: The system wakes up when IO is high.</li></ul>"]
pub mod wkp_io_is;
#[doc = "Enables the IO that wakes up the device (1 bit for IO) IO\\[13:9\\].<ul><li>0: The wakes up feature on the IO is disabled.</li><li>1: The wakes up feature on the IO is enabled.</li></ul>"]
pub struct WKP_IO_IE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Enables the IO that wakes up the device (1 bit for IO) IO\\[13:9\\].<ul><li>0: The wakes up feature on the IO is disabled.</li><li>1: The wakes up feature on the IO is enabled.</li></ul>"]
pub mod wkp_io_ie;
#[doc = "XO frequency indication to provide by the application"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "XO frequency indication to provide by the application"]
pub mod ctrl;
