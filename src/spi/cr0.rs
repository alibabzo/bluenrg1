#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR0 {
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
#[doc = "Possible values of the field `DSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSSR {
    #[doc = "4-bit data"]
    DATA_4BIT,
    #[doc = "5-bit data"]
    DATA_5BIT,
    #[doc = "6-bit data"]
    DATA_6BIT,
    #[doc = "7-bit data"]
    DATA_7BIT,
    #[doc = "8-bit data"]
    DATA_8BIT,
    #[doc = "9-bit data"]
    DATA_9BIT,
    #[doc = "10-bit data"]
    DATA_10BIT,
    #[doc = "11-bit data"]
    DATA_11BIT,
    #[doc = "12-bit data"]
    DATA_12BIT,
    #[doc = "13-bit data"]
    DATA_13BIT,
    #[doc = "14-bit data"]
    DATA_14BIT,
    #[doc = "15-bit data"]
    DATA_15BIT,
    #[doc = "16-bit data"]
    DATA_16BIT,
    #[doc = "17-bit data"]
    DATA_17BIT,
    #[doc = "18-bit data"]
    DATA_18BIT,
    #[doc = "19-bit data"]
    DATA_19BIT,
    #[doc = "20-bit data"]
    DATA_20BIT,
    #[doc = "21-bit data"]
    DATA_21BIT,
    #[doc = "22-bit data"]
    DATA_22BIT,
    #[doc = "23-bit data"]
    DATA_23BIT,
    #[doc = "24-bit data"]
    DATA_24BIT,
    #[doc = "25-bit data"]
    DATA_25BIT,
    #[doc = "26-bit data"]
    DATA_26BIT,
    #[doc = "27-bit data"]
    DATA_27BIT,
    #[doc = "28-bit data"]
    DATA_28BIT,
    #[doc = "29-bit data"]
    DATA_29BIT,
    #[doc = "30-bit data"]
    DATA_30BIT,
    #[doc = "31-bit data"]
    DATA_31BIT,
    #[doc = "32-bit data"]
    DATA_32BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DSSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSSR::DATA_4BIT => 3,
            DSSR::DATA_5BIT => 4,
            DSSR::DATA_6BIT => 5,
            DSSR::DATA_7BIT => 6,
            DSSR::DATA_8BIT => 7,
            DSSR::DATA_9BIT => 8,
            DSSR::DATA_10BIT => 9,
            DSSR::DATA_11BIT => 10,
            DSSR::DATA_12BIT => 11,
            DSSR::DATA_13BIT => 12,
            DSSR::DATA_14BIT => 13,
            DSSR::DATA_15BIT => 14,
            DSSR::DATA_16BIT => 15,
            DSSR::DATA_17BIT => 16,
            DSSR::DATA_18BIT => 17,
            DSSR::DATA_19BIT => 18,
            DSSR::DATA_20BIT => 19,
            DSSR::DATA_21BIT => 20,
            DSSR::DATA_22BIT => 21,
            DSSR::DATA_23BIT => 22,
            DSSR::DATA_24BIT => 23,
            DSSR::DATA_25BIT => 24,
            DSSR::DATA_26BIT => 25,
            DSSR::DATA_27BIT => 26,
            DSSR::DATA_28BIT => 27,
            DSSR::DATA_29BIT => 28,
            DSSR::DATA_30BIT => 29,
            DSSR::DATA_31BIT => 30,
            DSSR::DATA_32BIT => 31,
            DSSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSSR {
        match value {
            3 => DSSR::DATA_4BIT,
            4 => DSSR::DATA_5BIT,
            5 => DSSR::DATA_6BIT,
            6 => DSSR::DATA_7BIT,
            7 => DSSR::DATA_8BIT,
            8 => DSSR::DATA_9BIT,
            9 => DSSR::DATA_10BIT,
            10 => DSSR::DATA_11BIT,
            11 => DSSR::DATA_12BIT,
            12 => DSSR::DATA_13BIT,
            13 => DSSR::DATA_14BIT,
            14 => DSSR::DATA_15BIT,
            15 => DSSR::DATA_16BIT,
            16 => DSSR::DATA_17BIT,
            17 => DSSR::DATA_18BIT,
            18 => DSSR::DATA_19BIT,
            19 => DSSR::DATA_20BIT,
            20 => DSSR::DATA_21BIT,
            21 => DSSR::DATA_22BIT,
            22 => DSSR::DATA_23BIT,
            23 => DSSR::DATA_24BIT,
            24 => DSSR::DATA_25BIT,
            25 => DSSR::DATA_26BIT,
            26 => DSSR::DATA_27BIT,
            27 => DSSR::DATA_28BIT,
            28 => DSSR::DATA_29BIT,
            29 => DSSR::DATA_30BIT,
            30 => DSSR::DATA_31BIT,
            31 => DSSR::DATA_32BIT,
            i => DSSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DATA_4BIT`"]
    #[inline]
    pub fn is_data_4bit(&self) -> bool {
        *self == DSSR::DATA_4BIT
    }
    #[doc = "Checks if the value of the field is `DATA_5BIT`"]
    #[inline]
    pub fn is_data_5bit(&self) -> bool {
        *self == DSSR::DATA_5BIT
    }
    #[doc = "Checks if the value of the field is `DATA_6BIT`"]
    #[inline]
    pub fn is_data_6bit(&self) -> bool {
        *self == DSSR::DATA_6BIT
    }
    #[doc = "Checks if the value of the field is `DATA_7BIT`"]
    #[inline]
    pub fn is_data_7bit(&self) -> bool {
        *self == DSSR::DATA_7BIT
    }
    #[doc = "Checks if the value of the field is `DATA_8BIT`"]
    #[inline]
    pub fn is_data_8bit(&self) -> bool {
        *self == DSSR::DATA_8BIT
    }
    #[doc = "Checks if the value of the field is `DATA_9BIT`"]
    #[inline]
    pub fn is_data_9bit(&self) -> bool {
        *self == DSSR::DATA_9BIT
    }
    #[doc = "Checks if the value of the field is `DATA_10BIT`"]
    #[inline]
    pub fn is_data_10bit(&self) -> bool {
        *self == DSSR::DATA_10BIT
    }
    #[doc = "Checks if the value of the field is `DATA_11BIT`"]
    #[inline]
    pub fn is_data_11bit(&self) -> bool {
        *self == DSSR::DATA_11BIT
    }
    #[doc = "Checks if the value of the field is `DATA_12BIT`"]
    #[inline]
    pub fn is_data_12bit(&self) -> bool {
        *self == DSSR::DATA_12BIT
    }
    #[doc = "Checks if the value of the field is `DATA_13BIT`"]
    #[inline]
    pub fn is_data_13bit(&self) -> bool {
        *self == DSSR::DATA_13BIT
    }
    #[doc = "Checks if the value of the field is `DATA_14BIT`"]
    #[inline]
    pub fn is_data_14bit(&self) -> bool {
        *self == DSSR::DATA_14BIT
    }
    #[doc = "Checks if the value of the field is `DATA_15BIT`"]
    #[inline]
    pub fn is_data_15bit(&self) -> bool {
        *self == DSSR::DATA_15BIT
    }
    #[doc = "Checks if the value of the field is `DATA_16BIT`"]
    #[inline]
    pub fn is_data_16bit(&self) -> bool {
        *self == DSSR::DATA_16BIT
    }
    #[doc = "Checks if the value of the field is `DATA_17BIT`"]
    #[inline]
    pub fn is_data_17bit(&self) -> bool {
        *self == DSSR::DATA_17BIT
    }
    #[doc = "Checks if the value of the field is `DATA_18BIT`"]
    #[inline]
    pub fn is_data_18bit(&self) -> bool {
        *self == DSSR::DATA_18BIT
    }
    #[doc = "Checks if the value of the field is `DATA_19BIT`"]
    #[inline]
    pub fn is_data_19bit(&self) -> bool {
        *self == DSSR::DATA_19BIT
    }
    #[doc = "Checks if the value of the field is `DATA_20BIT`"]
    #[inline]
    pub fn is_data_20bit(&self) -> bool {
        *self == DSSR::DATA_20BIT
    }
    #[doc = "Checks if the value of the field is `DATA_21BIT`"]
    #[inline]
    pub fn is_data_21bit(&self) -> bool {
        *self == DSSR::DATA_21BIT
    }
    #[doc = "Checks if the value of the field is `DATA_22BIT`"]
    #[inline]
    pub fn is_data_22bit(&self) -> bool {
        *self == DSSR::DATA_22BIT
    }
    #[doc = "Checks if the value of the field is `DATA_23BIT`"]
    #[inline]
    pub fn is_data_23bit(&self) -> bool {
        *self == DSSR::DATA_23BIT
    }
    #[doc = "Checks if the value of the field is `DATA_24BIT`"]
    #[inline]
    pub fn is_data_24bit(&self) -> bool {
        *self == DSSR::DATA_24BIT
    }
    #[doc = "Checks if the value of the field is `DATA_25BIT`"]
    #[inline]
    pub fn is_data_25bit(&self) -> bool {
        *self == DSSR::DATA_25BIT
    }
    #[doc = "Checks if the value of the field is `DATA_26BIT`"]
    #[inline]
    pub fn is_data_26bit(&self) -> bool {
        *self == DSSR::DATA_26BIT
    }
    #[doc = "Checks if the value of the field is `DATA_27BIT`"]
    #[inline]
    pub fn is_data_27bit(&self) -> bool {
        *self == DSSR::DATA_27BIT
    }
    #[doc = "Checks if the value of the field is `DATA_28BIT`"]
    #[inline]
    pub fn is_data_28bit(&self) -> bool {
        *self == DSSR::DATA_28BIT
    }
    #[doc = "Checks if the value of the field is `DATA_29BIT`"]
    #[inline]
    pub fn is_data_29bit(&self) -> bool {
        *self == DSSR::DATA_29BIT
    }
    #[doc = "Checks if the value of the field is `DATA_30BIT`"]
    #[inline]
    pub fn is_data_30bit(&self) -> bool {
        *self == DSSR::DATA_30BIT
    }
    #[doc = "Checks if the value of the field is `DATA_31BIT`"]
    #[inline]
    pub fn is_data_31bit(&self) -> bool {
        *self == DSSR::DATA_31BIT
    }
    #[doc = "Checks if the value of the field is `DATA_32BIT`"]
    #[inline]
    pub fn is_data_32bit(&self) -> bool {
        *self == DSSR::DATA_32BIT
    }
}
#[doc = "Possible values of the field `SPO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOR {
    #[doc = "The inactive or idle state of SSPCLKO is LOW"]
    INACTIVE_LOW,
    #[doc = "The inactive or idle state of SSPCLKO is HIGH"]
    INACTIVE_HIGH,
}
impl SPOR {
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
            SPOR::INACTIVE_LOW => false,
            SPOR::INACTIVE_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPOR {
        match value {
            false => SPOR::INACTIVE_LOW,
            true => SPOR::INACTIVE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE_LOW`"]
    #[inline]
    pub fn is_inactive_low(&self) -> bool {
        *self == SPOR::INACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `INACTIVE_HIGH`"]
    #[inline]
    pub fn is_inactive_high(&self) -> bool {
        *self == SPOR::INACTIVE_HIGH
    }
}
#[doc = "Possible values of the field `SPH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPHR {
    #[doc = "Received data is captured on the rising edge (SPO=0) or on the falling edge (SPO=1) of SSPCLKO. Transmitted data is sent on the falling edge (SPO=0) or on the rising edge (SPO=1) of SSPCLKO"]
    PHASE_0,
    #[doc = "Received data is captured on the falling edge (SPO=0) or on the rising edge (SPO=1) of SSPCLKO.Transmitted data is sent on the rising edge (SPO=0) or on the falling edge (SPO=1) of SSPCLKO"]
    PHASE_1,
}
impl SPHR {
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
            SPHR::PHASE_0 => false,
            SPHR::PHASE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPHR {
        match value {
            false => SPHR::PHASE_0,
            true => SPHR::PHASE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PHASE_0`"]
    #[inline]
    pub fn is_phase_0(&self) -> bool {
        *self == SPHR::PHASE_0
    }
    #[doc = "Checks if the value of the field is `PHASE_1`"]
    #[inline]
    pub fn is_phase_1(&self) -> bool {
        *self == SPHR::PHASE_1
    }
}
#[doc = r" Value of the field"]
pub struct SCRR {
    bits: u8,
}
impl SCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SPIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMR {
    #[doc = "SPI is configured in full duplex mode"]
    FULL_DUPLEX,
    #[doc = "SPI is configured in transmit mode"]
    TRANSMIT,
    #[doc = "SPI is configured in receive mode"]
    RECEIVE,
    #[doc = "SPI is configured in combined mode"]
    COMBINED,
}
impl SPIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPIMR::FULL_DUPLEX => 0,
            SPIMR::TRANSMIT => 1,
            SPIMR::RECEIVE => 2,
            SPIMR::COMBINED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPIMR {
        match value {
            0 => SPIMR::FULL_DUPLEX,
            1 => SPIMR::TRANSMIT,
            2 => SPIMR::RECEIVE,
            3 => SPIMR::COMBINED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FULL_DUPLEX`"]
    #[inline]
    pub fn is_full_duplex(&self) -> bool {
        *self == SPIMR::FULL_DUPLEX
    }
    #[doc = "Checks if the value of the field is `TRANSMIT`"]
    #[inline]
    pub fn is_transmit(&self) -> bool {
        *self == SPIMR::TRANSMIT
    }
    #[doc = "Checks if the value of the field is `RECEIVE`"]
    #[inline]
    pub fn is_receive(&self) -> bool {
        *self == SPIMR::RECEIVE
    }
    #[doc = "Checks if the value of the field is `COMBINED`"]
    #[inline]
    pub fn is_combined(&self) -> bool {
        *self == SPIMR::COMBINED
    }
}
#[doc = r" Value of the field"]
pub struct INVCLKR {
    bits: bool,
}
impl INVCLKR {
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
#[doc = "Possible values of the field `CS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS1R {
    #[doc = "Slave 1 is select"]
    CS1_NOT_SELECT,
    #[doc = "Slave 1 is not select"]
    CS1_SELECT,
}
impl CS1R {
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
            CS1R::CS1_NOT_SELECT => false,
            CS1R::CS1_SELECT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CS1R {
        match value {
            false => CS1R::CS1_NOT_SELECT,
            true => CS1R::CS1_SELECT,
        }
    }
    #[doc = "Checks if the value of the field is `CS1_NOT_SELECT`"]
    #[inline]
    pub fn is_cs1_not_select(&self) -> bool {
        *self == CS1R::CS1_NOT_SELECT
    }
    #[doc = "Checks if the value of the field is `CS1_SELECT`"]
    #[inline]
    pub fn is_cs1_select(&self) -> bool {
        *self == CS1R::CS1_SELECT
    }
}
#[doc = "Values that can be written to the field `DSS`"]
pub enum DSSW {
    #[doc = "4-bit data"]
    DATA_4BIT,
    #[doc = "5-bit data"]
    DATA_5BIT,
    #[doc = "6-bit data"]
    DATA_6BIT,
    #[doc = "7-bit data"]
    DATA_7BIT,
    #[doc = "8-bit data"]
    DATA_8BIT,
    #[doc = "9-bit data"]
    DATA_9BIT,
    #[doc = "10-bit data"]
    DATA_10BIT,
    #[doc = "11-bit data"]
    DATA_11BIT,
    #[doc = "12-bit data"]
    DATA_12BIT,
    #[doc = "13-bit data"]
    DATA_13BIT,
    #[doc = "14-bit data"]
    DATA_14BIT,
    #[doc = "15-bit data"]
    DATA_15BIT,
    #[doc = "16-bit data"]
    DATA_16BIT,
    #[doc = "17-bit data"]
    DATA_17BIT,
    #[doc = "18-bit data"]
    DATA_18BIT,
    #[doc = "19-bit data"]
    DATA_19BIT,
    #[doc = "20-bit data"]
    DATA_20BIT,
    #[doc = "21-bit data"]
    DATA_21BIT,
    #[doc = "22-bit data"]
    DATA_22BIT,
    #[doc = "23-bit data"]
    DATA_23BIT,
    #[doc = "24-bit data"]
    DATA_24BIT,
    #[doc = "25-bit data"]
    DATA_25BIT,
    #[doc = "26-bit data"]
    DATA_26BIT,
    #[doc = "27-bit data"]
    DATA_27BIT,
    #[doc = "28-bit data"]
    DATA_28BIT,
    #[doc = "29-bit data"]
    DATA_29BIT,
    #[doc = "30-bit data"]
    DATA_30BIT,
    #[doc = "31-bit data"]
    DATA_31BIT,
    #[doc = "32-bit data"]
    DATA_32BIT,
}
impl DSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSSW::DATA_4BIT => 3,
            DSSW::DATA_5BIT => 4,
            DSSW::DATA_6BIT => 5,
            DSSW::DATA_7BIT => 6,
            DSSW::DATA_8BIT => 7,
            DSSW::DATA_9BIT => 8,
            DSSW::DATA_10BIT => 9,
            DSSW::DATA_11BIT => 10,
            DSSW::DATA_12BIT => 11,
            DSSW::DATA_13BIT => 12,
            DSSW::DATA_14BIT => 13,
            DSSW::DATA_15BIT => 14,
            DSSW::DATA_16BIT => 15,
            DSSW::DATA_17BIT => 16,
            DSSW::DATA_18BIT => 17,
            DSSW::DATA_19BIT => 18,
            DSSW::DATA_20BIT => 19,
            DSSW::DATA_21BIT => 20,
            DSSW::DATA_22BIT => 21,
            DSSW::DATA_23BIT => 22,
            DSSW::DATA_24BIT => 23,
            DSSW::DATA_25BIT => 24,
            DSSW::DATA_26BIT => 25,
            DSSW::DATA_27BIT => 26,
            DSSW::DATA_28BIT => 27,
            DSSW::DATA_29BIT => 28,
            DSSW::DATA_30BIT => 29,
            DSSW::DATA_31BIT => 30,
            DSSW::DATA_32BIT => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSSW<'a> {
    w: &'a mut W,
}
impl<'a> _DSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "4-bit data"]
    #[inline]
    pub fn data_4bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_4BIT)
    }
    #[doc = "5-bit data"]
    #[inline]
    pub fn data_5bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_5BIT)
    }
    #[doc = "6-bit data"]
    #[inline]
    pub fn data_6bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_6BIT)
    }
    #[doc = "7-bit data"]
    #[inline]
    pub fn data_7bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_7BIT)
    }
    #[doc = "8-bit data"]
    #[inline]
    pub fn data_8bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_8BIT)
    }
    #[doc = "9-bit data"]
    #[inline]
    pub fn data_9bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_9BIT)
    }
    #[doc = "10-bit data"]
    #[inline]
    pub fn data_10bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_10BIT)
    }
    #[doc = "11-bit data"]
    #[inline]
    pub fn data_11bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_11BIT)
    }
    #[doc = "12-bit data"]
    #[inline]
    pub fn data_12bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_12BIT)
    }
    #[doc = "13-bit data"]
    #[inline]
    pub fn data_13bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_13BIT)
    }
    #[doc = "14-bit data"]
    #[inline]
    pub fn data_14bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_14BIT)
    }
    #[doc = "15-bit data"]
    #[inline]
    pub fn data_15bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_15BIT)
    }
    #[doc = "16-bit data"]
    #[inline]
    pub fn data_16bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_16BIT)
    }
    #[doc = "17-bit data"]
    #[inline]
    pub fn data_17bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_17BIT)
    }
    #[doc = "18-bit data"]
    #[inline]
    pub fn data_18bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_18BIT)
    }
    #[doc = "19-bit data"]
    #[inline]
    pub fn data_19bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_19BIT)
    }
    #[doc = "20-bit data"]
    #[inline]
    pub fn data_20bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_20BIT)
    }
    #[doc = "21-bit data"]
    #[inline]
    pub fn data_21bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_21BIT)
    }
    #[doc = "22-bit data"]
    #[inline]
    pub fn data_22bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_22BIT)
    }
    #[doc = "23-bit data"]
    #[inline]
    pub fn data_23bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_23BIT)
    }
    #[doc = "24-bit data"]
    #[inline]
    pub fn data_24bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_24BIT)
    }
    #[doc = "25-bit data"]
    #[inline]
    pub fn data_25bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_25BIT)
    }
    #[doc = "26-bit data"]
    #[inline]
    pub fn data_26bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_26BIT)
    }
    #[doc = "27-bit data"]
    #[inline]
    pub fn data_27bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_27BIT)
    }
    #[doc = "28-bit data"]
    #[inline]
    pub fn data_28bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_28BIT)
    }
    #[doc = "29-bit data"]
    #[inline]
    pub fn data_29bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_29BIT)
    }
    #[doc = "30-bit data"]
    #[inline]
    pub fn data_30bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_30BIT)
    }
    #[doc = "31-bit data"]
    #[inline]
    pub fn data_31bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_31BIT)
    }
    #[doc = "32-bit data"]
    #[inline]
    pub fn data_32bit(self) -> &'a mut W {
        self.variant(DSSW::DATA_32BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPO`"]
pub enum SPOW {
    #[doc = "The inactive or idle state of SSPCLKO is LOW"]
    INACTIVE_LOW,
    #[doc = "The inactive or idle state of SSPCLKO is HIGH"]
    INACTIVE_HIGH,
}
impl SPOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPOW::INACTIVE_LOW => false,
            SPOW::INACTIVE_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPOW<'a> {
    w: &'a mut W,
}
impl<'a> _SPOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The inactive or idle state of SSPCLKO is LOW"]
    #[inline]
    pub fn inactive_low(self) -> &'a mut W {
        self.variant(SPOW::INACTIVE_LOW)
    }
    #[doc = "The inactive or idle state of SSPCLKO is HIGH"]
    #[inline]
    pub fn inactive_high(self) -> &'a mut W {
        self.variant(SPOW::INACTIVE_HIGH)
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
#[doc = "Values that can be written to the field `SPH`"]
pub enum SPHW {
    #[doc = "Received data is captured on the rising edge (SPO=0) or on the falling edge (SPO=1) of SSPCLKO. Transmitted data is sent on the falling edge (SPO=0) or on the rising edge (SPO=1) of SSPCLKO"]
    PHASE_0,
    #[doc = "Received data is captured on the falling edge (SPO=0) or on the rising edge (SPO=1) of SSPCLKO.Transmitted data is sent on the rising edge (SPO=0) or on the falling edge (SPO=1) of SSPCLKO"]
    PHASE_1,
}
impl SPHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPHW::PHASE_0 => false,
            SPHW::PHASE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPHW<'a> {
    w: &'a mut W,
}
impl<'a> _SPHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Received data is captured on the rising edge (SPO=0) or on the falling edge (SPO=1) of SSPCLKO. Transmitted data is sent on the falling edge (SPO=0) or on the rising edge (SPO=1) of SSPCLKO"]
    #[inline]
    pub fn phase_0(self) -> &'a mut W {
        self.variant(SPHW::PHASE_0)
    }
    #[doc = "Received data is captured on the falling edge (SPO=0) or on the rising edge (SPO=1) of SSPCLKO.Transmitted data is sent on the rising edge (SPO=0) or on the falling edge (SPO=1) of SSPCLKO"]
    #[inline]
    pub fn phase_1(self) -> &'a mut W {
        self.variant(SPHW::PHASE_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCRW<'a> {
    w: &'a mut W,
}
impl<'a> _SCRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPIM`"]
pub enum SPIMW {
    #[doc = "SPI is configured in full duplex mode"]
    FULL_DUPLEX,
    #[doc = "SPI is configured in transmit mode"]
    TRANSMIT,
    #[doc = "SPI is configured in receive mode"]
    RECEIVE,
    #[doc = "SPI is configured in combined mode"]
    COMBINED,
}
impl SPIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPIMW::FULL_DUPLEX => 0,
            SPIMW::TRANSMIT => 1,
            SPIMW::RECEIVE => 2,
            SPIMW::COMBINED => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPIMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SPI is configured in full duplex mode"]
    #[inline]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(SPIMW::FULL_DUPLEX)
    }
    #[doc = "SPI is configured in transmit mode"]
    #[inline]
    pub fn transmit(self) -> &'a mut W {
        self.variant(SPIMW::TRANSMIT)
    }
    #[doc = "SPI is configured in receive mode"]
    #[inline]
    pub fn receive(self) -> &'a mut W {
        self.variant(SPIMW::RECEIVE)
    }
    #[doc = "SPI is configured in combined mode"]
    #[inline]
    pub fn combined(self) -> &'a mut W {
        self.variant(SPIMW::COMBINED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INVCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _INVCLKW<'a> {
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
#[doc = "Values that can be written to the field `CS1`"]
pub enum CS1W {
    #[doc = "Slave 1 is select"]
    CS1_NOT_SELECT,
    #[doc = "Slave 1 is not select"]
    CS1_SELECT,
}
impl CS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CS1W::CS1_NOT_SELECT => false,
            CS1W::CS1_SELECT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CS1W<'a> {
    w: &'a mut W,
}
impl<'a> _CS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave 1 is select"]
    #[inline]
    pub fn cs1_not_select(self) -> &'a mut W {
        self.variant(CS1W::CS1_NOT_SELECT)
    }
    #[doc = "Slave 1 is not select"]
    #[inline]
    pub fn cs1_select(self) -> &'a mut W {
        self.variant(CS1W::CS1_SELECT)
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
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:4 - Data size select. (DSS+1) defines the number of bits:<ul><li>0x00: Reserved.</li><li>0x01: Reserved.</li><li>0x02: Reserved.</li><li>0x03: 4-bit data.</li><li>0x04: 5-bit data.</li><li>...</li><li>0x1F: 32-bit data.</li></ul>"]
    #[inline]
    pub fn dss(&self) -> DSSR {
        DSSR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Clock polarity.<ul><li>0: Steady state of clock polarity is low.</li><li>1: Steady state of clock polarity is high.</li></ul>"]
    #[inline]
    pub fn spo(&self) -> SPOR {
        SPOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Clock phase.<ul><li>0: Steady state of clock phase is low.</li><li>1: Steady state of clock phase is high.</li></ul>"]
    #[inline]
    pub fn sph(&self) -> SPHR {
        SPHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:15 - Serial Clock Rate.<p>The SRC value is used to generate the transmit and receive bit rate of the SPI. The bit rate is: f_SPICLK / (CPSDVR * (1 + SCR)), where CPSDVR is an even value from 2 to 254 and SCR is a value from 0 to 255.</p>"]
    #[inline]
    pub fn scr(&self) -> SCRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCRR { bits }
    }
    #[doc = "Bits 23:24 - SPI transmission mode.<ul><li>00b: Full duplex mode.</li><li>01b: Transmit mode.</li><li>10b: Receive mode.</li><li>11b: Combined mode.</li></ul>"]
    #[inline]
    pub fn spim(&self) -> SPIMR {
        SPIMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 25 - Activate inversion (in master mode only).<ul><li>0: Master samples the received data respecting the Motorola SPI protocol.</li><li>1: The sampling of the received data by master is delayed by half an SPI clock cycle..</li></ul>"]
    #[inline]
    pub fn invclk(&self) -> INVCLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INVCLKR { bits }
    }
    #[doc = "Bit 26 - Chip Selection for slave one<ul><li>0: the slave 1 is selected.</li><li>1: the slave 1 is not selected.</li></ul>"]
    #[inline]
    pub fn cs1(&self) -> CS1R {
        CS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 469762048 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Data size select. (DSS+1) defines the number of bits:<ul><li>0x00: Reserved.</li><li>0x01: Reserved.</li><li>0x02: Reserved.</li><li>0x03: 4-bit data.</li><li>0x04: 5-bit data.</li><li>...</li><li>0x1F: 32-bit data.</li></ul>"]
    #[inline]
    pub fn dss(&mut self) -> _DSSW {
        _DSSW { w: self }
    }
    #[doc = "Bit 6 - Clock polarity.<ul><li>0: Steady state of clock polarity is low.</li><li>1: Steady state of clock polarity is high.</li></ul>"]
    #[inline]
    pub fn spo(&mut self) -> _SPOW {
        _SPOW { w: self }
    }
    #[doc = "Bit 7 - Clock phase.<ul><li>0: Steady state of clock phase is low.</li><li>1: Steady state of clock phase is high.</li></ul>"]
    #[inline]
    pub fn sph(&mut self) -> _SPHW {
        _SPHW { w: self }
    }
    #[doc = "Bits 8:15 - Serial Clock Rate.<p>The SRC value is used to generate the transmit and receive bit rate of the SPI. The bit rate is: f_SPICLK / (CPSDVR * (1 + SCR)), where CPSDVR is an even value from 2 to 254 and SCR is a value from 0 to 255.</p>"]
    #[inline]
    pub fn scr(&mut self) -> _SCRW {
        _SCRW { w: self }
    }
    #[doc = "Bits 23:24 - SPI transmission mode.<ul><li>00b: Full duplex mode.</li><li>01b: Transmit mode.</li><li>10b: Receive mode.</li><li>11b: Combined mode.</li></ul>"]
    #[inline]
    pub fn spim(&mut self) -> _SPIMW {
        _SPIMW { w: self }
    }
    #[doc = "Bit 25 - Activate inversion (in master mode only).<ul><li>0: Master samples the received data respecting the Motorola SPI protocol.</li><li>1: The sampling of the received data by master is delayed by half an SPI clock cycle..</li></ul>"]
    #[inline]
    pub fn invclk(&mut self) -> _INVCLKW {
        _INVCLKW { w: self }
    }
    #[doc = "Bit 26 - Chip Selection for slave one<ul><li>0: the slave 1 is selected.</li><li>1: the slave 1 is not selected.</li></ul>"]
    #[inline]
    pub fn cs1(&mut self) -> _CS1W {
        _CS1W { w: self }
    }
}
