#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::TCR {
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
#[doc = r" Value of the field"]
pub struct OSR {
    bits: bool,
}
impl OSR {
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
pub struct ENR {
    bits: bool,
}
impl ENR {
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
pub struct SR {
    bits: bool,
}
impl SR {
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
pub struct SPR {
    bits: u8,
}
impl SPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKR {
    #[doc = "RTC timer is clocked by CLK32K"]
    KHZ32_CLK,
    #[doc = "RTC timer is clocked by the trimmed clock"]
    TRIMMED_CLK,
}
impl CLKR {
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
            CLKR::KHZ32_CLK => false,
            CLKR::TRIMMED_CLK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKR {
        match value {
            false => CLKR::KHZ32_CLK,
            true => CLKR::TRIMMED_CLK,
        }
    }
    #[doc = "Checks if the value of the field is `KHZ32_CLK`"]
    #[inline]
    pub fn is_khz32_clk(&self) -> bool {
        *self == CLKR::KHZ32_CLK
    }
    #[doc = "Checks if the value of the field is `TRIMMED_CLK`"]
    #[inline]
    pub fn is_trimmed_clk(&self) -> bool {
        *self == CLKR::TRIMMED_CLK
    }
}
#[doc = r" Value of the field"]
pub struct BYPASS_GATEDR {
    bits: bool,
}
impl BYPASS_GATEDR {
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
#[doc = r" Proxy"]
pub struct _OSW<'a> {
    w: &'a mut W,
}
impl<'a> _OSW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SW<'a> {
    w: &'a mut W,
}
impl<'a> _SW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPW<'a> {
    w: &'a mut W,
}
impl<'a> _SPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLK`"]
pub enum CLKW {
    #[doc = "RTC timer is clocked by CLK32K"]
    KHZ32_CLK,
    #[doc = "RTC timer is clocked by the trimmed clock"]
    TRIMMED_CLK,
}
impl CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKW::KHZ32_CLK => false,
            CLKW::TRIMMED_CLK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTC timer is clocked by CLK32K"]
    #[inline]
    pub fn khz32_clk(self) -> &'a mut W {
        self.variant(CLKW::KHZ32_CLK)
    }
    #[doc = "RTC timer is clocked by the trimmed clock"]
    #[inline]
    pub fn trimmed_clk(self) -> &'a mut W {
        self.variant(CLKW::TRIMMED_CLK)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BYPASS_GATEDW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASS_GATEDW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - RTC Timer one shot count.<ul><li>0: Periodic mode (default). When reaching zero, the RTC timer raises its interrupt and is reloaded from the LD content.</li><li>1: One-shot mode. When reaching zero, the RTC timer raise its interrupt and stops.</li></ul>"]
    #[inline]
    pub fn os(&self) -> OSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        OSR { bits }
    }
    #[doc = "Bit 1 - RTC Timer enable bit.<ul><li>0: The RTC timer is stopped on the next CLK32K cycle.</li><li>1: The RTC timer is enabled on the next CLK32K cycle.</li></ul>When the RTC timer is stopped, the content of the counter is frozen. A read returns the value of the EN bit. This bit set by hardware when the TLR register is written to while the counter is stopped. When the device is active, this bit is cleared by hardware when the counter reaches zero in one-shot mode."]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        ENR { bits }
    }
    #[doc = "Bit 2 - RTC Timer self start bit. When written to 1b, each write in a load register or a pattern will set EN to 1b, so, start the counter in the next CLK32K cycle."]
    #[inline]
    pub fn s(&self) -> SR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        SR { bits }
    }
    #[doc = "Bits 4:10 - RTC Timer Pattern size. Number of pattern bits crossed by the pointer. It defines the useful pattern size."]
    #[inline]
    pub fn sp(&self) -> SPR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        SPR { bits }
    }
    #[doc = "Bit 11 - RTC Timer clock.<ul><li>0: The RTC timer is clocked by CLK32K.</li><li>1: The RTC timer is clocked by the trimmed clock.</li></ul>"]
    #[inline]
    pub fn clk(&self) -> CLKR {
        CLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - Enable or disable the internal clock gating:<ul><li>0: The internal clock gating is activated.</li><li>1: No clock gating, clock is always enabled.</li></ul>"]
    #[inline]
    pub fn bypass_gated(&self) -> BYPASS_GATEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        BYPASS_GATEDR { bits }
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - RTC Timer one shot count.<ul><li>0: Periodic mode (default). When reaching zero, the RTC timer raises its interrupt and is reloaded from the LD content.</li><li>1: One-shot mode. When reaching zero, the RTC timer raise its interrupt and stops.</li></ul>"]
    #[inline]
    pub fn os(&mut self) -> _OSW {
        _OSW { w: self }
    }
    #[doc = "Bit 1 - RTC Timer enable bit.<ul><li>0: The RTC timer is stopped on the next CLK32K cycle.</li><li>1: The RTC timer is enabled on the next CLK32K cycle.</li></ul>When the RTC timer is stopped, the content of the counter is frozen. A read returns the value of the EN bit. This bit set by hardware when the TLR register is written to while the counter is stopped. When the device is active, this bit is cleared by hardware when the counter reaches zero in one-shot mode."]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 2 - RTC Timer self start bit. When written to 1b, each write in a load register or a pattern will set EN to 1b, so, start the counter in the next CLK32K cycle."]
    #[inline]
    pub fn s(&mut self) -> _SW {
        _SW { w: self }
    }
    #[doc = "Bits 4:10 - RTC Timer Pattern size. Number of pattern bits crossed by the pointer. It defines the useful pattern size."]
    #[inline]
    pub fn sp(&mut self) -> _SPW {
        _SPW { w: self }
    }
    #[doc = "Bit 11 - RTC Timer clock.<ul><li>0: The RTC timer is clocked by CLK32K.</li><li>1: The RTC timer is clocked by the trimmed clock.</li></ul>"]
    #[inline]
    pub fn clk(&mut self) -> _CLKW {
        _CLKW { w: self }
    }
    #[doc = "Bit 12 - Enable or disable the internal clock gating:<ul><li>0: The internal clock gating is activated.</li><li>1: No clock gating, clock is always enabled.</li></ul>"]
    #[inline]
    pub fn bypass_gated(&mut self) -> _BYPASS_GATEDW {
        _BYPASS_GATEDW { w: self }
    }
}
