#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::IMSC {
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
#[doc = "Possible values of the field `RORIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RORIMR {
    #[doc = "irq disable"]
    IRQ_DISABLE,
    #[doc = "irq enable"]
    IRQ_ENABLE,
}
impl RORIMR {
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
            RORIMR::IRQ_DISABLE => false,
            RORIMR::IRQ_ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RORIMR {
        match value {
            false => RORIMR::IRQ_DISABLE,
            true => RORIMR::IRQ_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `IRQ_DISABLE`"]
    #[inline]
    pub fn is_irq_disable(&self) -> bool {
        *self == RORIMR::IRQ_DISABLE
    }
    #[doc = "Checks if the value of the field is `IRQ_ENABLE`"]
    #[inline]
    pub fn is_irq_enable(&self) -> bool {
        *self == RORIMR::IRQ_ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct RTIMR {
    bits: bool,
}
impl RTIMR {
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
pub struct RXIMR {
    bits: bool,
}
impl RXIMR {
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
pub struct TURIMR {
    bits: bool,
}
impl TURIMR {
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
pub struct TEIMR {
    bits: bool,
}
impl TEIMR {
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
#[doc = "Values that can be written to the field `RORIM`"]
pub enum RORIMW {
    #[doc = "irq disable"]
    IRQ_DISABLE,
    #[doc = "irq enable"]
    IRQ_ENABLE,
}
impl RORIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RORIMW::IRQ_DISABLE => false,
            RORIMW::IRQ_ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RORIMW<'a> {
    w: &'a mut W,
}
impl<'a> _RORIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RORIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "irq disable"]
    #[inline]
    pub fn irq_disable(self) -> &'a mut W {
        self.variant(RORIMW::IRQ_DISABLE)
    }
    #[doc = "irq enable"]
    #[inline]
    pub fn irq_enable(self) -> &'a mut W {
        self.variant(RORIMW::IRQ_ENABLE)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _RTIMW<'a> {
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
#[doc = r" Proxy"]
pub struct _RXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIMW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXIMW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TURIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TURIMW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TEIMW<'a> {
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Receive overrun interrupt mask:<ul><li>0: RX FIFO written to while full condition interrupt is masked (irq disabled).</li><li>1: RX FIFO written to while full condition interrupt is not masked (irq enabled).</li></ul>"]
    #[inline]
    pub fn rorim(&self) -> RORIMR {
        RORIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Receive timeout interrupt mask:<ul><li>0: RX FIFO not empty or no read prior to the timeout period interrupt is masked (irq disabled).</li><li>1: RX FIFO not empty or no read prior to the timeout period interrupt is not masked (irq enabled).</li></ul>"]
    #[inline]
    pub fn rtim(&self) -> RTIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        RTIMR { bits }
    }
    #[doc = "Bit 2 - Receive FIFO interrupt mask:<ul><li>0: Receive interrupt is masked (irq disabled).</li><li>1: Receive interrupt is not masked (irq enabled).</li></ul>"]
    #[inline]
    pub fn rxim(&self) -> RXIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        RXIMR { bits }
    }
    #[doc = "Bit 3 - Transmit FIFO interrupt mask:<ul><li>0: Transmit interrupt is masked (irq disabled).</li><li>1: Transmit interrupt is not masked (irq enabled).</li></ul>"]
    #[inline]
    pub fn txim(&self) -> TXIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        TXIMR { bits }
    }
    #[doc = "Bit 4 - Transmit underrun interrupt mask:<ul><li>0: Transmit underrun interrupt is masked (irq disabled).</li><li>1: Transmit underrun interrupt is not masked (irq enabled).</li></ul>"]
    #[inline]
    pub fn turim(&self) -> TURIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        TURIMR { bits }
    }
    #[doc = "Bit 5 - Transmit FIFO empty interrupt mask:<ul><li>0: TX FIFO empty interrupt is masked (irq disabled).</li><li>1: TX FIFO empty interrupt is not masked (irq enabled).</li></ul>"]
    #[inline]
    pub fn teim(&self) -> TEIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        TEIMR { bits }
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
    #[doc = "Bit 0 - Receive overrun interrupt mask:<ul><li>0: RX FIFO written to while full condition interrupt is masked (irq disabled).</li><li>1: RX FIFO written to while full condition interrupt is not masked (irq enabled).</li></ul>"]
    #[inline]
    pub fn rorim(&mut self) -> _RORIMW {
        _RORIMW { w: self }
    }
    #[doc = "Bit 1 - Receive timeout interrupt mask:<ul><li>0: RX FIFO not empty or no read prior to the timeout period interrupt is masked (irq disabled).</li><li>1: RX FIFO not empty or no read prior to the timeout period interrupt is not masked (irq enabled).</li></ul>"]
    #[inline]
    pub fn rtim(&mut self) -> _RTIMW {
        _RTIMW { w: self }
    }
    #[doc = "Bit 2 - Receive FIFO interrupt mask:<ul><li>0: Receive interrupt is masked (irq disabled).</li><li>1: Receive interrupt is not masked (irq enabled).</li></ul>"]
    #[inline]
    pub fn rxim(&mut self) -> _RXIMW {
        _RXIMW { w: self }
    }
    #[doc = "Bit 3 - Transmit FIFO interrupt mask:<ul><li>0: Transmit interrupt is masked (irq disabled).</li><li>1: Transmit interrupt is not masked (irq enabled).</li></ul>"]
    #[inline]
    pub fn txim(&mut self) -> _TXIMW {
        _TXIMW { w: self }
    }
    #[doc = "Bit 4 - Transmit underrun interrupt mask:<ul><li>0: Transmit underrun interrupt is masked (irq disabled).</li><li>1: Transmit underrun interrupt is not masked (irq enabled).</li></ul>"]
    #[inline]
    pub fn turim(&mut self) -> _TURIMW {
        _TURIMW { w: self }
    }
    #[doc = "Bit 5 - Transmit FIFO empty interrupt mask:<ul><li>0: TX FIFO empty interrupt is masked (irq disabled).</li><li>1: TX FIFO empty interrupt is not masked (irq enabled).</li></ul>"]
    #[inline]
    pub fn teim(&mut self) -> _TEIMW {
        _TEIMW { w: self }
    }
}
