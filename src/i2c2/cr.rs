#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "I2C disable"]
    DISABLE,
    #[doc = "I2C enable"]
    ENABLE,
}
impl PER {
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
            PER::DISABLE => false,
            PER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER {
        match value {
            false => PER::DISABLE,
            true => PER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PER::ENABLE
    }
}
#[doc = "Possible values of the field `OM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OMR {
    #[doc = "The peripheral can only respond (transmit/receive) when addressed by a master device"]
    SLAVE,
    #[doc = "The peripheral works in a multi-master system where itself cannot be addressed by another master device. It can only initiate a new transfer as master device"]
    MASTER,
    #[doc = "The peripheral works in a multi-master system where itself can be addressed by another master device, besides to initiate a transfer as master device"]
    MASTER_SLAVE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OMR::SLAVE => 0,
            OMR::MASTER => 1,
            OMR::MASTER_SLAVE => 2,
            OMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OMR {
        match value {
            0 => OMR::SLAVE,
            1 => OMR::MASTER,
            2 => OMR::MASTER_SLAVE,
            i => OMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline]
    pub fn is_slave(&self) -> bool {
        *self == OMR::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline]
    pub fn is_master(&self) -> bool {
        *self == OMR::MASTER
    }
    #[doc = "Checks if the value of the field is `MASTER_SLAVE`"]
    #[inline]
    pub fn is_master_slave(&self) -> bool {
        *self == OMR::MASTER_SLAVE
    }
}
#[doc = "Possible values of the field `SAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMR {
    #[doc = "7-bit addressing mode"]
    ADDR_7BIT,
    #[doc = "10-bit addressing mode"]
    ADDR_10BIT,
}
impl SAMR {
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
            SAMR::ADDR_7BIT => false,
            SAMR::ADDR_10BIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAMR {
        match value {
            false => SAMR::ADDR_7BIT,
            true => SAMR::ADDR_10BIT,
        }
    }
    #[doc = "Checks if the value of the field is `ADDR_7BIT`"]
    #[inline]
    pub fn is_addr_7bit(&self) -> bool {
        *self == SAMR::ADDR_7BIT
    }
    #[doc = "Checks if the value of the field is `ADDR_10BIT`"]
    #[inline]
    pub fn is_addr_10bit(&self) -> bool {
        *self == SAMR::ADDR_10BIT
    }
}
#[doc = "Possible values of the field `SM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMR {
    #[doc = "Standard mode (up to 100 K/s)"]
    STANDARD_MODE,
    #[doc = "Fast mode (up to 400 K/s)"]
    FAST_MODE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMR::STANDARD_MODE => 0,
            SMR::FAST_MODE => 1,
            SMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMR {
        match value {
            0 => SMR::STANDARD_MODE,
            1 => SMR::FAST_MODE,
            i => SMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD_MODE`"]
    #[inline]
    pub fn is_standard_mode(&self) -> bool {
        *self == SMR::STANDARD_MODE
    }
    #[doc = "Checks if the value of the field is `FAST_MODE`"]
    #[inline]
    pub fn is_fast_mode(&self) -> bool {
        *self == SMR::FAST_MODE
    }
}
#[doc = "Possible values of the field `SGCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SGCMR {
    #[doc = "Transparent mode, the slave receiver recognizes the general call ut any action is taken by software after the decoding of the message included in the Rx FIFO"]
    TRANSPARENT_MODE,
    #[doc = "Direct mode, the slave receiver recognizes the general call and executes directly (without software intervention) the related actions. Only the status code word is stored in the SR register for notification to the application"]
    DIRECT_MODE,
}
impl SGCMR {
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
            SGCMR::TRANSPARENT_MODE => false,
            SGCMR::DIRECT_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SGCMR {
        match value {
            false => SGCMR::TRANSPARENT_MODE,
            true => SGCMR::DIRECT_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSPARENT_MODE`"]
    #[inline]
    pub fn is_transparent_mode(&self) -> bool {
        *self == SGCMR::TRANSPARENT_MODE
    }
    #[doc = "Checks if the value of the field is `DIRECT_MODE`"]
    #[inline]
    pub fn is_direct_mode(&self) -> bool {
        *self == SGCMR::DIRECT_MODE
    }
}
#[doc = r" Value of the field"]
pub struct FTXR {
    bits: bool,
}
impl FTXR {
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
pub struct FRXR {
    bits: bool,
}
impl FRXR {
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
#[doc = "Possible values of the field `DMA_TX_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_TX_ENR {
    #[doc = "DMA TX interface disable"]
    DISABLE,
    #[doc = "DMA TX interface enable"]
    ENABLE,
}
impl DMA_TX_ENR {
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
            DMA_TX_ENR::DISABLE => false,
            DMA_TX_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_TX_ENR {
        match value {
            false => DMA_TX_ENR::DISABLE,
            true => DMA_TX_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DMA_TX_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DMA_TX_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `DMA_RX_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_RX_ENR {
    #[doc = "DMA RX interface disable"]
    DISABLE,
    #[doc = "DMA RX interface enable"]
    ENABLE,
}
impl DMA_RX_ENR {
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
            DMA_RX_ENR::DISABLE => false,
            DMA_RX_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_RX_ENR {
        match value {
            false => DMA_RX_ENR::DISABLE,
            true => DMA_RX_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DMA_RX_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DMA_RX_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `FON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FONR {
    #[doc = "No digital filters are inserted"]
    NONE,
    #[doc = "Digital filters (filter 1 ck wide spikes) are inserted"]
    CK1_SPIKES,
    #[doc = "Digital filters (filter 2 ck wide spikes) are inserted"]
    CK2_SPIKES,
    #[doc = "Digital filters (filter 4 ck wide spikes) are inserted"]
    CK4_SPIKES,
}
impl FONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FONR::NONE => 0,
            FONR::CK1_SPIKES => 1,
            FONR::CK2_SPIKES => 2,
            FONR::CK4_SPIKES => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FONR {
        match value {
            0 => FONR::NONE,
            1 => FONR::CK1_SPIKES,
            2 => FONR::CK2_SPIKES,
            3 => FONR::CK4_SPIKES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == FONR::NONE
    }
    #[doc = "Checks if the value of the field is `CK1_SPIKES`"]
    #[inline]
    pub fn is_ck1_spikes(&self) -> bool {
        *self == FONR::CK1_SPIKES
    }
    #[doc = "Checks if the value of the field is `CK2_SPIKES`"]
    #[inline]
    pub fn is_ck2_spikes(&self) -> bool {
        *self == FONR::CK2_SPIKES
    }
    #[doc = "Checks if the value of the field is `CK4_SPIKES`"]
    #[inline]
    pub fn is_ck4_spikes(&self) -> bool {
        *self == FONR::CK4_SPIKES
    }
}
#[doc = "Possible values of the field `FS_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FS_1R {
    #[doc = "Force stop disable"]
    DISABLE,
    #[doc = "Force stop enable"]
    ENABLE,
}
impl FS_1R {
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
            FS_1R::DISABLE => false,
            FS_1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FS_1R {
        match value {
            false => FS_1R::DISABLE,
            true => FS_1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FS_1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FS_1R::ENABLE
    }
}
#[doc = "Values that can be written to the field `PE`"]
pub enum PEW {
    #[doc = "I2C disable"]
    DISABLE,
    #[doc = "I2C enable"]
    ENABLE,
}
impl PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEW::DISABLE => false,
            PEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2C disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PEW::DISABLE)
    }
    #[doc = "I2C enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PEW::ENABLE)
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
#[doc = "Values that can be written to the field `OM`"]
pub enum OMW {
    #[doc = "The peripheral can only respond (transmit/receive) when addressed by a master device"]
    SLAVE,
    #[doc = "The peripheral works in a multi-master system where itself cannot be addressed by another master device. It can only initiate a new transfer as master device"]
    MASTER,
    #[doc = "The peripheral works in a multi-master system where itself can be addressed by another master device, besides to initiate a transfer as master device"]
    MASTER_SLAVE,
}
impl OMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OMW::SLAVE => 0,
            OMW::MASTER => 1,
            OMW::MASTER_SLAVE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OMW<'a> {
    w: &'a mut W,
}
impl<'a> _OMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The peripheral can only respond (transmit/receive) when addressed by a master device"]
    #[inline]
    pub fn slave(self) -> &'a mut W {
        self.variant(OMW::SLAVE)
    }
    #[doc = "The peripheral works in a multi-master system where itself cannot be addressed by another master device. It can only initiate a new transfer as master device"]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(OMW::MASTER)
    }
    #[doc = "The peripheral works in a multi-master system where itself can be addressed by another master device, besides to initiate a transfer as master device"]
    #[inline]
    pub fn master_slave(self) -> &'a mut W {
        self.variant(OMW::MASTER_SLAVE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAM`"]
pub enum SAMW {
    #[doc = "7-bit addressing mode"]
    ADDR_7BIT,
    #[doc = "10-bit addressing mode"]
    ADDR_10BIT,
}
impl SAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAMW::ADDR_7BIT => false,
            SAMW::ADDR_10BIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "7-bit addressing mode"]
    #[inline]
    pub fn addr_7bit(self) -> &'a mut W {
        self.variant(SAMW::ADDR_7BIT)
    }
    #[doc = "10-bit addressing mode"]
    #[inline]
    pub fn addr_10bit(self) -> &'a mut W {
        self.variant(SAMW::ADDR_10BIT)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM`"]
pub enum SMW {
    #[doc = "Standard mode (up to 100 K/s)"]
    STANDARD_MODE,
    #[doc = "Fast mode (up to 400 K/s)"]
    FAST_MODE,
}
impl SMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMW::STANDARD_MODE => 0,
            SMW::FAST_MODE => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMW<'a> {
    w: &'a mut W,
}
impl<'a> _SMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Standard mode (up to 100 K/s)"]
    #[inline]
    pub fn standard_mode(self) -> &'a mut W {
        self.variant(SMW::STANDARD_MODE)
    }
    #[doc = "Fast mode (up to 400 K/s)"]
    #[inline]
    pub fn fast_mode(self) -> &'a mut W {
        self.variant(SMW::FAST_MODE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SGCM`"]
pub enum SGCMW {
    #[doc = "Transparent mode, the slave receiver recognizes the general call ut any action is taken by software after the decoding of the message included in the Rx FIFO"]
    TRANSPARENT_MODE,
    #[doc = "Direct mode, the slave receiver recognizes the general call and executes directly (without software intervention) the related actions. Only the status code word is stored in the SR register for notification to the application"]
    DIRECT_MODE,
}
impl SGCMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SGCMW::TRANSPARENT_MODE => false,
            SGCMW::DIRECT_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SGCMW<'a> {
    w: &'a mut W,
}
impl<'a> _SGCMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SGCMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transparent mode, the slave receiver recognizes the general call ut any action is taken by software after the decoding of the message included in the Rx FIFO"]
    #[inline]
    pub fn transparent_mode(self) -> &'a mut W {
        self.variant(SGCMW::TRANSPARENT_MODE)
    }
    #[doc = "Direct mode, the slave receiver recognizes the general call and executes directly (without software intervention) the related actions. Only the status code word is stored in the SR register for notification to the application"]
    #[inline]
    pub fn direct_mode(self) -> &'a mut W {
        self.variant(SGCMW::DIRECT_MODE)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FTXW<'a> {
    w: &'a mut W,
}
impl<'a> _FTXW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRXW<'a> {
    w: &'a mut W,
}
impl<'a> _FRXW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA_TX_EN`"]
pub enum DMA_TX_ENW {
    #[doc = "DMA TX interface disable"]
    DISABLE,
    #[doc = "DMA TX interface enable"]
    ENABLE,
}
impl DMA_TX_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_TX_ENW::DISABLE => false,
            DMA_TX_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_TX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_TX_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_TX_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA TX interface disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_TX_ENW::DISABLE)
    }
    #[doc = "DMA TX interface enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_TX_ENW::ENABLE)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA_RX_EN`"]
pub enum DMA_RX_ENW {
    #[doc = "DMA RX interface disable"]
    DISABLE,
    #[doc = "DMA RX interface enable"]
    ENABLE,
}
impl DMA_RX_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_RX_ENW::DISABLE => false,
            DMA_RX_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_RX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_RX_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_RX_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA RX interface disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_RX_ENW::DISABLE)
    }
    #[doc = "DMA RX interface enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_RX_ENW::ENABLE)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FON`"]
pub enum FONW {
    #[doc = "No digital filters are inserted"]
    NONE,
    #[doc = "Digital filters (filter 1 ck wide spikes) are inserted"]
    CK1_SPIKES,
    #[doc = "Digital filters (filter 2 ck wide spikes) are inserted"]
    CK2_SPIKES,
    #[doc = "Digital filters (filter 4 ck wide spikes) are inserted"]
    CK4_SPIKES,
}
impl FONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FONW::NONE => 0,
            FONW::CK1_SPIKES => 1,
            FONW::CK2_SPIKES => 2,
            FONW::CK4_SPIKES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FONW<'a> {
    w: &'a mut W,
}
impl<'a> _FONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FONW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No digital filters are inserted"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(FONW::NONE)
    }
    #[doc = "Digital filters (filter 1 ck wide spikes) are inserted"]
    #[inline]
    pub fn ck1_spikes(self) -> &'a mut W {
        self.variant(FONW::CK1_SPIKES)
    }
    #[doc = "Digital filters (filter 2 ck wide spikes) are inserted"]
    #[inline]
    pub fn ck2_spikes(self) -> &'a mut W {
        self.variant(FONW::CK2_SPIKES)
    }
    #[doc = "Digital filters (filter 4 ck wide spikes) are inserted"]
    #[inline]
    pub fn ck4_spikes(self) -> &'a mut W {
        self.variant(FONW::CK4_SPIKES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FS_1`"]
pub enum FS_1W {
    #[doc = "Force stop disable"]
    DISABLE,
    #[doc = "Force stop enable"]
    ENABLE,
}
impl FS_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FS_1W::DISABLE => false,
            FS_1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FS_1W<'a> {
    w: &'a mut W,
}
impl<'a> _FS_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FS_1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Force stop disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FS_1W::DISABLE)
    }
    #[doc = "Force stop enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FS_1W::ENABLE)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - I2C enable disable:<ul><li>0: I2C disable.</li><li>1: I2C enable.</li></ul>This bit when deasserted works as software reset for I2C peripheral."]
    #[inline]
    pub fn pe(&self) -> PER {
        PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - Select the operating mode:<ul><li>00b: Slave mode. The peripheral can only respond (transmit/receive) when addressed by a master device</li><li>01b: Master mode. The peripheral works in a multi-master system where itself cannot be addressed by another master device. It can only initiate a new transfer as master device.</li><li>10b: Master/slave mode. The peripheral works in a multi-master system where itself can be addressed by another master device, besides to initiate a transfer as master device.</li></ul>"]
    #[inline]
    pub fn om(&self) -> OMR {
        OMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Slave addressing mode. SAM defines the slave addressing mode when the peripheral works in slave or master/slave mode. The received address is compared with the content of the register SCR.<ul><li>0: 7-bit addressing mode.</li><li>1: 10-bit addressing mode.</li></ul>"]
    #[inline]
    pub fn sam(&self) -> SAMR {
        SAMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Speed mode. SM defines the speed mode related to the serial bit rate:<ul><li>0: Standard mode (up to 100 K/s).</li><li>1: Fast mode (up to 400 K/s).</li></ul>"]
    #[inline]
    pub fn sm(&self) -> SMR {
        SMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Slave general call mode defines the operating mode of the slave controller when a general call is received. This setting does not affect the hardware general call that is always managed in transparent mode.<ul><li>0: transparent mode, the slave receiver recognizes the general call but any action is taken by the hardware after the decoding of the message included in the Rx FIFO.</li><li>1: direct mode, the slave receiver recognizes the general call and executes directly (without software intervention) the related actions. Only the status code word is stored in the I2C_SR register for notification to the application.</li></ul>"]
    #[inline]
    pub fn sgcm(&self) -> SGCMR {
        SGCMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - FTX flushes the transmit circuitry (FIFO, fsm). The configuration of the I2C node (register setting) is not affected by the flushing operation. The flushing operation is performed on modules working on different clock domains (system and I2C clocks) and needs several system clock cycles before being completed. Upon completion, the I2C node (internal logic) clears this bit. The application must not access the Tx FIFO during the flushing operation and should poll on this bit waiting for completion.<ul><li>0: Flush operation is completed (I2C controller clears the bit).</li><li>1: Flush operation is started and in progress (set by application).</li></ul>"]
    #[inline]
    pub fn ftx(&self) -> FTXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FTXR { bits }
    }
    #[doc = "Bit 8 - FRX flushes the receive circuitry (FIFO, fsm).The configuration of the I2C node (register setting) is not affected by the flushing operation. The flushing operation is performed on modules working on different clock domains (system and I2C clocks) and needs several system clock cycles before to be completed. Upon completion, the I2C node (internal logic) clears this bit. The application must not access the Rx FIFO during the flushing operation and should poll on this bit waiting for the completion.<ul><li>0: Flush operation is completed (I2C controller clears the bit).</li><li>1: Flush operation is started and in progress (set by application).</li></ul>"]
    #[inline]
    pub fn frx(&self) -> FRXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRXR { bits }
    }
    #[doc = "Bit 9 - Enables the DMA TX interface.<ul><li>0: Idle state, the DMA TX interface is disabled.</li><li>1: Run state, the DMA TX interface is enabled.</li></ul>On the completion of the DMA transfer, the DMA TX interface is automatically turned off clearing this bit when the end of transfer signal coming from the DMA is raised. DMA_TX_EN and DMA_RX_EN must not enabled at the same time."]
    #[inline]
    pub fn dma_tx_en(&self) -> DMA_TX_ENR {
        DMA_TX_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enables the DMA RX interface.<ul><li>0: Idle state, the DMA RX interface is disabled.</li><li>1: Run state, the DMA RX interface is enabled.</li></ul>On the completion of the DMA transfer, the DMA RX interface is automatically turned off clearing this bit when the end of transfer signal coming from the DMA is raised. DMA_TX_EN and DMA_RX_EN must not enabled at the same time."]
    #[inline]
    pub fn dma_rx_en(&self) -> DMA_RX_ENR {
        DMA_RX_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 13:14 - Filtering on sets the digital filters on the SDA, SCL line, according to the I2C bus requirements, when standard open-drain pads are used:<ul><li>00b: No digital filters are inserted.</li><li>01b: Digital filters (filter 1 ck wide spikes) are inserted.</li><li>10b: Digital filters (filter 2 ck wide spikes) are inserted.</li><li>11b: Digital filters (filter 4 ck wide spikes) are inserted.</li></ul>"]
    #[inline]
    pub fn fon(&self) -> FONR {
        FONR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Force stop enable bit. When set to 1b, the STOP condition is generated.<ul><li>0: Force stop disabled.</li><li>1: Enable force stop.</li></ul>"]
    #[inline]
    pub fn fs_1(&self) -> FS_1R {
        FS_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - I2C enable disable:<ul><li>0: I2C disable.</li><li>1: I2C enable.</li></ul>This bit when deasserted works as software reset for I2C peripheral."]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bits 1:2 - Select the operating mode:<ul><li>00b: Slave mode. The peripheral can only respond (transmit/receive) when addressed by a master device</li><li>01b: Master mode. The peripheral works in a multi-master system where itself cannot be addressed by another master device. It can only initiate a new transfer as master device.</li><li>10b: Master/slave mode. The peripheral works in a multi-master system where itself can be addressed by another master device, besides to initiate a transfer as master device.</li></ul>"]
    #[inline]
    pub fn om(&mut self) -> _OMW {
        _OMW { w: self }
    }
    #[doc = "Bit 3 - Slave addressing mode. SAM defines the slave addressing mode when the peripheral works in slave or master/slave mode. The received address is compared with the content of the register SCR.<ul><li>0: 7-bit addressing mode.</li><li>1: 10-bit addressing mode.</li></ul>"]
    #[inline]
    pub fn sam(&mut self) -> _SAMW {
        _SAMW { w: self }
    }
    #[doc = "Bits 4:5 - Speed mode. SM defines the speed mode related to the serial bit rate:<ul><li>0: Standard mode (up to 100 K/s).</li><li>1: Fast mode (up to 400 K/s).</li></ul>"]
    #[inline]
    pub fn sm(&mut self) -> _SMW {
        _SMW { w: self }
    }
    #[doc = "Bit 6 - Slave general call mode defines the operating mode of the slave controller when a general call is received. This setting does not affect the hardware general call that is always managed in transparent mode.<ul><li>0: transparent mode, the slave receiver recognizes the general call but any action is taken by the hardware after the decoding of the message included in the Rx FIFO.</li><li>1: direct mode, the slave receiver recognizes the general call and executes directly (without software intervention) the related actions. Only the status code word is stored in the I2C_SR register for notification to the application.</li></ul>"]
    #[inline]
    pub fn sgcm(&mut self) -> _SGCMW {
        _SGCMW { w: self }
    }
    #[doc = "Bit 7 - FTX flushes the transmit circuitry (FIFO, fsm). The configuration of the I2C node (register setting) is not affected by the flushing operation. The flushing operation is performed on modules working on different clock domains (system and I2C clocks) and needs several system clock cycles before being completed. Upon completion, the I2C node (internal logic) clears this bit. The application must not access the Tx FIFO during the flushing operation and should poll on this bit waiting for completion.<ul><li>0: Flush operation is completed (I2C controller clears the bit).</li><li>1: Flush operation is started and in progress (set by application).</li></ul>"]
    #[inline]
    pub fn ftx(&mut self) -> _FTXW {
        _FTXW { w: self }
    }
    #[doc = "Bit 8 - FRX flushes the receive circuitry (FIFO, fsm).The configuration of the I2C node (register setting) is not affected by the flushing operation. The flushing operation is performed on modules working on different clock domains (system and I2C clocks) and needs several system clock cycles before to be completed. Upon completion, the I2C node (internal logic) clears this bit. The application must not access the Rx FIFO during the flushing operation and should poll on this bit waiting for the completion.<ul><li>0: Flush operation is completed (I2C controller clears the bit).</li><li>1: Flush operation is started and in progress (set by application).</li></ul>"]
    #[inline]
    pub fn frx(&mut self) -> _FRXW {
        _FRXW { w: self }
    }
    #[doc = "Bit 9 - Enables the DMA TX interface.<ul><li>0: Idle state, the DMA TX interface is disabled.</li><li>1: Run state, the DMA TX interface is enabled.</li></ul>On the completion of the DMA transfer, the DMA TX interface is automatically turned off clearing this bit when the end of transfer signal coming from the DMA is raised. DMA_TX_EN and DMA_RX_EN must not enabled at the same time."]
    #[inline]
    pub fn dma_tx_en(&mut self) -> _DMA_TX_ENW {
        _DMA_TX_ENW { w: self }
    }
    #[doc = "Bit 10 - Enables the DMA RX interface.<ul><li>0: Idle state, the DMA RX interface is disabled.</li><li>1: Run state, the DMA RX interface is enabled.</li></ul>On the completion of the DMA transfer, the DMA RX interface is automatically turned off clearing this bit when the end of transfer signal coming from the DMA is raised. DMA_TX_EN and DMA_RX_EN must not enabled at the same time."]
    #[inline]
    pub fn dma_rx_en(&mut self) -> _DMA_RX_ENW {
        _DMA_RX_ENW { w: self }
    }
    #[doc = "Bits 13:14 - Filtering on sets the digital filters on the SDA, SCL line, according to the I2C bus requirements, when standard open-drain pads are used:<ul><li>00b: No digital filters are inserted.</li><li>01b: Digital filters (filter 1 ck wide spikes) are inserted.</li><li>10b: Digital filters (filter 2 ck wide spikes) are inserted.</li><li>11b: Digital filters (filter 4 ck wide spikes) are inserted.</li></ul>"]
    #[inline]
    pub fn fon(&mut self) -> _FONW {
        _FONW { w: self }
    }
    #[doc = "Bit 15 - Force stop enable bit. When set to 1b, the STOP condition is generated.<ul><li>0: Force stop disabled.</li><li>1: Enable force stop.</li></ul>"]
    #[inline]
    pub fn fs_1(&mut self) -> _FS_1W {
        _FS_1W { w: self }
    }
}
