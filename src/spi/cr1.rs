#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
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
#[doc = "Possible values of the field `SSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSER {
    #[doc = "SSP operation disable"]
    DISABLE,
    #[doc = "SSP operation enable"]
    ENABLE,
}
impl SSER {
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
            SSER::DISABLE => false,
            SSER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSER {
        match value {
            false => SSER::DISABLE,
            true => SSER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SSER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SSER::ENABLE
    }
}
#[doc = "Possible values of the field `MS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSR {
    #[doc = "Master mode"]
    MASTER,
    #[doc = "Slave mode"]
    SLAVE,
}
impl MSR {
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
            MSR::MASTER => false,
            MSR::SLAVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSR {
        match value {
            false => MSR::MASTER,
            true => MSR::SLAVE,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline]
    pub fn is_master(&self) -> bool {
        *self == MSR::MASTER
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline]
    pub fn is_slave(&self) -> bool {
        *self == MSR::SLAVE
    }
}
#[doc = r" Value of the field"]
pub struct SODR {
    bits: bool,
}
impl SODR {
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
#[doc = "Possible values of the field `RENDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RENDNR {
    #[doc = "The element is received MSByte-first and MSbit-first"]
    MSB_FIRST_MSB_FIRST,
    #[doc = "The element is received LSByte-first and MSbit-first"]
    LSB_FIRST_MSB_FIRST,
    #[doc = "The element is received MSByte-first and LSbit-first"]
    MSB_FIRST_LSB_FIRST,
    #[doc = "The element is received LSByte-first and LSbit-first"]
    LSB_FIRST_LSB_FIRST,
}
impl RENDNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RENDNR::MSB_FIRST_MSB_FIRST => 0,
            RENDNR::LSB_FIRST_MSB_FIRST => 1,
            RENDNR::MSB_FIRST_LSB_FIRST => 2,
            RENDNR::LSB_FIRST_LSB_FIRST => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RENDNR {
        match value {
            0 => RENDNR::MSB_FIRST_MSB_FIRST,
            1 => RENDNR::LSB_FIRST_MSB_FIRST,
            2 => RENDNR::MSB_FIRST_LSB_FIRST,
            3 => RENDNR::LSB_FIRST_LSB_FIRST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MSB_FIRST_MSB_FIRST`"]
    #[inline]
    pub fn is_msb_first_msb_first(&self) -> bool {
        *self == RENDNR::MSB_FIRST_MSB_FIRST
    }
    #[doc = "Checks if the value of the field is `LSB_FIRST_MSB_FIRST`"]
    #[inline]
    pub fn is_lsb_first_msb_first(&self) -> bool {
        *self == RENDNR::LSB_FIRST_MSB_FIRST
    }
    #[doc = "Checks if the value of the field is `MSB_FIRST_LSB_FIRST`"]
    #[inline]
    pub fn is_msb_first_lsb_first(&self) -> bool {
        *self == RENDNR::MSB_FIRST_LSB_FIRST
    }
    #[doc = "Checks if the value of the field is `LSB_FIRST_LSB_FIRST`"]
    #[inline]
    pub fn is_lsb_first_lsb_first(&self) -> bool {
        *self == RENDNR::LSB_FIRST_LSB_FIRST
    }
}
#[doc = "Possible values of the field `RXIFLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIFLSELR {
    #[doc = "Rx FIFO contains 1 element or more"]
    MIN_1ELEMENT,
    #[doc = "Rx FIFO contains 4 elements or more"]
    MIN_4ELEMENTS,
    #[doc = "Rx FIFO contains 8 elements or more"]
    MIN_8ELEMENTS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXIFLSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXIFLSELR::MIN_1ELEMENT => 0,
            RXIFLSELR::MIN_4ELEMENTS => 1,
            RXIFLSELR::MIN_8ELEMENTS => 2,
            RXIFLSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXIFLSELR {
        match value {
            0 => RXIFLSELR::MIN_1ELEMENT,
            1 => RXIFLSELR::MIN_4ELEMENTS,
            2 => RXIFLSELR::MIN_8ELEMENTS,
            i => RXIFLSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MIN_1ELEMENT`"]
    #[inline]
    pub fn is_min_1element(&self) -> bool {
        *self == RXIFLSELR::MIN_1ELEMENT
    }
    #[doc = "Checks if the value of the field is `MIN_4ELEMENTS`"]
    #[inline]
    pub fn is_min_4elements(&self) -> bool {
        *self == RXIFLSELR::MIN_4ELEMENTS
    }
    #[doc = "Checks if the value of the field is `MIN_8ELEMENTS`"]
    #[inline]
    pub fn is_min_8elements(&self) -> bool {
        *self == RXIFLSELR::MIN_8ELEMENTS
    }
}
#[doc = "Possible values of the field `TXIFLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIFLSELR {
    #[doc = "Tx FIFO contains 1 element or more"]
    MIN_1ELEMENT,
    #[doc = "Tx FIFO contains 4 elements or more"]
    MIN_4ELEMENTS,
    #[doc = "Tx FIFO contains 8 elements or more"]
    MIN_8ELEMENTS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXIFLSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXIFLSELR::MIN_1ELEMENT => 0,
            TXIFLSELR::MIN_4ELEMENTS => 1,
            TXIFLSELR::MIN_8ELEMENTS => 2,
            TXIFLSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXIFLSELR {
        match value {
            0 => TXIFLSELR::MIN_1ELEMENT,
            1 => TXIFLSELR::MIN_4ELEMENTS,
            2 => TXIFLSELR::MIN_8ELEMENTS,
            i => TXIFLSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MIN_1ELEMENT`"]
    #[inline]
    pub fn is_min_1element(&self) -> bool {
        *self == TXIFLSELR::MIN_1ELEMENT
    }
    #[doc = "Checks if the value of the field is `MIN_4ELEMENTS`"]
    #[inline]
    pub fn is_min_4elements(&self) -> bool {
        *self == TXIFLSELR::MIN_4ELEMENTS
    }
    #[doc = "Checks if the value of the field is `MIN_8ELEMENTS`"]
    #[inline]
    pub fn is_min_8elements(&self) -> bool {
        *self == TXIFLSELR::MIN_8ELEMENTS
    }
}
#[doc = "Possible values of the field `FLOWCTRLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLOWCTRLENR {
    #[doc = "Flow control disable"]
    DISABLE,
    #[doc = "Flow control enable"]
    ENABLE,
}
impl FLOWCTRLENR {
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
            FLOWCTRLENR::DISABLE => false,
            FLOWCTRLENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLOWCTRLENR {
        match value {
            false => FLOWCTRLENR::DISABLE,
            true => FLOWCTRLENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FLOWCTRLENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FLOWCTRLENR::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct MSPIWAITR {
    bits: u8,
}
impl MSPIWAITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TENDNR {
    bits: u8,
}
impl TENDNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATAINDELR {
    bits: bool,
}
impl DATAINDELR {
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
#[doc = "Values that can be written to the field `SSE`"]
pub enum SSEW {
    #[doc = "SSP operation disable"]
    DISABLE,
    #[doc = "SSP operation enable"]
    ENABLE,
}
impl SSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSEW::DISABLE => false,
            SSEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SSP operation disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SSEW::DISABLE)
    }
    #[doc = "SSP operation enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SSEW::ENABLE)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MS`"]
pub enum MSW {
    #[doc = "Master mode"]
    MASTER,
    #[doc = "Slave mode"]
    SLAVE,
}
impl MSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSW::MASTER => false,
            MSW::SLAVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSW<'a> {
    w: &'a mut W,
}
impl<'a> _MSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Master mode"]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(MSW::MASTER)
    }
    #[doc = "Slave mode"]
    #[inline]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSW::SLAVE)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SODW<'a> {
    w: &'a mut W,
}
impl<'a> _SODW<'a> {
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
#[doc = "Values that can be written to the field `RENDN`"]
pub enum RENDNW {
    #[doc = "The element is received MSByte-first and MSbit-first"]
    MSB_FIRST_MSB_FIRST,
    #[doc = "The element is received LSByte-first and MSbit-first"]
    LSB_FIRST_MSB_FIRST,
    #[doc = "The element is received MSByte-first and LSbit-first"]
    MSB_FIRST_LSB_FIRST,
    #[doc = "The element is received LSByte-first and LSbit-first"]
    LSB_FIRST_LSB_FIRST,
}
impl RENDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RENDNW::MSB_FIRST_MSB_FIRST => 0,
            RENDNW::LSB_FIRST_MSB_FIRST => 1,
            RENDNW::MSB_FIRST_LSB_FIRST => 2,
            RENDNW::LSB_FIRST_LSB_FIRST => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RENDNW<'a> {
    w: &'a mut W,
}
impl<'a> _RENDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RENDNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The element is received MSByte-first and MSbit-first"]
    #[inline]
    pub fn msb_first_msb_first(self) -> &'a mut W {
        self.variant(RENDNW::MSB_FIRST_MSB_FIRST)
    }
    #[doc = "The element is received LSByte-first and MSbit-first"]
    #[inline]
    pub fn lsb_first_msb_first(self) -> &'a mut W {
        self.variant(RENDNW::LSB_FIRST_MSB_FIRST)
    }
    #[doc = "The element is received MSByte-first and LSbit-first"]
    #[inline]
    pub fn msb_first_lsb_first(self) -> &'a mut W {
        self.variant(RENDNW::MSB_FIRST_LSB_FIRST)
    }
    #[doc = "The element is received LSByte-first and LSbit-first"]
    #[inline]
    pub fn lsb_first_lsb_first(self) -> &'a mut W {
        self.variant(RENDNW::LSB_FIRST_LSB_FIRST)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXIFLSEL`"]
pub enum RXIFLSELW {
    #[doc = "Rx FIFO contains 1 element or more"]
    MIN_1ELEMENT,
    #[doc = "Rx FIFO contains 4 elements or more"]
    MIN_4ELEMENTS,
    #[doc = "Rx FIFO contains 8 elements or more"]
    MIN_8ELEMENTS,
}
impl RXIFLSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXIFLSELW::MIN_1ELEMENT => 0,
            RXIFLSELW::MIN_4ELEMENTS => 1,
            RXIFLSELW::MIN_8ELEMENTS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIFLSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIFLSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIFLSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Rx FIFO contains 1 element or more"]
    #[inline]
    pub fn min_1element(self) -> &'a mut W {
        self.variant(RXIFLSELW::MIN_1ELEMENT)
    }
    #[doc = "Rx FIFO contains 4 elements or more"]
    #[inline]
    pub fn min_4elements(self) -> &'a mut W {
        self.variant(RXIFLSELW::MIN_4ELEMENTS)
    }
    #[doc = "Rx FIFO contains 8 elements or more"]
    #[inline]
    pub fn min_8elements(self) -> &'a mut W {
        self.variant(RXIFLSELW::MIN_8ELEMENTS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXIFLSEL`"]
pub enum TXIFLSELW {
    #[doc = "Tx FIFO contains 1 element or more"]
    MIN_1ELEMENT,
    #[doc = "Tx FIFO contains 4 elements or more"]
    MIN_4ELEMENTS,
    #[doc = "Tx FIFO contains 8 elements or more"]
    MIN_8ELEMENTS,
}
impl TXIFLSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXIFLSELW::MIN_1ELEMENT => 0,
            TXIFLSELW::MIN_4ELEMENTS => 1,
            TXIFLSELW::MIN_8ELEMENTS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXIFLSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TXIFLSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXIFLSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Tx FIFO contains 1 element or more"]
    #[inline]
    pub fn min_1element(self) -> &'a mut W {
        self.variant(TXIFLSELW::MIN_1ELEMENT)
    }
    #[doc = "Tx FIFO contains 4 elements or more"]
    #[inline]
    pub fn min_4elements(self) -> &'a mut W {
        self.variant(TXIFLSELW::MIN_4ELEMENTS)
    }
    #[doc = "Tx FIFO contains 8 elements or more"]
    #[inline]
    pub fn min_8elements(self) -> &'a mut W {
        self.variant(TXIFLSELW::MIN_8ELEMENTS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLOWCTRLEN`"]
pub enum FLOWCTRLENW {
    #[doc = "Flow control disable"]
    DISABLE,
    #[doc = "Flow control enable"]
    ENABLE,
}
impl FLOWCTRLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLOWCTRLENW::DISABLE => false,
            FLOWCTRLENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLOWCTRLENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLOWCTRLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLOWCTRLENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flow control disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLOWCTRLENW::DISABLE)
    }
    #[doc = "Flow control enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLOWCTRLENW::ENABLE)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MSPIWAITW<'a> {
    w: &'a mut W,
}
impl<'a> _MSPIWAITW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TENDNW<'a> {
    w: &'a mut W,
}
impl<'a> _TENDNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATAINDELW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAINDELW<'a> {
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
        const OFFSET: u8 = 21;
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
    #[doc = "Bit 1 - SPI enable.<ul><li>0: SPI disable.</li><li>1: SPI enable.</li></ul>"]
    #[inline]
    pub fn sse(&self) -> SSER {
        SSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Master or slave mode select.<ul><li>0: Master mode.</li><li>1: Slave mode.</li></ul>"]
    #[inline]
    pub fn ms(&self) -> MSR {
        MSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Slave mode output disable (slave mode only).<ul><li>0: SPI can drive the MISO signal in slave mode.</li><li>1: SPI must not drive the MISO signal in slave mode.</li></ul>In multiple slave system, it is possible for a SPI master to broadcast a message to all slaves in the system while ensuring only one slave drives data onto the serial output line MISO."]
    #[inline]
    pub fn sod(&self) -> SODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SODR { bits }
    }
    #[doc = "Bits 4:5 - Receive endian format.<ul><li>00b: The element is received MSByte-first and MSbit-first.</li><li>01b: The element is received LSByte-first and MSbit-first.</li><li>10b: The element is received MSByte-first and LSbit-first.</li><li>11b: The element is received LSByte-first and LSbit-first.</li></ul>The cases 00b and 11b are set for data frame size from 4 to 32 bits. The cases 01b and 10b are set only for data frame size 16, 24 and 32 bits."]
    #[inline]
    pub fn rendn(&self) -> RENDNR {
        RENDNR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:9 - Receive interrupt FIFO level select. This bit field selects the trigger points to receive FIFO interrupt:<ul><li>000b: RX FIFO contains 1 element or more.</li><li>001b: RX FIFO contains 4 elements or more.</li><li>010b: RX FIFO contains 8 elements or more.</li><li>Others: Reserved.</li></ul>"]
    #[inline]
    pub fn rxiflsel(&self) -> RXIFLSELR {
        RXIFLSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:12 - Transmit interrupt FIFO level select. This bit field selects the trigger points to transmit FIFO interrupt:<ul><li>000b: TX FIFO contains 1 element or more.</li><li>001b: TX FIFO contains 4 elements or more.</li><li>010b: TX FIFO contains 8 elements or more.</li><li>Others: Reserved.</li></ul>"]
    #[inline]
    pub fn txiflsel(&self) -> TXIFLSELR {
        TXIFLSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 13 - Flow Control Enable.<ul><li>0: Flow control is disabled.</li><li>1: Flow control is enabled.</li></ul>"]
    #[inline]
    pub fn flowctrlen(&self) -> FLOWCTRLENR {
        FLOWCTRLENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:17 - SPI Wait mode. This value is used to insert a wait state between frames."]
    #[inline]
    pub fn mspiwait(&self) -> MSPIWAITR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MSPIWAITR { bits }
    }
    #[doc = "Bits 18:19 - Transmit endian format.<ul><li>00b: The element is transmitted MSByte-first and MSbit-first.</li><li>01b: The element is transmitted LSByte-first and MSbit-first.</li><li>10b: The element is transmitted MSByte-first and LSbit-first.</li><li>11b: The element is transmitted LSByte-first and LSbit-first.</li></ul>The cases 00b and 11b are set for data frame size from 4 to 32 bits. The cases 01b and 10b are set only for data frame size 16, 24 and 32 bits."]
    #[inline]
    pub fn tendn(&self) -> TENDNR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TENDNR { bits }
    }
    #[doc = "Bit 21 - Data input delay.<ul><li>0: No delay is inserted in data input.</li><li>1: A delay of 2 clock cycles is inserted in the data input path.</li></ul>"]
    #[inline]
    pub fn dataindel(&self) -> DATAINDELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATAINDELR { bits }
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
    #[doc = "Bit 1 - SPI enable.<ul><li>0: SPI disable.</li><li>1: SPI enable.</li></ul>"]
    #[inline]
    pub fn sse(&mut self) -> _SSEW {
        _SSEW { w: self }
    }
    #[doc = "Bit 2 - Master or slave mode select.<ul><li>0: Master mode.</li><li>1: Slave mode.</li></ul>"]
    #[inline]
    pub fn ms(&mut self) -> _MSW {
        _MSW { w: self }
    }
    #[doc = "Bit 3 - Slave mode output disable (slave mode only).<ul><li>0: SPI can drive the MISO signal in slave mode.</li><li>1: SPI must not drive the MISO signal in slave mode.</li></ul>In multiple slave system, it is possible for a SPI master to broadcast a message to all slaves in the system while ensuring only one slave drives data onto the serial output line MISO."]
    #[inline]
    pub fn sod(&mut self) -> _SODW {
        _SODW { w: self }
    }
    #[doc = "Bits 4:5 - Receive endian format.<ul><li>00b: The element is received MSByte-first and MSbit-first.</li><li>01b: The element is received LSByte-first and MSbit-first.</li><li>10b: The element is received MSByte-first and LSbit-first.</li><li>11b: The element is received LSByte-first and LSbit-first.</li></ul>The cases 00b and 11b are set for data frame size from 4 to 32 bits. The cases 01b and 10b are set only for data frame size 16, 24 and 32 bits."]
    #[inline]
    pub fn rendn(&mut self) -> _RENDNW {
        _RENDNW { w: self }
    }
    #[doc = "Bits 7:9 - Receive interrupt FIFO level select. This bit field selects the trigger points to receive FIFO interrupt:<ul><li>000b: RX FIFO contains 1 element or more.</li><li>001b: RX FIFO contains 4 elements or more.</li><li>010b: RX FIFO contains 8 elements or more.</li><li>Others: Reserved.</li></ul>"]
    #[inline]
    pub fn rxiflsel(&mut self) -> _RXIFLSELW {
        _RXIFLSELW { w: self }
    }
    #[doc = "Bits 10:12 - Transmit interrupt FIFO level select. This bit field selects the trigger points to transmit FIFO interrupt:<ul><li>000b: TX FIFO contains 1 element or more.</li><li>001b: TX FIFO contains 4 elements or more.</li><li>010b: TX FIFO contains 8 elements or more.</li><li>Others: Reserved.</li></ul>"]
    #[inline]
    pub fn txiflsel(&mut self) -> _TXIFLSELW {
        _TXIFLSELW { w: self }
    }
    #[doc = "Bit 13 - Flow Control Enable.<ul><li>0: Flow control is disabled.</li><li>1: Flow control is enabled.</li></ul>"]
    #[inline]
    pub fn flowctrlen(&mut self) -> _FLOWCTRLENW {
        _FLOWCTRLENW { w: self }
    }
    #[doc = "Bits 14:17 - SPI Wait mode. This value is used to insert a wait state between frames."]
    #[inline]
    pub fn mspiwait(&mut self) -> _MSPIWAITW {
        _MSPIWAITW { w: self }
    }
    #[doc = "Bits 18:19 - Transmit endian format.<ul><li>00b: The element is transmitted MSByte-first and MSbit-first.</li><li>01b: The element is transmitted LSByte-first and MSbit-first.</li><li>10b: The element is transmitted MSByte-first and LSbit-first.</li><li>11b: The element is transmitted LSByte-first and LSbit-first.</li></ul>The cases 00b and 11b are set for data frame size from 4 to 32 bits. The cases 01b and 10b are set only for data frame size 16, 24 and 32 bits."]
    #[inline]
    pub fn tendn(&mut self) -> _TENDNW {
        _TENDNW { w: self }
    }
    #[doc = "Bit 21 - Data input delay.<ul><li>0: No delay is inserted in data input.</li><li>1: A delay of 2 clock cycles is inserted in the data input path.</li></ul>"]
    #[inline]
    pub fn dataindel(&mut self) -> _DATAINDELW {
        _DATAINDELW { w: self }
    }
}
