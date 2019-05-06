#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RDYR {
    bits: bool,
}
impl RDYR {
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
pub struct REVCLKR {
    bits: bool,
}
impl REVCLKR {
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
pub struct FAULTR {
    bits: bool,
}
impl FAULTR {
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
    #[doc = "Bit 0 - New random value ready.<ul><li>0: The RNG_VAL register value is not yet valid. If performing a read access to VAL, the host will be put on hold (by wait-states insertion on the AHB bus) until a random value is available.</li><li>1: The VAL register contains a valid random number.</li></ul>This bit remains at 0 when the RNG is disabled (RNGDIS bit = 1b in CR)"]
    #[inline]
    pub fn rdy(&self) -> RDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RDYR { bits }
    }
    #[doc = "Bit 1 - REVCLK clock reveal bit. A write with 1b to bit TSTCLK in CR resets this bit. If the RNGCLK is present, this bit will be 1b after four RNGCLK cycles after the end of the write to RNG_CR.If REVCLK = 0b after this period, it means the RNGCLK is not present and reading VAL will trigger a AHB error response."]
    #[inline]
    pub fn revclk(&self) -> REVCLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REVCLKR { bits }
    }
    #[doc = "Bit 2 - Fault reveal bit. This bit is set by hardware when a faulty sequence of bits occurs. The faulty sequences are:<ul><li>0: Sequence of more than 32 consecutive bits of same value (0b or 1b).</li><li>1: Sequence of more than 16 consecutive alternation of 0b and 1b (010101...01b).</li></ul>Writing this bit:<ul><li>0: No effect.</li><li>1: Clear the bit.</li></ul>"]
    #[inline]
    pub fn fault(&self) -> FAULTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FAULTR { bits }
    }
}
