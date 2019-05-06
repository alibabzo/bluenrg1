#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::TNCKC {
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
#[doc = "Possible values of the field `TNC1CSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNC1CSELR {
    #[doc = "No clock (Timer/Counter 1 stopped)"]
    NO_CLOCK,
    #[doc = "Prescaled system clock pclk"]
    PRESCALED,
    #[doc = "External event on TnB"]
    EXTERNAL_EVENT,
    #[doc = "Pulse accumulate"]
    PULSE_ACCUMULATE,
    #[doc = "Low-speed clock slow_clk_c"]
    LOW_SPEED_CLOCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TNC1CSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TNC1CSELR::NO_CLOCK => 0,
            TNC1CSELR::PRESCALED => 1,
            TNC1CSELR::EXTERNAL_EVENT => 2,
            TNC1CSELR::PULSE_ACCUMULATE => 3,
            TNC1CSELR::LOW_SPEED_CLOCK => 4,
            TNC1CSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TNC1CSELR {
        match value {
            0 => TNC1CSELR::NO_CLOCK,
            1 => TNC1CSELR::PRESCALED,
            2 => TNC1CSELR::EXTERNAL_EVENT,
            3 => TNC1CSELR::PULSE_ACCUMULATE,
            4 => TNC1CSELR::LOW_SPEED_CLOCK,
            i => TNC1CSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK`"]
    #[inline]
    pub fn is_no_clock(&self) -> bool {
        *self == TNC1CSELR::NO_CLOCK
    }
    #[doc = "Checks if the value of the field is `PRESCALED`"]
    #[inline]
    pub fn is_prescaled(&self) -> bool {
        *self == TNC1CSELR::PRESCALED
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_EVENT`"]
    #[inline]
    pub fn is_external_event(&self) -> bool {
        *self == TNC1CSELR::EXTERNAL_EVENT
    }
    #[doc = "Checks if the value of the field is `PULSE_ACCUMULATE`"]
    #[inline]
    pub fn is_pulse_accumulate(&self) -> bool {
        *self == TNC1CSELR::PULSE_ACCUMULATE
    }
    #[doc = "Checks if the value of the field is `LOW_SPEED_CLOCK`"]
    #[inline]
    pub fn is_low_speed_clock(&self) -> bool {
        *self == TNC1CSELR::LOW_SPEED_CLOCK
    }
}
#[doc = r" Value of the field"]
pub struct TNC2CSELR {
    bits: u8,
}
impl TNC2CSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TNC1CSEL`"]
pub enum TNC1CSELW {
    #[doc = "No clock (Timer/Counter 1 stopped)"]
    NO_CLOCK,
    #[doc = "Prescaled system clock pclk"]
    PRESCALED,
    #[doc = "External event on TnB"]
    EXTERNAL_EVENT,
    #[doc = "Pulse accumulate"]
    PULSE_ACCUMULATE,
    #[doc = "Low-speed clock slow_clk_c"]
    LOW_SPEED_CLOCK,
}
impl TNC1CSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TNC1CSELW::NO_CLOCK => 0,
            TNC1CSELW::PRESCALED => 1,
            TNC1CSELW::EXTERNAL_EVENT => 2,
            TNC1CSELW::PULSE_ACCUMULATE => 3,
            TNC1CSELW::LOW_SPEED_CLOCK => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TNC1CSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TNC1CSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TNC1CSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No clock (Timer/Counter 1 stopped)"]
    #[inline]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(TNC1CSELW::NO_CLOCK)
    }
    #[doc = "Prescaled system clock pclk"]
    #[inline]
    pub fn prescaled(self) -> &'a mut W {
        self.variant(TNC1CSELW::PRESCALED)
    }
    #[doc = "External event on TnB"]
    #[inline]
    pub fn external_event(self) -> &'a mut W {
        self.variant(TNC1CSELW::EXTERNAL_EVENT)
    }
    #[doc = "Pulse accumulate"]
    #[inline]
    pub fn pulse_accumulate(self) -> &'a mut W {
        self.variant(TNC1CSELW::PULSE_ACCUMULATE)
    }
    #[doc = "Low-speed clock slow_clk_c"]
    #[inline]
    pub fn low_speed_clock(self) -> &'a mut W {
        self.variant(TNC1CSELW::LOW_SPEED_CLOCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TNC2CSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TNC2CSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
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
    #[doc = "Bits 0:2 - Define the clock mode for timer/counter 1:<ul><li>000b: No clock (Timer/Counter 1 stopped).</li><li>001b: Prescaled system clock.</li><li>010b: External event on TnB (mode 1 and 3 only).</li><li>011b: Pulse accumulate (mode 1 and 3 only).</li><li>100b: Low-speed clock.</li></ul>"]
    #[inline]
    pub fn tnc1csel(&self) -> TNC1CSELR {
        TNC1CSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 3:5 - Define the clock mode for timer/counter 2:<ul><li>000b: No clock (Timer/Counter 1 stopped).</li><li>001b: Prescaled system clock.</li><li>010b: External event on TnB (mode 1 and 3 only).</li><li>011b: Pulse accumulate (mode 1 and 3 only).</li><li>100b: Low-speed clock.</li></ul>"]
    #[inline]
    pub fn tnc2csel(&self) -> TNC2CSELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        TNC2CSELR { bits }
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
    #[doc = "Bits 0:2 - Define the clock mode for timer/counter 1:<ul><li>000b: No clock (Timer/Counter 1 stopped).</li><li>001b: Prescaled system clock.</li><li>010b: External event on TnB (mode 1 and 3 only).</li><li>011b: Pulse accumulate (mode 1 and 3 only).</li><li>100b: Low-speed clock.</li></ul>"]
    #[inline]
    pub fn tnc1csel(&mut self) -> _TNC1CSELW {
        _TNC1CSELW { w: self }
    }
    #[doc = "Bits 3:5 - Define the clock mode for timer/counter 2:<ul><li>000b: No clock (Timer/Counter 1 stopped).</li><li>001b: Prescaled system clock.</li><li>010b: External event on TnB (mode 1 and 3 only).</li><li>011b: Pulse accumulate (mode 1 and 3 only).</li><li>100b: Low-speed clock.</li></ul>"]
    #[inline]
    pub fn tnc2csel(&mut self) -> _TNC2CSELW {
        _TNC2CSELW { w: self }
    }
}
