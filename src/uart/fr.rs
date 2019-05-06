#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::FR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CTSR {
    bits: bool,
}
impl CTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct BUSYR {
    bits: bool,
}
impl BUSYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RXFER {
    bits: bool,
}
impl RXFER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TXFFR {
    bits: bool,
}
impl TXFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RXFFR {
    bits: bool,
}
impl RXFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TXFER {
    bits: bool,
}
impl TXFER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DCTSR {
    bits: bool,
}
impl DCTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RTXDISR {
    bits: bool,
}
impl RTXDISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Clear to send."]
    #[inline]
    pub fn cts(&self) -> CTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CTSR { bits }
    }
    #[doc = "Bit 3 - UART Busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. However, if the transmit section of the UART is disabled in the middle of a transmission, the BUSY bit gets cleared. This bit is set again once the transmit section is re-enabled to complete the remaining transmission.This bit is set as soon as the transmit FIFO becomes nonempty (regardless of whether the UART is enabled or not)."]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        BUSYR { bits }
    }
    #[doc = "Bit 4 - Receive FIFO empty. If the FIFO is disabled (bit FEN = 0b), this bit is set when the receive holding register is empty. If the FIFO is enabled (FEN = 1b), the RXFE bit is set when the receive FIFO is empty."]
    #[inline]
    pub fn rxfe(&self) -> RXFER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RXFER { bits }
    }
    #[doc = "Bit 5 - Transmit FIFO full. If the FIFO is disabled (bit FEN = 0b), this bit is set when the transmit holding register is full. If the FIFO is enabled (FEN = 1b), the TXFF bit is set when the transmit FIFO is full."]
    #[inline]
    pub fn txff(&self) -> TXFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TXFFR { bits }
    }
    #[doc = "Bit 6 - Receive FIFO full. If the FIFO is disabled (bit FEN = 0b), this bit is set when the receive holding register is full. If the FIFO is enabled (FEN = 1b), the RXFF bit is set when the receive FIFO is full."]
    #[inline]
    pub fn rxff(&self) -> RXFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RXFFR { bits }
    }
    #[doc = "Bit 7 - Transmit FIFO empty. If the FIFO is disabled (bit FEN = 0b), this bit is set when the transmit holding register is empty. If the FIFO is enabled (FEN = 1b), the TXFE bit is set when the transmit FIFO is empty."]
    #[inline]
    pub fn txfe(&self) -> TXFER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TXFER { bits }
    }
    #[doc = "Bit 9 - Delta Clear To Send. This bit is set CTS changes since the last read of the FR register."]
    #[inline]
    pub fn dcts(&self) -> DCTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        DCTSR { bits }
    }
    #[doc = "Bit 13 - Remote Transmitter Disabled (software flow control). This bit indicates an Xoff character was sent to the remote transmitter to stop it after the received FIFO has passed over its trigger limit. This bit is cleared when a Xon character is sent to the remote transmitter."]
    #[inline]
    pub fn rtxdis(&self) -> RTXDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RTXDISR { bits }
    }
}
