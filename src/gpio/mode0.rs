#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE0 {
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
#[doc = "Possible values of the field `IO0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO0R {
    #[doc = "GPIO mode"]
    GPIO_MODE,
    #[doc = "serial1 mode"]
    SERIAL1_MODE,
    #[doc = "standalone mode"]
    STANDALONE_MODE,
    #[doc = "BLE mode"]
    BLUE_MODE,
    #[doc = "serial0 mode"]
    SERIAL0_MODE,
    #[doc = "ADC mode for microphone"]
    MICROPHONE_ADC_MODE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IO0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IO0R::GPIO_MODE => 0,
            IO0R::SERIAL1_MODE => 1,
            IO0R::STANDALONE_MODE => 2,
            IO0R::BLUE_MODE => 3,
            IO0R::SERIAL0_MODE => 4,
            IO0R::MICROPHONE_ADC_MODE => 5,
            IO0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IO0R {
        match value {
            0 => IO0R::GPIO_MODE,
            1 => IO0R::SERIAL1_MODE,
            2 => IO0R::STANDALONE_MODE,
            3 => IO0R::BLUE_MODE,
            4 => IO0R::SERIAL0_MODE,
            5 => IO0R::MICROPHONE_ADC_MODE,
            i => IO0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_MODE`"]
    #[inline]
    pub fn is_gpio_mode(&self) -> bool {
        *self == IO0R::GPIO_MODE
    }
    #[doc = "Checks if the value of the field is `SERIAL1_MODE`"]
    #[inline]
    pub fn is_serial1_mode(&self) -> bool {
        *self == IO0R::SERIAL1_MODE
    }
    #[doc = "Checks if the value of the field is `STANDALONE_MODE`"]
    #[inline]
    pub fn is_standalone_mode(&self) -> bool {
        *self == IO0R::STANDALONE_MODE
    }
    #[doc = "Checks if the value of the field is `BLUE_MODE`"]
    #[inline]
    pub fn is_blue_mode(&self) -> bool {
        *self == IO0R::BLUE_MODE
    }
    #[doc = "Checks if the value of the field is `SERIAL0_MODE`"]
    #[inline]
    pub fn is_serial0_mode(&self) -> bool {
        *self == IO0R::SERIAL0_MODE
    }
    #[doc = "Checks if the value of the field is `MICROPHONE_ADC_MODE`"]
    #[inline]
    pub fn is_microphone_adc_mode(&self) -> bool {
        *self == IO0R::MICROPHONE_ADC_MODE
    }
}
#[doc = r" Value of the field"]
pub struct IO1R {
    bits: u8,
}
impl IO1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IO2R {
    bits: u8,
}
impl IO2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IO3R {
    bits: u8,
}
impl IO3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IO4R {
    bits: u8,
}
impl IO4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IO5R {
    bits: u8,
}
impl IO5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IO6R {
    bits: u8,
}
impl IO6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IO7R {
    bits: u8,
}
impl IO7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `IO0`"]
pub enum IO0W {
    #[doc = "GPIO mode"]
    GPIO_MODE,
    #[doc = "serial1 mode"]
    SERIAL1_MODE,
    #[doc = "standalone mode"]
    STANDALONE_MODE,
    #[doc = "BLE mode"]
    BLUE_MODE,
    #[doc = "serial0 mode"]
    SERIAL0_MODE,
    #[doc = "ADC mode for microphone"]
    MICROPHONE_ADC_MODE,
}
impl IO0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IO0W::GPIO_MODE => 0,
            IO0W::SERIAL1_MODE => 1,
            IO0W::STANDALONE_MODE => 2,
            IO0W::BLUE_MODE => 3,
            IO0W::SERIAL0_MODE => 4,
            IO0W::MICROPHONE_ADC_MODE => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IO0W<'a> {
    w: &'a mut W,
}
impl<'a> _IO0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IO0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO mode"]
    #[inline]
    pub fn gpio_mode(self) -> &'a mut W {
        self.variant(IO0W::GPIO_MODE)
    }
    #[doc = "serial1 mode"]
    #[inline]
    pub fn serial1_mode(self) -> &'a mut W {
        self.variant(IO0W::SERIAL1_MODE)
    }
    #[doc = "standalone mode"]
    #[inline]
    pub fn standalone_mode(self) -> &'a mut W {
        self.variant(IO0W::STANDALONE_MODE)
    }
    #[doc = "BLE mode"]
    #[inline]
    pub fn blue_mode(self) -> &'a mut W {
        self.variant(IO0W::BLUE_MODE)
    }
    #[doc = "serial0 mode"]
    #[inline]
    pub fn serial0_mode(self) -> &'a mut W {
        self.variant(IO0W::SERIAL0_MODE)
    }
    #[doc = "ADC mode for microphone"]
    #[inline]
    pub fn microphone_adc_mode(self) -> &'a mut W {
        self.variant(IO0W::MICROPHONE_ADC_MODE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IO1W<'a> {
    w: &'a mut W,
}
impl<'a> _IO1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IO2W<'a> {
    w: &'a mut W,
}
impl<'a> _IO2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IO3W<'a> {
    w: &'a mut W,
}
impl<'a> _IO3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IO4W<'a> {
    w: &'a mut W,
}
impl<'a> _IO4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IO5W<'a> {
    w: &'a mut W,
}
impl<'a> _IO5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IO6W<'a> {
    w: &'a mut W,
}
impl<'a> _IO6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IO7W<'a> {
    w: &'a mut W,
}
impl<'a> _IO7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - IO0 mode"]
    #[inline]
    pub fn io0(&self) -> IO0R {
        IO0R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - IO1 mode"]
    #[inline]
    pub fn io1(&self) -> IO1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IO1R { bits }
    }
    #[doc = "Bits 8:10 - IO2 mode"]
    #[inline]
    pub fn io2(&self) -> IO2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IO2R { bits }
    }
    #[doc = "Bits 12:14 - IO3 mode"]
    #[inline]
    pub fn io3(&self) -> IO3R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IO3R { bits }
    }
    #[doc = "Bits 16:18 - IO4 mode"]
    #[inline]
    pub fn io4(&self) -> IO4R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IO4R { bits }
    }
    #[doc = "Bits 20:22 - IO5 mode"]
    #[inline]
    pub fn io5(&self) -> IO5R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IO5R { bits }
    }
    #[doc = "Bits 24:26 - IO6 mode"]
    #[inline]
    pub fn io6(&self) -> IO6R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IO6R { bits }
    }
    #[doc = "Bits 28:30 - IO7 mode"]
    #[inline]
    pub fn io7(&self) -> IO7R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IO7R { bits }
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
    #[doc = "Bits 0:2 - IO0 mode"]
    #[inline]
    pub fn io0(&mut self) -> _IO0W {
        _IO0W { w: self }
    }
    #[doc = "Bits 4:6 - IO1 mode"]
    #[inline]
    pub fn io1(&mut self) -> _IO1W {
        _IO1W { w: self }
    }
    #[doc = "Bits 8:10 - IO2 mode"]
    #[inline]
    pub fn io2(&mut self) -> _IO2W {
        _IO2W { w: self }
    }
    #[doc = "Bits 12:14 - IO3 mode"]
    #[inline]
    pub fn io3(&mut self) -> _IO3W {
        _IO3W { w: self }
    }
    #[doc = "Bits 16:18 - IO4 mode"]
    #[inline]
    pub fn io4(&mut self) -> _IO4W {
        _IO4W { w: self }
    }
    #[doc = "Bits 20:22 - IO5 mode"]
    #[inline]
    pub fn io5(&mut self) -> _IO5W {
        _IO5W { w: self }
    }
    #[doc = "Bits 24:26 - IO6 mode"]
    #[inline]
    pub fn io6(&mut self) -> _IO6W {
        _IO6W { w: self }
    }
    #[doc = "Bits 28:30 - IO7 mode"]
    #[inline]
    pub fn io7(&mut self) -> _IO7W {
        _IO7W { w: self }
    }
}
