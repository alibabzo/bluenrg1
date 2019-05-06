#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONF {
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
#[doc = "Possible values of the field `EN_DFMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_DFMODER {
    #[doc = "Differential mode with DC common mode current not nulled"]
    DFMODE_OFF,
    #[doc = "Differential mode with DC common mode current nulled"]
    DFMODE_ON,
}
impl EN_DFMODER {
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
            EN_DFMODER::DFMODE_OFF => false,
            EN_DFMODER::DFMODE_ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_DFMODER {
        match value {
            false => EN_DFMODER::DFMODE_OFF,
            true => EN_DFMODER::DFMODE_ON,
        }
    }
    #[doc = "Checks if the value of the field is `DFMODE_OFF`"]
    #[inline]
    pub fn is_dfmode_off(&self) -> bool {
        *self == EN_DFMODER::DFMODE_OFF
    }
    #[doc = "Checks if the value of the field is `DFMODE_ON`"]
    #[inline]
    pub fn is_dfmode_on(&self) -> bool {
        *self == EN_DFMODER::DFMODE_ON
    }
}
#[doc = "Possible values of the field `CHSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSELR {
    #[doc = "All switch open"]
    ALL_SWITCH_OPEN,
    #[doc = "Single ended InP=ANATEST2 pin, InN=VREF (internal)"]
    SINGLE_VINP,
    #[doc = "Single ended InN=ANATEST3 pin, InP=VREF (internal)"]
    SINGLE_VINN,
    #[doc = "Differential InP=ANATEST2 pin, InN=ANATEST3 pin"]
    DIFF_INP_INN,
    #[doc = "InP=VTEMPSENS (internal), InN=0.6V (internal)"]
    TEMP_SENS,
    #[doc = "InP=VBATSENS (internal), InN=0.6V (internal)"]
    BATT_SENS,
    #[doc = "InP=InN=0.6V (internal)"]
    SHORT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CHSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHSELR::ALL_SWITCH_OPEN => 0,
            CHSELR::SINGLE_VINP => 1,
            CHSELR::SINGLE_VINN => 2,
            CHSELR::DIFF_INP_INN => 3,
            CHSELR::TEMP_SENS => 4,
            CHSELR::BATT_SENS => 5,
            CHSELR::SHORT => 6,
            CHSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHSELR {
        match value {
            0 => CHSELR::ALL_SWITCH_OPEN,
            1 => CHSELR::SINGLE_VINP,
            2 => CHSELR::SINGLE_VINN,
            3 => CHSELR::DIFF_INP_INN,
            4 => CHSELR::TEMP_SENS,
            5 => CHSELR::BATT_SENS,
            6 => CHSELR::SHORT,
            i => CHSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALL_SWITCH_OPEN`"]
    #[inline]
    pub fn is_all_switch_open(&self) -> bool {
        *self == CHSELR::ALL_SWITCH_OPEN
    }
    #[doc = "Checks if the value of the field is `SINGLE_VINP`"]
    #[inline]
    pub fn is_single_vinp(&self) -> bool {
        *self == CHSELR::SINGLE_VINP
    }
    #[doc = "Checks if the value of the field is `SINGLE_VINN`"]
    #[inline]
    pub fn is_single_vinn(&self) -> bool {
        *self == CHSELR::SINGLE_VINN
    }
    #[doc = "Checks if the value of the field is `DIFF_INP_INN`"]
    #[inline]
    pub fn is_diff_inp_inn(&self) -> bool {
        *self == CHSELR::DIFF_INP_INN
    }
    #[doc = "Checks if the value of the field is `TEMP_SENS`"]
    #[inline]
    pub fn is_temp_sens(&self) -> bool {
        *self == CHSELR::TEMP_SENS
    }
    #[doc = "Checks if the value of the field is `BATT_SENS`"]
    #[inline]
    pub fn is_batt_sens(&self) -> bool {
        *self == CHSELR::BATT_SENS
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline]
    pub fn is_short(&self) -> bool {
        *self == CHSELR::SHORT
    }
}
#[doc = "Possible values of the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELR {
    #[doc = "Set the VREF at 0.0 V"]
    RESEL_0V0,
    #[doc = "Set the VREF at 0.4 V"]
    RESEL_0V4,
    #[doc = "Set the VREF at 0.6 V"]
    RESEL_0V6,
    #[doc = "Set the VREF at 1.2 V"]
    RESEL_1V2,
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFSELR::RESEL_0V0 => 0,
            REFSELR::RESEL_0V4 => 1,
            REFSELR::RESEL_0V6 => 2,
            REFSELR::RESEL_1V2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFSELR {
        match value {
            0 => REFSELR::RESEL_0V0,
            1 => REFSELR::RESEL_0V4,
            2 => REFSELR::RESEL_0V6,
            3 => REFSELR::RESEL_1V2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESEL_0V0`"]
    #[inline]
    pub fn is_resel_0v0(&self) -> bool {
        *self == REFSELR::RESEL_0V0
    }
    #[doc = "Checks if the value of the field is `RESEL_0V4`"]
    #[inline]
    pub fn is_resel_0v4(&self) -> bool {
        *self == REFSELR::RESEL_0V4
    }
    #[doc = "Checks if the value of the field is `RESEL_0V6`"]
    #[inline]
    pub fn is_resel_0v6(&self) -> bool {
        *self == REFSELR::RESEL_0V6
    }
    #[doc = "Checks if the value of the field is `RESEL_1V2`"]
    #[inline]
    pub fn is_resel_1v2(&self) -> bool {
        *self == REFSELR::RESEL_1V2
    }
}
#[doc = "Possible values of the field `DECIM_RATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECIM_RATER {
    #[doc = "Set the decimation factor to 200"]
    DECIM_200,
    #[doc = "Set the decimation factor to 100"]
    DECIM_100,
    #[doc = "Set the decimation factor to 64"]
    DECIM_64,
    #[doc = "Set the decimation factor to 32"]
    DECIM_32,
}
impl DECIM_RATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DECIM_RATER::DECIM_200 => 0,
            DECIM_RATER::DECIM_100 => 1,
            DECIM_RATER::DECIM_64 => 2,
            DECIM_RATER::DECIM_32 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DECIM_RATER {
        match value {
            0 => DECIM_RATER::DECIM_200,
            1 => DECIM_RATER::DECIM_100,
            2 => DECIM_RATER::DECIM_64,
            3 => DECIM_RATER::DECIM_32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DECIM_200`"]
    #[inline]
    pub fn is_decim_200(&self) -> bool {
        *self == DECIM_RATER::DECIM_200
    }
    #[doc = "Checks if the value of the field is `DECIM_100`"]
    #[inline]
    pub fn is_decim_100(&self) -> bool {
        *self == DECIM_RATER::DECIM_100
    }
    #[doc = "Checks if the value of the field is `DECIM_64`"]
    #[inline]
    pub fn is_decim_64(&self) -> bool {
        *self == DECIM_RATER::DECIM_64
    }
    #[doc = "Checks if the value of the field is `DECIM_32`"]
    #[inline]
    pub fn is_decim_32(&self) -> bool {
        *self == DECIM_RATER::DECIM_32
    }
}
#[doc = "Possible values of the field `PGASEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGASELR {
    #[doc = "Input attenuator at 0 dB"]
    IN_ATT_0DB0,
    #[doc = "Input attenuator at 6.02 dB"]
    IN_ATT_6DB02,
    #[doc = "Input attenuator at 9.54 dB"]
    IN_ATT_9DB54,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PGASELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PGASELR::IN_ATT_0DB0 => 0,
            PGASELR::IN_ATT_6DB02 => 1,
            PGASELR::IN_ATT_9DB54 => 2,
            PGASELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PGASELR {
        match value {
            0 => PGASELR::IN_ATT_0DB0,
            1 => PGASELR::IN_ATT_6DB02,
            2 => PGASELR::IN_ATT_9DB54,
            i => PGASELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IN_ATT_0DB0`"]
    #[inline]
    pub fn is_in_att_0d_b0(&self) -> bool {
        *self == PGASELR::IN_ATT_0DB0
    }
    #[doc = "Checks if the value of the field is `IN_ATT_6DB02`"]
    #[inline]
    pub fn is_in_att_6d_b02(&self) -> bool {
        *self == PGASELR::IN_ATT_6DB02
    }
    #[doc = "Checks if the value of the field is `IN_ATT_9DB54`"]
    #[inline]
    pub fn is_in_att_9d_b54(&self) -> bool {
        *self == PGASELR::IN_ATT_9DB54
    }
}
#[doc = "Possible values of the field `CONT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTR {
    #[doc = "Single conversion mode"]
    SINGLE,
    #[doc = "Continuous conversion mode"]
    CONT,
}
impl CONTR {
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
            CONTR::SINGLE => false,
            CONTR::CONT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONTR {
        match value {
            false => CONTR::SINGLE,
            true => CONTR::CONT,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == CONTR::SINGLE
    }
    #[doc = "Checks if the value of the field is `CONT`"]
    #[inline]
    pub fn is_cont(&self) -> bool {
        *self == CONTR::CONT
    }
}
#[doc = "Possible values of the field `ROUND16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROUND16R {
    #[doc = "Output result mapped to 32 bits"]
    MAPPED_32,
    #[doc = "Output result mapped to 16 bits"]
    MAPPED_16,
}
impl ROUND16R {
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
            ROUND16R::MAPPED_32 => false,
            ROUND16R::MAPPED_16 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROUND16R {
        match value {
            false => ROUND16R::MAPPED_32,
            true => ROUND16R::MAPPED_16,
        }
    }
    #[doc = "Checks if the value of the field is `MAPPED_32`"]
    #[inline]
    pub fn is_mapped_32(&self) -> bool {
        *self == ROUND16R::MAPPED_32
    }
    #[doc = "Checks if the value of the field is `MAPPED_16`"]
    #[inline]
    pub fn is_mapped_16(&self) -> bool {
        *self == ROUND16R::MAPPED_16
    }
}
#[doc = "Possible values of the field `SKIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKIPR {
    #[doc = "Filter for comb not bypassed"]
    FILTER_OFF,
    #[doc = "Filter for comb bypassed"]
    FILTER_ON,
}
impl SKIPR {
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
            SKIPR::FILTER_OFF => false,
            SKIPR::FILTER_ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SKIPR {
        match value {
            false => SKIPR::FILTER_OFF,
            true => SKIPR::FILTER_ON,
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_OFF`"]
    #[inline]
    pub fn is_filter_off(&self) -> bool {
        *self == SKIPR::FILTER_OFF
    }
    #[doc = "Checks if the value of the field is `FILTER_ON`"]
    #[inline]
    pub fn is_filter_on(&self) -> bool {
        *self == SKIPR::FILTER_ON
    }
}
#[doc = "Possible values of the field `DIG_FILT_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIG_FILT_CLKR {
    #[doc = "Frequency clock to 0.8 MHz"]
    CLK_0MHZ8,
    #[doc = "Frequency clock to 1.6 MHz"]
    CLK_1MHZ6,
}
impl DIG_FILT_CLKR {
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
            DIG_FILT_CLKR::CLK_0MHZ8 => false,
            DIG_FILT_CLKR::CLK_1MHZ6 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIG_FILT_CLKR {
        match value {
            false => DIG_FILT_CLKR::CLK_0MHZ8,
            true => DIG_FILT_CLKR::CLK_1MHZ6,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_0MHZ8`"]
    #[inline]
    pub fn is_clk_0mhz8(&self) -> bool {
        *self == DIG_FILT_CLKR::CLK_0MHZ8
    }
    #[doc = "Checks if the value of the field is `CLK_1MHZ6`"]
    #[inline]
    pub fn is_clk_1mhz6(&self) -> bool {
        *self == DIG_FILT_CLKR::CLK_1MHZ6
    }
}
#[doc = "Possible values of the field `DIS_WKP_WAIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_WKP_WAITR {
    #[doc = "Do not disable the wake up time before conversion"]
    ENABLE,
    #[doc = "Disable the wake up time before conversion"]
    DISABLE,
}
impl DIS_WKP_WAITR {
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
            DIS_WKP_WAITR::ENABLE => false,
            DIS_WKP_WAITR::DISABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIS_WKP_WAITR {
        match value {
            false => DIS_WKP_WAITR::ENABLE,
            true => DIS_WKP_WAITR::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DIS_WKP_WAITR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DIS_WKP_WAITR::DISABLE
    }
}
#[doc = "Possible values of the field `MIC_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIC_SELR {
    #[doc = "Do not provided any external clock source"]
    DISABLE,
    #[doc = "Provide clock source from GPIO"]
    ENABLE,
}
impl MIC_SELR {
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
            MIC_SELR::DISABLE => false,
            MIC_SELR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIC_SELR {
        match value {
            false => MIC_SELR::DISABLE,
            true => MIC_SELR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MIC_SELR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MIC_SELR::ENABLE
    }
}
#[doc = "Values that can be written to the field `EN_DFMODE`"]
pub enum EN_DFMODEW {
    #[doc = "Differential mode with DC common mode current not nulled"]
    DFMODE_OFF,
    #[doc = "Differential mode with DC common mode current nulled"]
    DFMODE_ON,
}
impl EN_DFMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_DFMODEW::DFMODE_OFF => false,
            EN_DFMODEW::DFMODE_ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_DFMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_DFMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_DFMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Differential mode with DC common mode current not nulled"]
    #[inline]
    pub fn dfmode_off(self) -> &'a mut W {
        self.variant(EN_DFMODEW::DFMODE_OFF)
    }
    #[doc = "Differential mode with DC common mode current nulled"]
    #[inline]
    pub fn dfmode_on(self) -> &'a mut W {
        self.variant(EN_DFMODEW::DFMODE_ON)
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
#[doc = "Values that can be written to the field `CHSEL`"]
pub enum CHSELW {
    #[doc = "All switch open"]
    ALL_SWITCH_OPEN,
    #[doc = "Single ended InP=ANATEST2 pin, InN=VREF (internal)"]
    SINGLE_VINP,
    #[doc = "Single ended InN=ANATEST3 pin, InP=VREF (internal)"]
    SINGLE_VINN,
    #[doc = "Differential InP=ANATEST2 pin, InN=ANATEST3 pin"]
    DIFF_INP_INN,
    #[doc = "InP=VTEMPSENS (internal), InN=0.6V (internal)"]
    TEMP_SENS,
    #[doc = "InP=VBATSENS (internal), InN=0.6V (internal)"]
    BATT_SENS,
    #[doc = "InP=InN=0.6V (internal)"]
    SHORT,
}
impl CHSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHSELW::ALL_SWITCH_OPEN => 0,
            CHSELW::SINGLE_VINP => 1,
            CHSELW::SINGLE_VINN => 2,
            CHSELW::DIFF_INP_INN => 3,
            CHSELW::TEMP_SENS => 4,
            CHSELW::BATT_SENS => 5,
            CHSELW::SHORT => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CHSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "All switch open"]
    #[inline]
    pub fn all_switch_open(self) -> &'a mut W {
        self.variant(CHSELW::ALL_SWITCH_OPEN)
    }
    #[doc = "Single ended InP=ANATEST2 pin, InN=VREF (internal)"]
    #[inline]
    pub fn single_vinp(self) -> &'a mut W {
        self.variant(CHSELW::SINGLE_VINP)
    }
    #[doc = "Single ended InN=ANATEST3 pin, InP=VREF (internal)"]
    #[inline]
    pub fn single_vinn(self) -> &'a mut W {
        self.variant(CHSELW::SINGLE_VINN)
    }
    #[doc = "Differential InP=ANATEST2 pin, InN=ANATEST3 pin"]
    #[inline]
    pub fn diff_inp_inn(self) -> &'a mut W {
        self.variant(CHSELW::DIFF_INP_INN)
    }
    #[doc = "InP=VTEMPSENS (internal), InN=0.6V (internal)"]
    #[inline]
    pub fn temp_sens(self) -> &'a mut W {
        self.variant(CHSELW::TEMP_SENS)
    }
    #[doc = "InP=VBATSENS (internal), InN=0.6V (internal)"]
    #[inline]
    pub fn batt_sens(self) -> &'a mut W {
        self.variant(CHSELW::BATT_SENS)
    }
    #[doc = "InP=InN=0.6V (internal)"]
    #[inline]
    pub fn short(self) -> &'a mut W {
        self.variant(CHSELW::SHORT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "Set the VREF at 0.0 V"]
    RESEL_0V0,
    #[doc = "Set the VREF at 0.4 V"]
    RESEL_0V4,
    #[doc = "Set the VREF at 0.6 V"]
    RESEL_0V6,
    #[doc = "Set the VREF at 1.2 V"]
    RESEL_1V2,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFSELW::RESEL_0V0 => 0,
            REFSELW::RESEL_0V4 => 1,
            REFSELW::RESEL_0V6 => 2,
            REFSELW::RESEL_1V2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _REFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Set the VREF at 0.0 V"]
    #[inline]
    pub fn resel_0v0(self) -> &'a mut W {
        self.variant(REFSELW::RESEL_0V0)
    }
    #[doc = "Set the VREF at 0.4 V"]
    #[inline]
    pub fn resel_0v4(self) -> &'a mut W {
        self.variant(REFSELW::RESEL_0V4)
    }
    #[doc = "Set the VREF at 0.6 V"]
    #[inline]
    pub fn resel_0v6(self) -> &'a mut W {
        self.variant(REFSELW::RESEL_0V6)
    }
    #[doc = "Set the VREF at 1.2 V"]
    #[inline]
    pub fn resel_1v2(self) -> &'a mut W {
        self.variant(REFSELW::RESEL_1V2)
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
#[doc = "Values that can be written to the field `DECIM_RATE`"]
pub enum DECIM_RATEW {
    #[doc = "Set the decimation factor to 200"]
    DECIM_200,
    #[doc = "Set the decimation factor to 100"]
    DECIM_100,
    #[doc = "Set the decimation factor to 64"]
    DECIM_64,
    #[doc = "Set the decimation factor to 32"]
    DECIM_32,
}
impl DECIM_RATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DECIM_RATEW::DECIM_200 => 0,
            DECIM_RATEW::DECIM_100 => 1,
            DECIM_RATEW::DECIM_64 => 2,
            DECIM_RATEW::DECIM_32 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DECIM_RATEW<'a> {
    w: &'a mut W,
}
impl<'a> _DECIM_RATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DECIM_RATEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Set the decimation factor to 200"]
    #[inline]
    pub fn decim_200(self) -> &'a mut W {
        self.variant(DECIM_RATEW::DECIM_200)
    }
    #[doc = "Set the decimation factor to 100"]
    #[inline]
    pub fn decim_100(self) -> &'a mut W {
        self.variant(DECIM_RATEW::DECIM_100)
    }
    #[doc = "Set the decimation factor to 64"]
    #[inline]
    pub fn decim_64(self) -> &'a mut W {
        self.variant(DECIM_RATEW::DECIM_64)
    }
    #[doc = "Set the decimation factor to 32"]
    #[inline]
    pub fn decim_32(self) -> &'a mut W {
        self.variant(DECIM_RATEW::DECIM_32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PGASEL`"]
pub enum PGASELW {
    #[doc = "Input attenuator at 0 dB"]
    IN_ATT_0DB0,
    #[doc = "Input attenuator at 6.02 dB"]
    IN_ATT_6DB02,
    #[doc = "Input attenuator at 9.54 dB"]
    IN_ATT_9DB54,
}
impl PGASELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PGASELW::IN_ATT_0DB0 => 0,
            PGASELW::IN_ATT_6DB02 => 1,
            PGASELW::IN_ATT_9DB54 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PGASELW<'a> {
    w: &'a mut W,
}
impl<'a> _PGASELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PGASELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input attenuator at 0 dB"]
    #[inline]
    pub fn in_att_0d_b0(self) -> &'a mut W {
        self.variant(PGASELW::IN_ATT_0DB0)
    }
    #[doc = "Input attenuator at 6.02 dB"]
    #[inline]
    pub fn in_att_6d_b02(self) -> &'a mut W {
        self.variant(PGASELW::IN_ATT_6DB02)
    }
    #[doc = "Input attenuator at 9.54 dB"]
    #[inline]
    pub fn in_att_9d_b54(self) -> &'a mut W {
        self.variant(PGASELW::IN_ATT_9DB54)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CONT`"]
pub enum CONTW {
    #[doc = "Single conversion mode"]
    SINGLE,
    #[doc = "Continuous conversion mode"]
    CONT,
}
impl CONTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONTW::SINGLE => false,
            CONTW::CONT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONTW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single conversion mode"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(CONTW::SINGLE)
    }
    #[doc = "Continuous conversion mode"]
    #[inline]
    pub fn cont(self) -> &'a mut W {
        self.variant(CONTW::CONT)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ROUND16`"]
pub enum ROUND16W {
    #[doc = "Output result mapped to 32 bits"]
    MAPPED_32,
    #[doc = "Output result mapped to 16 bits"]
    MAPPED_16,
}
impl ROUND16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROUND16W::MAPPED_32 => false,
            ROUND16W::MAPPED_16 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROUND16W<'a> {
    w: &'a mut W,
}
impl<'a> _ROUND16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROUND16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output result mapped to 32 bits"]
    #[inline]
    pub fn mapped_32(self) -> &'a mut W {
        self.variant(ROUND16W::MAPPED_32)
    }
    #[doc = "Output result mapped to 16 bits"]
    #[inline]
    pub fn mapped_16(self) -> &'a mut W {
        self.variant(ROUND16W::MAPPED_16)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SKIP`"]
pub enum SKIPW {
    #[doc = "Filter for comb not bypassed"]
    FILTER_OFF,
    #[doc = "Filter for comb bypassed"]
    FILTER_ON,
}
impl SKIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SKIPW::FILTER_OFF => false,
            SKIPW::FILTER_ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SKIPW<'a> {
    w: &'a mut W,
}
impl<'a> _SKIPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SKIPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Filter for comb not bypassed"]
    #[inline]
    pub fn filter_off(self) -> &'a mut W {
        self.variant(SKIPW::FILTER_OFF)
    }
    #[doc = "Filter for comb bypassed"]
    #[inline]
    pub fn filter_on(self) -> &'a mut W {
        self.variant(SKIPW::FILTER_ON)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIG_FILT_CLK`"]
pub enum DIG_FILT_CLKW {
    #[doc = "Frequency clock to 0.8 MHz"]
    CLK_0MHZ8,
    #[doc = "Frequency clock to 1.6 MHz"]
    CLK_1MHZ6,
}
impl DIG_FILT_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIG_FILT_CLKW::CLK_0MHZ8 => false,
            DIG_FILT_CLKW::CLK_1MHZ6 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIG_FILT_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _DIG_FILT_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIG_FILT_CLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frequency clock to 0.8 MHz"]
    #[inline]
    pub fn clk_0mhz8(self) -> &'a mut W {
        self.variant(DIG_FILT_CLKW::CLK_0MHZ8)
    }
    #[doc = "Frequency clock to 1.6 MHz"]
    #[inline]
    pub fn clk_1mhz6(self) -> &'a mut W {
        self.variant(DIG_FILT_CLKW::CLK_1MHZ6)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIS_WKP_WAIT`"]
pub enum DIS_WKP_WAITW {
    #[doc = "Do not disable the wake up time before conversion"]
    ENABLE,
    #[doc = "Disable the wake up time before conversion"]
    DISABLE,
}
impl DIS_WKP_WAITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIS_WKP_WAITW::ENABLE => false,
            DIS_WKP_WAITW::DISABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIS_WKP_WAITW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_WKP_WAITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIS_WKP_WAITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not disable the wake up time before conversion"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DIS_WKP_WAITW::ENABLE)
    }
    #[doc = "Disable the wake up time before conversion"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DIS_WKP_WAITW::DISABLE)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MIC_SEL`"]
pub enum MIC_SELW {
    #[doc = "Do not provided any external clock source"]
    DISABLE,
    #[doc = "Provide clock source from GPIO"]
    ENABLE,
}
impl MIC_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MIC_SELW::DISABLE => false,
            MIC_SELW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MIC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _MIC_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MIC_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not provided any external clock source"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MIC_SELW::DISABLE)
    }
    #[doc = "Provide clock source from GPIO"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MIC_SELW::ENABLE)
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
        const OFFSET: u8 = 22;
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
    #[doc = "Bit 0 - Control the current in differential mode:<ul><li>0: Differential mode with DC common mode current not nulled.</li><li>1: Differential mode with DC common mode current nulled.</li></ul>"]
    #[inline]
    pub fn en_dfmode(&self) -> EN_DFMODER {
        EN_DFMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - Select the input channel:<ul><li>000b: All switches open.</li><li>001b: Single ended through ADC2 pin. InP=VREF (internal), InN=ADC2 pin.</li><li>010b: Single ended through ADC1 pin. InP=ADC1 pin, InN=VREF (internal).</li><li>011b: Differential ADC1 pin - ADC2 pin, InP=ADC1 pin, InN=ADC2 pin.</li><li>101b: Battery level detector. InP=0.6V (internal), InN=BLD.</li><li>110b: Short InN=InP=0.6V (internal).</li></ul>"]
    #[inline]
    pub fn chsel(&self) -> CHSELR {
        CHSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Set the VREF for single ended conversion:<ul><li>00b: 0.0V.</li><li>01b: 0.4V.</li><li>10b: 0.6V.</li><li>11b: 1.2V.</li></ul>"]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Set the ADC resolution:<ul><li>00b: Set the decimation factor to 200.</li><li>01b: Set the decimation factor to 100.</li><li>10b: Set the decimation factor to 64.</li><li>11b: Set the decimation factor to 32.</li></ul>"]
    #[inline]
    pub fn decim_rate(&self) -> DECIM_RATER {
        DECIM_RATER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Set the input attenuator value:<ul><li>000b: Input attenuator at 0 dB.</li><li>001b: Input attenuator at 6.02 dB.</li><li>010b: Input attenuator at 9.54 dB.</li></ul>"]
    #[inline]
    pub fn pgasel(&self) -> PGASELR {
        PGASELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Enable the continuous conversion mode:<ul><li>0: Single conversion.</li><li>1: Continuous conversion.</li></ul>"]
    #[inline]
    pub fn cont(&self) -> CONTR {
        CONTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Result mapped on 32 or 16 bits:<ul><li>0: Output result mapped to 32 bits.</li><li>1: Output result mapped to 16 bits.</li></ul>"]
    #[inline]
    pub fn round16(&self) -> ROUND16R {
        ROUND16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - It permits to bypass the filter comb to speed up the conversion for signal at low frequency:<ul><li>0: Filter for comb not bypassed.</li><li>1: Filter for comb bypassed.</li></ul>"]
    #[inline]
    pub fn skip(&self) -> SKIPR {
        SKIPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Frequency clock selection value on GPIO0 when MIC_SEL=1:<ul><li>0: 0.8 MHz.</li><li>1: 1.6 MHz.</li></ul>"]
    #[inline]
    pub fn dig_filt_clk(&self) -> DIG_FILT_CLKR {
        DIG_FILT_CLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Disable the wake-up timer before to start the conversion from input:<ul><li>0: Do not disable the wake up time before conversion.</li><li>1: Disable the wake up time before conversion.</li></ul>"]
    #[inline]
    pub fn dis_wkp_wait(&self) -> DIS_WKP_WAITR {
        DIS_WKP_WAITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Provides the clock on GPIO:<ul><li>0: Do not provided any external clock source.</li><li>1: Provide clock source from GPIO.</li></ul>"]
    #[inline]
    pub fn mic_sel(&self) -> MIC_SELR {
        MIC_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Control the current in differential mode:<ul><li>0: Differential mode with DC common mode current not nulled.</li><li>1: Differential mode with DC common mode current nulled.</li></ul>"]
    #[inline]
    pub fn en_dfmode(&mut self) -> _EN_DFMODEW {
        _EN_DFMODEW { w: self }
    }
    #[doc = "Bits 1:3 - Select the input channel:<ul><li>000b: All switches open.</li><li>001b: Single ended through ADC2 pin. InP=VREF (internal), InN=ADC2 pin.</li><li>010b: Single ended through ADC1 pin. InP=ADC1 pin, InN=VREF (internal).</li><li>011b: Differential ADC1 pin - ADC2 pin, InP=ADC1 pin, InN=ADC2 pin.</li><li>101b: Battery level detector. InP=0.6V (internal), InN=BLD.</li><li>110b: Short InN=InP=0.6V (internal).</li></ul>"]
    #[inline]
    pub fn chsel(&mut self) -> _CHSELW {
        _CHSELW { w: self }
    }
    #[doc = "Bits 4:5 - Set the VREF for single ended conversion:<ul><li>00b: 0.0V.</li><li>01b: 0.4V.</li><li>10b: 0.6V.</li><li>11b: 1.2V.</li></ul>"]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
    #[doc = "Bits 6:7 - Set the ADC resolution:<ul><li>00b: Set the decimation factor to 200.</li><li>01b: Set the decimation factor to 100.</li><li>10b: Set the decimation factor to 64.</li><li>11b: Set the decimation factor to 32.</li></ul>"]
    #[inline]
    pub fn decim_rate(&mut self) -> _DECIM_RATEW {
        _DECIM_RATEW { w: self }
    }
    #[doc = "Bits 8:9 - Set the input attenuator value:<ul><li>000b: Input attenuator at 0 dB.</li><li>001b: Input attenuator at 6.02 dB.</li><li>010b: Input attenuator at 9.54 dB.</li></ul>"]
    #[inline]
    pub fn pgasel(&mut self) -> _PGASELW {
        _PGASELW { w: self }
    }
    #[doc = "Bit 11 - Enable the continuous conversion mode:<ul><li>0: Single conversion.</li><li>1: Continuous conversion.</li></ul>"]
    #[inline]
    pub fn cont(&mut self) -> _CONTW {
        _CONTW { w: self }
    }
    #[doc = "Bit 17 - Result mapped on 32 or 16 bits:<ul><li>0: Output result mapped to 32 bits.</li><li>1: Output result mapped to 16 bits.</li></ul>"]
    #[inline]
    pub fn round16(&mut self) -> _ROUND16W {
        _ROUND16W { w: self }
    }
    #[doc = "Bit 18 - It permits to bypass the filter comb to speed up the conversion for signal at low frequency:<ul><li>0: Filter for comb not bypassed.</li><li>1: Filter for comb bypassed.</li></ul>"]
    #[inline]
    pub fn skip(&mut self) -> _SKIPW {
        _SKIPW { w: self }
    }
    #[doc = "Bit 20 - Frequency clock selection value on GPIO0 when MIC_SEL=1:<ul><li>0: 0.8 MHz.</li><li>1: 1.6 MHz.</li></ul>"]
    #[inline]
    pub fn dig_filt_clk(&mut self) -> _DIG_FILT_CLKW {
        _DIG_FILT_CLKW { w: self }
    }
    #[doc = "Bit 21 - Disable the wake-up timer before to start the conversion from input:<ul><li>0: Do not disable the wake up time before conversion.</li><li>1: Disable the wake up time before conversion.</li></ul>"]
    #[inline]
    pub fn dis_wkp_wait(&mut self) -> _DIS_WKP_WAITW {
        _DIS_WKP_WAITW { w: self }
    }
    #[doc = "Bit 22 - Provides the clock on GPIO:<ul><li>0: Do not provided any external clock source.</li><li>1: Provide clock source from GPIO.</li></ul>"]
    #[inline]
    pub fn mic_sel(&mut self) -> _MIC_SELW {
        _MIC_SELW { w: self }
    }
}
