#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IMSCR {
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
#[doc = "Possible values of the field `TXFEM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFEMR {
    #[doc = "Disable the interrupt mask"]
    DISABLE,
    #[doc = "Enable the interrupt mask"]
    ENABLE,
}
impl TXFEMR {
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
            TXFEMR::DISABLE => false,
            TXFEMR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFEMR {
        match value {
            false => TXFEMR::DISABLE,
            true => TXFEMR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TXFEMR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TXFEMR::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct TXFNEMR {
    bits: bool,
}
impl TXFNEMR {
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
pub struct TXFFMR {
    bits: bool,
}
impl TXFFMR {
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
pub struct TXFOVRMR {
    bits: bool,
}
impl TXFOVRMR {
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
pub struct RXFEMR {
    bits: bool,
}
impl RXFEMR {
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
pub struct RXFNFMR {
    bits: bool,
}
impl RXFNFMR {
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
pub struct RXFFMR {
    bits: bool,
}
impl RXFFMR {
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
pub struct RFSRMR {
    bits: bool,
}
impl RFSRMR {
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
pub struct RFSEMR {
    bits: bool,
}
impl RFSEMR {
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
pub struct WTSRMR {
    bits: bool,
}
impl WTSRMR {
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
pub struct MTDMR {
    bits: bool,
}
impl MTDMR {
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
pub struct STDMR {
    bits: bool,
}
impl STDMR {
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
pub struct MALMR {
    bits: bool,
}
impl MALMR {
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
pub struct BERRMR {
    bits: bool,
}
impl BERRMR {
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
pub struct MTDWSMR {
    bits: bool,
}
impl MTDWSMR {
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
#[doc = "Values that can be written to the field `TXFEM`"]
pub enum TXFEMW {
    #[doc = "Disable the interrupt mask"]
    DISABLE,
    #[doc = "Enable the interrupt mask"]
    ENABLE,
}
impl TXFEMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFEMW::DISABLE => false,
            TXFEMW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFEMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFEMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFEMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the interrupt mask"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXFEMW::DISABLE)
    }
    #[doc = "Enable the interrupt mask"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXFEMW::ENABLE)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXFNEMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFNEMW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXFFMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFFMW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXFOVRMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFOVRMW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXFEMW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFEMW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXFNFMW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFNFMW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXFFMW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFFMW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFSRMW<'a> {
    w: &'a mut W,
}
impl<'a> _RFSRMW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFSEMW<'a> {
    w: &'a mut W,
}
impl<'a> _RFSEMW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WTSRMW<'a> {
    w: &'a mut W,
}
impl<'a> _WTSRMW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MTDMW<'a> {
    w: &'a mut W,
}
impl<'a> _MTDMW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STDMW<'a> {
    w: &'a mut W,
}
impl<'a> _STDMW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MALMW<'a> {
    w: &'a mut W,
}
impl<'a> _MALMW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BERRMW<'a> {
    w: &'a mut W,
}
impl<'a> _BERRMW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MTDWSMW<'a> {
    w: &'a mut W,
}
impl<'a> _MTDWSMW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - TX FIFO empty mask. TXFEM enables the interrupt bit TXFE:<ul><li>0: TXFE interrupt is disabled.</li><li>1: TXFE interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn txfem(&self) -> TXFEMR {
        TXFEMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TX FIFO nearly empty mask. TXFNEM enables the interrupt bit TXFNE:<ul><li>0: TXFNE interrupt is disabled.</li><li>1: TXFNE interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn txfnem(&self) -> TXFNEMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFNEMR { bits }
    }
    #[doc = "Bit 2 - TX FIFO full mask. TXFFM enables the interrupt bit TXFF:<ul><li>0: TXFF interrupt is disabled.</li><li>1: TXFF interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn txffm(&self) -> TXFFMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFFMR { bits }
    }
    #[doc = "Bit 3 - TX FIFO overrun mask. TXOVRM enables the interrupt bit TXOVR:<ul><li>0: TXOVR interrupt is disabled.</li><li>1: TXOVR interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn txfovrm(&self) -> TXFOVRMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXFOVRMR { bits }
    }
    #[doc = "Bit 4 - RX FIFO empty mask. RXFEM enables the interrupt bit RXFE:<ul><li>0: RXFE interrupt is disabled.</li><li>1: RXFE interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn rxfem(&self) -> RXFEMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFEMR { bits }
    }
    #[doc = "Bit 5 - RX FIFO nearly full mask. RXNFM enables the interrupt bit RXNF:<ul><li>0: RXNF interrupt is disabled.</li><li>1: RXNF interrupt is enabled</li></ul>"]
    #[inline]
    pub fn rxfnfm(&self) -> RXFNFMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFNFMR { bits }
    }
    #[doc = "Bit 6 - RX FIFO full mask. RXFFM enables the interrupt bit RXFF:<ul><li>0: RXFF interrupt is disabled.</li><li>1: RXFF interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn rxffm(&self) -> RXFFMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFFMR { bits }
    }
    #[doc = "Bit 16 - Read-from-Slave request mask. RFSRM enables the interrupt bit RFSR:<ul><li>0: RFSR interrupt is disabled.</li><li>1: RFSR interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn rfsrm(&self) -> RFSRMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFSRMR { bits }
    }
    #[doc = "Bit 17 - Read-from-Slave empty mask. RFSEM enables the interrupt bit RFSE:<ul><li>0: RFSE interrupt is disabled.</li><li>1: RFSE interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn rfsem(&self) -> RFSEMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFSEMR { bits }
    }
    #[doc = "Bit 18 - Write-to-Slave request mask. WTSRM enables the interrupt bit WTSR:<ul><li>0: WTSR interrupt is disabled.</li><li>1: WTSR interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn wtsrm(&self) -> WTSRMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WTSRMR { bits }
    }
    #[doc = "Bit 19 - Master Transaction done mask. MTDM enables the interrupt bit MTD:<ul><li>0: MTD interrupt is disabled.</li><li>1: MTD interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn mtdm(&self) -> MTDMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MTDMR { bits }
    }
    #[doc = "Bit 20 - Slave Transaction done mask. STDM enables the interrupt bit STD:<ul><li>0: STDM interrupt is disabled.</li><li>1: STDM interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn stdm(&self) -> STDMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STDMR { bits }
    }
    #[doc = "Bit 24 - Master Arbitration lost mask. MALM enables the interrupt bit MAL:<ul><li>0: MAL interrupt is disabled.</li><li>1: MAL interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn malm(&self) -> MALMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MALMR { bits }
    }
    #[doc = "Bit 25 - Bus Error mask. BERRM enables the interrupt bit BERR:<ul><li>0: BERR interrupt is disabled.</li><li>1: BERR interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn berrm(&self) -> BERRMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BERRMR { bits }
    }
    #[doc = "Bit 28 - Master Transaction done without stop mask. MTDWSM enables the interrupt bit MTDWS:<ul><li>0: MTDWS interrupt is disabled.</li><li>1: MTDWS interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn mtdwsm(&self) -> MTDWSMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MTDWSMR { bits }
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - TX FIFO empty mask. TXFEM enables the interrupt bit TXFE:<ul><li>0: TXFE interrupt is disabled.</li><li>1: TXFE interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn txfem(&mut self) -> _TXFEMW {
        _TXFEMW { w: self }
    }
    #[doc = "Bit 1 - TX FIFO nearly empty mask. TXFNEM enables the interrupt bit TXFNE:<ul><li>0: TXFNE interrupt is disabled.</li><li>1: TXFNE interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn txfnem(&mut self) -> _TXFNEMW {
        _TXFNEMW { w: self }
    }
    #[doc = "Bit 2 - TX FIFO full mask. TXFFM enables the interrupt bit TXFF:<ul><li>0: TXFF interrupt is disabled.</li><li>1: TXFF interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn txffm(&mut self) -> _TXFFMW {
        _TXFFMW { w: self }
    }
    #[doc = "Bit 3 - TX FIFO overrun mask. TXOVRM enables the interrupt bit TXOVR:<ul><li>0: TXOVR interrupt is disabled.</li><li>1: TXOVR interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn txfovrm(&mut self) -> _TXFOVRMW {
        _TXFOVRMW { w: self }
    }
    #[doc = "Bit 4 - RX FIFO empty mask. RXFEM enables the interrupt bit RXFE:<ul><li>0: RXFE interrupt is disabled.</li><li>1: RXFE interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn rxfem(&mut self) -> _RXFEMW {
        _RXFEMW { w: self }
    }
    #[doc = "Bit 5 - RX FIFO nearly full mask. RXNFM enables the interrupt bit RXNF:<ul><li>0: RXNF interrupt is disabled.</li><li>1: RXNF interrupt is enabled</li></ul>"]
    #[inline]
    pub fn rxfnfm(&mut self) -> _RXFNFMW {
        _RXFNFMW { w: self }
    }
    #[doc = "Bit 6 - RX FIFO full mask. RXFFM enables the interrupt bit RXFF:<ul><li>0: RXFF interrupt is disabled.</li><li>1: RXFF interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn rxffm(&mut self) -> _RXFFMW {
        _RXFFMW { w: self }
    }
    #[doc = "Bit 16 - Read-from-Slave request mask. RFSRM enables the interrupt bit RFSR:<ul><li>0: RFSR interrupt is disabled.</li><li>1: RFSR interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn rfsrm(&mut self) -> _RFSRMW {
        _RFSRMW { w: self }
    }
    #[doc = "Bit 17 - Read-from-Slave empty mask. RFSEM enables the interrupt bit RFSE:<ul><li>0: RFSE interrupt is disabled.</li><li>1: RFSE interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn rfsem(&mut self) -> _RFSEMW {
        _RFSEMW { w: self }
    }
    #[doc = "Bit 18 - Write-to-Slave request mask. WTSRM enables the interrupt bit WTSR:<ul><li>0: WTSR interrupt is disabled.</li><li>1: WTSR interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn wtsrm(&mut self) -> _WTSRMW {
        _WTSRMW { w: self }
    }
    #[doc = "Bit 19 - Master Transaction done mask. MTDM enables the interrupt bit MTD:<ul><li>0: MTD interrupt is disabled.</li><li>1: MTD interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn mtdm(&mut self) -> _MTDMW {
        _MTDMW { w: self }
    }
    #[doc = "Bit 20 - Slave Transaction done mask. STDM enables the interrupt bit STD:<ul><li>0: STDM interrupt is disabled.</li><li>1: STDM interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn stdm(&mut self) -> _STDMW {
        _STDMW { w: self }
    }
    #[doc = "Bit 24 - Master Arbitration lost mask. MALM enables the interrupt bit MAL:<ul><li>0: MAL interrupt is disabled.</li><li>1: MAL interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn malm(&mut self) -> _MALMW {
        _MALMW { w: self }
    }
    #[doc = "Bit 25 - Bus Error mask. BERRM enables the interrupt bit BERR:<ul><li>0: BERR interrupt is disabled.</li><li>1: BERR interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn berrm(&mut self) -> _BERRMW {
        _BERRMW { w: self }
    }
    #[doc = "Bit 28 - Master Transaction done without stop mask. MTDWSM enables the interrupt bit MTDWS:<ul><li>0: MTDWS interrupt is disabled.</li><li>1: MTDWS interrupt is enabled.</li></ul>"]
    #[inline]
    pub fn mtdwsm(&mut self) -> _MTDWSMW {
        _MTDWSMW { w: self }
    }
}
