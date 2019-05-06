#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FER {
    bits: bool,
}
impl FER {
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
pub struct PER {
    bits: bool,
}
impl PER {
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
pub struct BER {
    bits: bool,
}
impl BER {
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
pub struct OER {
    bits: bool,
}
impl OER {
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
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Frame error. This bit is set to 1 if the received character did not have a valid stop bit (a valid stop bit is 1).This bit is cleared to 0b after a write to ECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline]
    pub fn fe(&self) -> FER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FER { bits }
    }
    #[doc = "Bit 1 - Parity error. This bit is set to 1 if the parity of the received data character does not match the parity selected as defined by bits 2 and 7 of the LCRH_RX register.This bit is cleared to 0b after a write to ECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline]
    pub fn pe(&self) -> PER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PER { bits }
    }
    #[doc = "Bit 2 - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held low for longer than a full-word transmission time (defined as start, data, parity and stop bits). This bit is cleared to 0b after a write to ECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to HIGH (marking state), and the next valid start bit is received."]
    #[inline]
    pub fn be(&self) -> BER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BER { bits }
    }
    #[doc = "Bit 3 - Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full. This is cleared to 0 by a write to ECR (data value is not important). The FIFO contents remain valid since no further data is written when the FIFO is full, only the content of the shift register are overwritten. The CPU or DMA must now read the data in order to empty the FIFO."]
    #[inline]
    pub fn oe(&self) -> OER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OER { bits }
    }
}
