#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::XFCR {
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
#[doc = "Possible values of the field `SFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFENR {
    #[doc = "software flow ctrl enable"]
    SOFTWARE_FLOW_CTRL_ENABLE,
    #[doc = "software flow ctrl disable"]
    SOFTWARE_FLOW_CTRL_DISABLE,
}
impl SFENR {
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
            SFENR::SOFTWARE_FLOW_CTRL_ENABLE => true,
            SFENR::SOFTWARE_FLOW_CTRL_DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SFENR {
        match value {
            true => SFENR::SOFTWARE_FLOW_CTRL_ENABLE,
            false => SFENR::SOFTWARE_FLOW_CTRL_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE_FLOW_CTRL_ENABLE`"]
    #[inline]
    pub fn is_software_flow_ctrl_enable(&self) -> bool {
        *self == SFENR::SOFTWARE_FLOW_CTRL_ENABLE
    }
    #[doc = "Checks if the value of the field is `SOFTWARE_FLOW_CTRL_DISABLE`"]
    #[inline]
    pub fn is_software_flow_ctrl_disable(&self) -> bool {
        *self == SFENR::SOFTWARE_FLOW_CTRL_DISABLE
    }
}
#[doc = "Possible values of the field `SFRMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFRMODR {
    #[doc = "Receive flow control is disable"]
    SFR_MODE_DISABLE,
    #[doc = "Xon1, Xoff1 characters are used in receive software flow control"]
    SFR_MODE_XON1_XOFF1,
    #[doc = "Xon2, Xoff2 characters are used in receive software flow control"]
    SFR_MODE_XON2_XOFF2,
    #[doc = "Xon1 and Xon2, Xoff1 and Xoff2 characters are used in receive software flow control"]
    SFR_MODE_XON1_XON2_XOFF1_XOFF2,
}
impl SFRMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SFRMODR::SFR_MODE_DISABLE => 0,
            SFRMODR::SFR_MODE_XON1_XOFF1 => 1,
            SFRMODR::SFR_MODE_XON2_XOFF2 => 2,
            SFRMODR::SFR_MODE_XON1_XON2_XOFF1_XOFF2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SFRMODR {
        match value {
            0 => SFRMODR::SFR_MODE_DISABLE,
            1 => SFRMODR::SFR_MODE_XON1_XOFF1,
            2 => SFRMODR::SFR_MODE_XON2_XOFF2,
            3 => SFRMODR::SFR_MODE_XON1_XON2_XOFF1_XOFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SFR_MODE_DISABLE`"]
    #[inline]
    pub fn is_sfr_mode_disable(&self) -> bool {
        *self == SFRMODR::SFR_MODE_DISABLE
    }
    #[doc = "Checks if the value of the field is `SFR_MODE_XON1_XOFF1`"]
    #[inline]
    pub fn is_sfr_mode_xon1_xoff1(&self) -> bool {
        *self == SFRMODR::SFR_MODE_XON1_XOFF1
    }
    #[doc = "Checks if the value of the field is `SFR_MODE_XON2_XOFF2`"]
    #[inline]
    pub fn is_sfr_mode_xon2_xoff2(&self) -> bool {
        *self == SFRMODR::SFR_MODE_XON2_XOFF2
    }
    #[doc = "Checks if the value of the field is `SFR_MODE_XON1_XON2_XOFF1_XOFF2`"]
    #[inline]
    pub fn is_sfr_mode_xon1_xon2_xoff1_xoff2(&self) -> bool {
        *self == SFRMODR::SFR_MODE_XON1_XON2_XOFF1_XOFF2
    }
}
#[doc = "Possible values of the field `SFTMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFTMODR {
    #[doc = "Transmit flow control is disable"]
    SFR_MODE_DISABLE,
    #[doc = "Xon1, Xoff1 characters are used in transmit software flow control"]
    SFR_MODE_XON1_XOFF1,
    #[doc = "Xon2, Xoff2 characters are used in transmit software flow control"]
    SFR_MODE_XON2_XOFF2,
    #[doc = "Xon1 and Xon2, Xoff1 and Xoff2 characters are used in transmit software flow control"]
    SFR_MODE_XON1_XON2_XOFF1_XOFF2,
}
impl SFTMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SFTMODR::SFR_MODE_DISABLE => 0,
            SFTMODR::SFR_MODE_XON1_XOFF1 => 1,
            SFTMODR::SFR_MODE_XON2_XOFF2 => 2,
            SFTMODR::SFR_MODE_XON1_XON2_XOFF1_XOFF2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SFTMODR {
        match value {
            0 => SFTMODR::SFR_MODE_DISABLE,
            1 => SFTMODR::SFR_MODE_XON1_XOFF1,
            2 => SFTMODR::SFR_MODE_XON2_XOFF2,
            3 => SFTMODR::SFR_MODE_XON1_XON2_XOFF1_XOFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SFR_MODE_DISABLE`"]
    #[inline]
    pub fn is_sfr_mode_disable(&self) -> bool {
        *self == SFTMODR::SFR_MODE_DISABLE
    }
    #[doc = "Checks if the value of the field is `SFR_MODE_XON1_XOFF1`"]
    #[inline]
    pub fn is_sfr_mode_xon1_xoff1(&self) -> bool {
        *self == SFTMODR::SFR_MODE_XON1_XOFF1
    }
    #[doc = "Checks if the value of the field is `SFR_MODE_XON2_XOFF2`"]
    #[inline]
    pub fn is_sfr_mode_xon2_xoff2(&self) -> bool {
        *self == SFTMODR::SFR_MODE_XON2_XOFF2
    }
    #[doc = "Checks if the value of the field is `SFR_MODE_XON1_XON2_XOFF1_XOFF2`"]
    #[inline]
    pub fn is_sfr_mode_xon1_xon2_xoff1_xoff2(&self) -> bool {
        *self == SFTMODR::SFR_MODE_XON1_XON2_XOFF1_XOFF2
    }
}
#[doc = "Possible values of the field `XONANY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XONANYR {
    #[doc = "any incoming character is considered as a valid Xon"]
    XONANY_ENABLE,
    #[doc = "incoming character must match Xon programmed value(s) to be a valid Xon"]
    XONANY_DISABLE,
}
impl XONANYR {
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
            XONANYR::XONANY_ENABLE => true,
            XONANYR::XONANY_DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XONANYR {
        match value {
            true => XONANYR::XONANY_ENABLE,
            false => XONANYR::XONANY_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `XONANY_ENABLE`"]
    #[inline]
    pub fn is_xonany_enable(&self) -> bool {
        *self == XONANYR::XONANY_ENABLE
    }
    #[doc = "Checks if the value of the field is `XONANY_DISABLE`"]
    #[inline]
    pub fn is_xonany_disable(&self) -> bool {
        *self == XONANYR::XONANY_DISABLE
    }
}
#[doc = "Possible values of the field `SPECHAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPECHARR {
    #[doc = "special character detection enabled"]
    SPECHAR_ENABLE,
    #[doc = "pecial character detection disabled"]
    SPECHAR_DISABLE,
}
impl SPECHARR {
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
            SPECHARR::SPECHAR_ENABLE => true,
            SPECHARR::SPECHAR_DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPECHARR {
        match value {
            true => SPECHARR::SPECHAR_ENABLE,
            false => SPECHARR::SPECHAR_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `SPECHAR_ENABLE`"]
    #[inline]
    pub fn is_spechar_enable(&self) -> bool {
        *self == SPECHARR::SPECHAR_ENABLE
    }
    #[doc = "Checks if the value of the field is `SPECHAR_DISABLE`"]
    #[inline]
    pub fn is_spechar_disable(&self) -> bool {
        *self == SPECHARR::SPECHAR_DISABLE
    }
}
#[doc = "Values that can be written to the field `SFEN`"]
pub enum SFENW {
    #[doc = "software flow ctrl enable"]
    SOFTWARE_FLOW_CTRL_ENABLE,
    #[doc = "software flow ctrl disable"]
    SOFTWARE_FLOW_CTRL_DISABLE,
}
impl SFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SFENW::SOFTWARE_FLOW_CTRL_ENABLE => true,
            SFENW::SOFTWARE_FLOW_CTRL_DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SFENW<'a> {
    w: &'a mut W,
}
impl<'a> _SFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "software flow ctrl enable"]
    #[inline]
    pub fn software_flow_ctrl_enable(self) -> &'a mut W {
        self.variant(SFENW::SOFTWARE_FLOW_CTRL_ENABLE)
    }
    #[doc = "software flow ctrl disable"]
    #[inline]
    pub fn software_flow_ctrl_disable(self) -> &'a mut W {
        self.variant(SFENW::SOFTWARE_FLOW_CTRL_DISABLE)
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
#[doc = "Values that can be written to the field `SFRMOD`"]
pub enum SFRMODW {
    #[doc = "Receive flow control is disable"]
    SFR_MODE_DISABLE,
    #[doc = "Xon1, Xoff1 characters are used in receive software flow control"]
    SFR_MODE_XON1_XOFF1,
    #[doc = "Xon2, Xoff2 characters are used in receive software flow control"]
    SFR_MODE_XON2_XOFF2,
    #[doc = "Xon1 and Xon2, Xoff1 and Xoff2 characters are used in receive software flow control"]
    SFR_MODE_XON1_XON2_XOFF1_XOFF2,
}
impl SFRMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SFRMODW::SFR_MODE_DISABLE => 0,
            SFRMODW::SFR_MODE_XON1_XOFF1 => 1,
            SFRMODW::SFR_MODE_XON2_XOFF2 => 2,
            SFRMODW::SFR_MODE_XON1_XON2_XOFF1_XOFF2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SFRMODW<'a> {
    w: &'a mut W,
}
impl<'a> _SFRMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SFRMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Receive flow control is disable"]
    #[inline]
    pub fn sfr_mode_disable(self) -> &'a mut W {
        self.variant(SFRMODW::SFR_MODE_DISABLE)
    }
    #[doc = "Xon1, Xoff1 characters are used in receive software flow control"]
    #[inline]
    pub fn sfr_mode_xon1_xoff1(self) -> &'a mut W {
        self.variant(SFRMODW::SFR_MODE_XON1_XOFF1)
    }
    #[doc = "Xon2, Xoff2 characters are used in receive software flow control"]
    #[inline]
    pub fn sfr_mode_xon2_xoff2(self) -> &'a mut W {
        self.variant(SFRMODW::SFR_MODE_XON2_XOFF2)
    }
    #[doc = "Xon1 and Xon2, Xoff1 and Xoff2 characters are used in receive software flow control"]
    #[inline]
    pub fn sfr_mode_xon1_xon2_xoff1_xoff2(self) -> &'a mut W {
        self.variant(SFRMODW::SFR_MODE_XON1_XON2_XOFF1_XOFF2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SFTMOD`"]
pub enum SFTMODW {
    #[doc = "Transmit flow control is disable"]
    SFR_MODE_DISABLE,
    #[doc = "Xon1, Xoff1 characters are used in transmit software flow control"]
    SFR_MODE_XON1_XOFF1,
    #[doc = "Xon2, Xoff2 characters are used in transmit software flow control"]
    SFR_MODE_XON2_XOFF2,
    #[doc = "Xon1 and Xon2, Xoff1 and Xoff2 characters are used in transmit software flow control"]
    SFR_MODE_XON1_XON2_XOFF1_XOFF2,
}
impl SFTMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SFTMODW::SFR_MODE_DISABLE => 0,
            SFTMODW::SFR_MODE_XON1_XOFF1 => 1,
            SFTMODW::SFR_MODE_XON2_XOFF2 => 2,
            SFTMODW::SFR_MODE_XON1_XON2_XOFF1_XOFF2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SFTMODW<'a> {
    w: &'a mut W,
}
impl<'a> _SFTMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SFTMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Transmit flow control is disable"]
    #[inline]
    pub fn sfr_mode_disable(self) -> &'a mut W {
        self.variant(SFTMODW::SFR_MODE_DISABLE)
    }
    #[doc = "Xon1, Xoff1 characters are used in transmit software flow control"]
    #[inline]
    pub fn sfr_mode_xon1_xoff1(self) -> &'a mut W {
        self.variant(SFTMODW::SFR_MODE_XON1_XOFF1)
    }
    #[doc = "Xon2, Xoff2 characters are used in transmit software flow control"]
    #[inline]
    pub fn sfr_mode_xon2_xoff2(self) -> &'a mut W {
        self.variant(SFTMODW::SFR_MODE_XON2_XOFF2)
    }
    #[doc = "Xon1 and Xon2, Xoff1 and Xoff2 characters are used in transmit software flow control"]
    #[inline]
    pub fn sfr_mode_xon1_xon2_xoff1_xoff2(self) -> &'a mut W {
        self.variant(SFTMODW::SFR_MODE_XON1_XON2_XOFF1_XOFF2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `XONANY`"]
pub enum XONANYW {
    #[doc = "any incoming character is considered as a valid Xon"]
    XONANY_ENABLE,
    #[doc = "incoming character must match Xon programmed value(s) to be a valid Xon"]
    XONANY_DISABLE,
}
impl XONANYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XONANYW::XONANY_ENABLE => true,
            XONANYW::XONANY_DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XONANYW<'a> {
    w: &'a mut W,
}
impl<'a> _XONANYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XONANYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "any incoming character is considered as a valid Xon"]
    #[inline]
    pub fn xonany_enable(self) -> &'a mut W {
        self.variant(XONANYW::XONANY_ENABLE)
    }
    #[doc = "incoming character must match Xon programmed value(s) to be a valid Xon"]
    #[inline]
    pub fn xonany_disable(self) -> &'a mut W {
        self.variant(XONANYW::XONANY_DISABLE)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPECHAR`"]
pub enum SPECHARW {
    #[doc = "special character detection enabled"]
    SPECHAR_ENABLE,
    #[doc = "pecial character detection disabled"]
    SPECHAR_DISABLE,
}
impl SPECHARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPECHARW::SPECHAR_ENABLE => true,
            SPECHARW::SPECHAR_DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPECHARW<'a> {
    w: &'a mut W,
}
impl<'a> _SPECHARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPECHARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "special character detection enabled"]
    #[inline]
    pub fn spechar_enable(self) -> &'a mut W {
        self.variant(SPECHARW::SPECHAR_ENABLE)
    }
    #[doc = "pecial character detection disabled"]
    #[inline]
    pub fn spechar_disable(self) -> &'a mut W {
        self.variant(SPECHARW::SPECHAR_DISABLE)
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
    #[doc = "Bit 0 - Software flow control enable.<ul><li>0: Software flow control disable.</li><li>1: software flow control enable.</li></ul>"]
    #[inline]
    pub fn sfen(&self) -> SFENR {
        SFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 1:2 - Software receive flow control mode:<ul><li>00b: Receive flow control is disabled.</li><li>01b: Xon1, Xoff1 characters are used in receiving software flow control.</li><li>10b: Xon2, Xoff2 characters are used in receiving software flow control.</li><li>11b: Xon1 and Xon2, Xoff1 and Xoff2 characters are used in receiving software flow control.</li></ul>"]
    #[inline]
    pub fn sfrmod(&self) -> SFRMODR {
        SFRMODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 3:4 - Software transmit flow control mode:<ul><li>00b: Transmit flow control is disabled.</li><li>01b: Xon1, Xoff1 characters are used in transmitting software flow control.</li><li>10b: Xon2, Xoff2 characters are used in transmitting software flow control.</li><li>11b: Xon1 and Xon2, Xoff1 and Xoff2 characters are used in transmitting software flow control.</li></ul>"]
    #[inline]
    pub fn sftmod(&self) -> SFTMODR {
        SFTMODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 5 - Xon-any bit:<ul><li>0: Incoming character must match Xon programmed value(s) to be a valid Xon.</li><li>1: Any incoming character is considered as a valid Xon.</li></ul>"]
    #[inline]
    pub fn xonany(&self) -> XONANYR {
        XONANYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Special character detection bit. <ul><li>0: Special character detection disabled.</li><li>1: Special character detection enabled.</li></ul>"]
    #[inline]
    pub fn spechar(&self) -> SPECHARR {
        SPECHARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Software flow control enable.<ul><li>0: Software flow control disable.</li><li>1: software flow control enable.</li></ul>"]
    #[inline]
    pub fn sfen(&mut self) -> _SFENW {
        _SFENW { w: self }
    }
    #[doc = "Bits 1:2 - Software receive flow control mode:<ul><li>00b: Receive flow control is disabled.</li><li>01b: Xon1, Xoff1 characters are used in receiving software flow control.</li><li>10b: Xon2, Xoff2 characters are used in receiving software flow control.</li><li>11b: Xon1 and Xon2, Xoff1 and Xoff2 characters are used in receiving software flow control.</li></ul>"]
    #[inline]
    pub fn sfrmod(&mut self) -> _SFRMODW {
        _SFRMODW { w: self }
    }
    #[doc = "Bits 3:4 - Software transmit flow control mode:<ul><li>00b: Transmit flow control is disabled.</li><li>01b: Xon1, Xoff1 characters are used in transmitting software flow control.</li><li>10b: Xon2, Xoff2 characters are used in transmitting software flow control.</li><li>11b: Xon1 and Xon2, Xoff1 and Xoff2 characters are used in transmitting software flow control.</li></ul>"]
    #[inline]
    pub fn sftmod(&mut self) -> _SFTMODW {
        _SFTMODW { w: self }
    }
    #[doc = "Bit 5 - Xon-any bit:<ul><li>0: Incoming character must match Xon programmed value(s) to be a valid Xon.</li><li>1: Any incoming character is considered as a valid Xon.</li></ul>"]
    #[inline]
    pub fn xonany(&mut self) -> _XONANYW {
        _XONANYW { w: self }
    }
    #[doc = "Bit 6 - Special character detection bit. <ul><li>0: Special character detection disabled.</li><li>1: Special character detection enabled.</li></ul>"]
    #[inline]
    pub fn spechar(&mut self) -> _SPECHARW {
        _SPECHARW { w: self }
    }
}
