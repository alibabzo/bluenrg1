#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
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
#[doc = "Possible values of the field `RIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RISR {
    #[doc = "Watchdog interrupt is not active"]
    IRQ_NOT_PENDING,
    #[doc = "Watchdog interrupt is active"]
    IRQ_PENDING,
}
impl RISR {
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
            RISR::IRQ_NOT_PENDING => false,
            RISR::IRQ_PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RISR {
        match value {
            false => RISR::IRQ_NOT_PENDING,
            true => RISR::IRQ_PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `IRQ_NOT_PENDING`"]
    #[inline]
    pub fn is_irq_not_pending(&self) -> bool {
        *self == RISR::IRQ_NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `IRQ_PENDING`"]
    #[inline]
    pub fn is_irq_pending(&self) -> bool {
        *self == RISR::IRQ_PENDING
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Watchdog raw interrupt status bit. Reflects the status of the interrupt status from the watchdog:<ul><li>0: watchdog interrupt is not active.</li><li>1: watchdog interrupt is active.</li></ul>Read-only bit. A write has no effect."]
    #[inline]
    pub fn ris(&self) -> RISR {
        RISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
