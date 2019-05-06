#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::ITCR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SWAPFIFO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAPFIFOR {
    #[doc = "FIFO normal mode"]
    FIFO_NORMAL_MODE,
    #[doc = "FIFO test mode"]
    FIFO_TEST_MODE,
}
impl SWAPFIFOR {
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
            SWAPFIFOR::FIFO_NORMAL_MODE => false,
            SWAPFIFOR::FIFO_TEST_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWAPFIFOR {
        match value {
            false => SWAPFIFOR::FIFO_NORMAL_MODE,
            true => SWAPFIFOR::FIFO_TEST_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_NORMAL_MODE`"]
    #[inline]
    pub fn is_fifo_normal_mode(&self) -> bool {
        *self == SWAPFIFOR::FIFO_NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `FIFO_TEST_MODE`"]
    #[inline]
    pub fn is_fifo_test_mode(&self) -> bool {
        *self == SWAPFIFOR::FIFO_TEST_MODE
    }
}
#[doc = "Values that can be written to the field `SWAPFIFO`"]
pub enum SWAPFIFOW {
    #[doc = "FIFO normal mode"]
    FIFO_NORMAL_MODE,
    #[doc = "FIFO test mode"]
    FIFO_TEST_MODE,
}
impl SWAPFIFOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWAPFIFOW::FIFO_NORMAL_MODE => false,
            SWAPFIFOW::FIFO_TEST_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWAPFIFOW<'a> {
    w: &'a mut W,
}
impl<'a> _SWAPFIFOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWAPFIFOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO normal mode"]
    #[inline]
    pub fn fifo_normal_mode(self) -> &'a mut W {
        self.variant(SWAPFIFOW::FIFO_NORMAL_MODE)
    }
    #[doc = "FIFO test mode"]
    #[inline]
    pub fn fifo_test_mode(self) -> &'a mut W {
        self.variant(SWAPFIFOW::FIFO_TEST_MODE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 1 - FIFO control mode:<ul><li>0: FIFO normal mode. Write in TDR register puts data in TX FIFO and read from TDR register read data from RX FIFO.</li><li>1: FIFO swapped mode. Write in TDR register puts data in RX FIFO and read from TDR register read data from TX FIFO.</li></ul>"]
    #[inline]
    pub fn swapfifo(&self) -> SWAPFIFOR {
        SWAPFIFOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - FIFO control mode:<ul><li>0: FIFO normal mode. Write in TDR register puts data in TX FIFO and read from TDR register read data from RX FIFO.</li><li>1: FIFO swapped mode. Write in TDR register puts data in RX FIFO and read from TDR register read data from TX FIFO.</li></ul>"]
    #[inline]
    pub fn swapfifo(&mut self) -> _SWAPFIFOW {
        _SWAPFIFOW { w: self }
    }
}
