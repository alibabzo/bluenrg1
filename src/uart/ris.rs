#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::RIS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `CTSMIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSMISR {
    #[doc = "interrupt pending"]
    INTERRUPT_PENDING,
    #[doc = "interrupt not pending"]
    INTERRUPT_NOT_PENDING,
}
impl CTSMISR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CTSMISR::INTERRUPT_PENDING => true,
            CTSMISR::INTERRUPT_NOT_PENDING => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSMISR {
        match value {
            true => CTSMISR::INTERRUPT_PENDING,
            false => CTSMISR::INTERRUPT_NOT_PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_PENDING`"]
    #[inline]
    pub fn is_interrupt_pending(&self) -> bool {
        *self == CTSMISR::INTERRUPT_PENDING
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_NOT_PENDING`"]
    #[inline]
    pub fn is_interrupt_not_pending(&self) -> bool {
        *self == CTSMISR::INTERRUPT_NOT_PENDING
    }
}
#[doc = r" Value of the field"]
pub struct RXISR {
    bits: bool,
}
impl RXISR {
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
pub struct TXIMR {
    bits: bool,
}
impl TXIMR {
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
pub struct RTISR {
    bits: bool,
}
impl RTISR {
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
pub struct FEISR {
    bits: bool,
}
impl FEISR {
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
pub struct PEISR {
    bits: bool,
}
impl PEISR {
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
pub struct BEISR {
    bits: bool,
}
impl BEISR {
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
pub struct OEISR {
    bits: bool,
}
impl OEISR {
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
pub struct XOFFISR {
    bits: bool,
}
impl XOFFISR {
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
pub struct TXFEISR {
    bits: bool,
}
impl TXFEISR {
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
    #[doc = "Bit 1 - Clear to send interrupt status.<ul><li>0: The interrupt is not pending.</li><li>1: The interrupt is pending.</li></ul>"]
    #[inline]
    pub fn ctsmis(&self) -> CTSMISR {
        CTSMISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Receive interrupt status.<ul><li>0: The interrupt is not pending.</li><li>1: The interrupt is pending.</li></ul>"]
    #[inline]
    pub fn rxis(&self) -> RXISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RXISR { bits }
    }
    #[doc = "Bit 5 - Transmit interrupt status.<ul><li>0: The interrupt is not pending.</li><li>1: The interrupt is pending.</li></ul>"]
    #[inline]
    pub fn txim(&self) -> TXIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TXIMR { bits }
    }
    #[doc = "Bit 6 - Receive timeout interrupt status.<ul><li>0: The interrupt is not pending.</li><li>1: The interrupt is pending.</li></ul>"]
    #[inline]
    pub fn rtis(&self) -> RTISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RTISR { bits }
    }
    #[doc = "Bit 7 - Framing error interrupt status.<ul><li>0: The interrupt is not pending.</li><li>1: The interrupt is pending.</li></ul>"]
    #[inline]
    pub fn feis(&self) -> FEISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        FEISR { bits }
    }
    #[doc = "Bit 8 - Parity error interrupt status.<ul><li>0: The interrupt is not pending.</li><li>1: The interrupt is pending.</li></ul>"]
    #[inline]
    pub fn peis(&self) -> PEISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        PEISR { bits }
    }
    #[doc = "Bit 9 - Break error interrupt status.<ul><li>0: The interrupt is not pending.</li><li>1: The interrupt is pending.</li></ul>"]
    #[inline]
    pub fn beis(&self) -> BEISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        BEISR { bits }
    }
    #[doc = "Bit 10 - Overrun error interrupt status.<ul><li>0: The interrupt is not pending.</li><li>1: The interrupt is pending.</li></ul>"]
    #[inline]
    pub fn oeis(&self) -> OEISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        OEISR { bits }
    }
    #[doc = "Bit 11 - XOFF interrupt status.<ul><li>0: The interrupt is not pending.</li><li>1: The interrupt is pending.</li></ul>"]
    #[inline]
    pub fn xoffis(&self) -> XOFFISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        XOFFISR { bits }
    }
    #[doc = "Bit 12 - TX FIFO empty interrupt status.<ul><li>0: The interrupt is not pending.</li><li>1: The interrupt is pending.</li></ul>"]
    #[inline]
    pub fn txfeis(&self) -> TXFEISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TXFEISR { bits }
    }
}
