#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HSTIMER0_OVF_INT_RAW` reader - The interrupt raw bit for high speed channel0 counter overflow."]
pub struct HSTIMER0_OVF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl HSTIMER0_OVF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSTIMER0_OVF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTIMER0_OVF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTIMER1_OVF_INT_RAW` reader - The interrupt raw bit for high speed channel1 counter overflow."]
pub struct HSTIMER1_OVF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl HSTIMER1_OVF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSTIMER1_OVF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTIMER1_OVF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTIMER2_OVF_INT_RAW` reader - The interrupt raw bit for high speed channel2 counter overflow."]
pub struct HSTIMER2_OVF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl HSTIMER2_OVF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSTIMER2_OVF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTIMER2_OVF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTIMER3_OVF_INT_RAW` reader - The interrupt raw bit for high speed channel3 counter overflow."]
pub struct HSTIMER3_OVF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl HSTIMER3_OVF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSTIMER3_OVF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTIMER3_OVF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER0_OVF_INT_RAW` reader - The interrupt raw bit for low speed channel0 counter overflow."]
pub struct LSTIMER0_OVF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl LSTIMER0_OVF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER0_OVF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER0_OVF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER1_OVF_INT_RAW` reader - The interrupt raw bit for low speed channel1 counter overflow."]
pub struct LSTIMER1_OVF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl LSTIMER1_OVF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER1_OVF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER1_OVF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER2_OVF_INT_RAW` reader - The interrupt raw bit for low speed channel2 counter overflow."]
pub struct LSTIMER2_OVF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl LSTIMER2_OVF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER2_OVF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER2_OVF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER3_OVF_INT_RAW` reader - The interrupt raw bit for low speed channel3 counter overflow."]
pub struct LSTIMER3_OVF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl LSTIMER3_OVF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER3_OVF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER3_OVF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH0_INT_RAW` reader - The interrupt raw bit for high speed channel 0 duty change done."]
pub struct DUTY_CHNG_END_HSCH0_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH0_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH0_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH0_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH1_INT_RAW` reader - The interrupt raw bit for high speed channel 1 duty change done."]
pub struct DUTY_CHNG_END_HSCH1_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH1_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH1_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH1_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH2_INT_RAW` reader - The interrupt raw bit for high speed channel 2 duty change done."]
pub struct DUTY_CHNG_END_HSCH2_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH2_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH2_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH2_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH3_INT_RAW` reader - The interrupt raw bit for high speed channel 3 duty change done."]
pub struct DUTY_CHNG_END_HSCH3_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH3_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH3_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH3_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH4_INT_RAW` reader - The interrupt raw bit for high speed channel 4 duty change done."]
pub struct DUTY_CHNG_END_HSCH4_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH4_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH4_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH4_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH5_INT_RAW` reader - The interrupt raw bit for high speed channel 5 duty change done."]
pub struct DUTY_CHNG_END_HSCH5_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH5_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH5_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH5_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH6_INT_RAW` reader - The interrupt raw bit for high speed channel 6 duty change done."]
pub struct DUTY_CHNG_END_HSCH6_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH6_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH6_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH6_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH7_INT_RAW` reader - The interrupt raw bit for high speed channel 7 duty change done."]
pub struct DUTY_CHNG_END_HSCH7_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH7_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH7_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH7_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_RAW` reader - The interrupt raw bit for low speed channel 0 duty change done."]
pub struct DUTY_CHNG_END_LSCH0_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH0_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH0_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH0_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_RAW` reader - The interrupt raw bit for low speed channel 1 duty change done."]
pub struct DUTY_CHNG_END_LSCH1_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH1_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH1_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH1_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_RAW` reader - The interrupt raw bit for low speed channel 2 duty change done."]
pub struct DUTY_CHNG_END_LSCH2_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH2_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH2_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH2_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_RAW` reader - The interrupt raw bit for low speed channel 3 duty change done."]
pub struct DUTY_CHNG_END_LSCH3_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH3_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH3_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH3_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_RAW` reader - The interrupt raw bit for low speed channel 4 duty change done."]
pub struct DUTY_CHNG_END_LSCH4_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH4_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH4_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH4_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_RAW` reader - The interrupt raw bit for low speed channel 5 duty change done."]
pub struct DUTY_CHNG_END_LSCH5_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH5_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH5_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH5_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH6_INT_RAW` reader - The interrupt raw bit for low speed channel 6 duty change done."]
pub struct DUTY_CHNG_END_LSCH6_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH6_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH6_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH6_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH7_INT_RAW` reader - The interrupt raw bit for low speed channel 7 duty change done."]
pub struct DUTY_CHNG_END_LSCH7_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH7_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH7_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH7_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The interrupt raw bit for high speed channel0 counter overflow."]
    #[inline(always)]
    pub fn hstimer0_ovf_int_raw(&self) -> HSTIMER0_OVF_INT_RAW_R {
        HSTIMER0_OVF_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for high speed channel1 counter overflow."]
    #[inline(always)]
    pub fn hstimer1_ovf_int_raw(&self) -> HSTIMER1_OVF_INT_RAW_R {
        HSTIMER1_OVF_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for high speed channel2 counter overflow."]
    #[inline(always)]
    pub fn hstimer2_ovf_int_raw(&self) -> HSTIMER2_OVF_INT_RAW_R {
        HSTIMER2_OVF_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for high speed channel3 counter overflow."]
    #[inline(always)]
    pub fn hstimer3_ovf_int_raw(&self) -> HSTIMER3_OVF_INT_RAW_R {
        HSTIMER3_OVF_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for low speed channel0 counter overflow."]
    #[inline(always)]
    pub fn lstimer0_ovf_int_raw(&self) -> LSTIMER0_OVF_INT_RAW_R {
        LSTIMER0_OVF_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for low speed channel1 counter overflow."]
    #[inline(always)]
    pub fn lstimer1_ovf_int_raw(&self) -> LSTIMER1_OVF_INT_RAW_R {
        LSTIMER1_OVF_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The interrupt raw bit for low speed channel2 counter overflow."]
    #[inline(always)]
    pub fn lstimer2_ovf_int_raw(&self) -> LSTIMER2_OVF_INT_RAW_R {
        LSTIMER2_OVF_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for low speed channel3 counter overflow."]
    #[inline(always)]
    pub fn lstimer3_ovf_int_raw(&self) -> LSTIMER3_OVF_INT_RAW_R {
        LSTIMER3_OVF_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The interrupt raw bit for high speed channel 0 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch0_int_raw(&self) -> DUTY_CHNG_END_HSCH0_INT_RAW_R {
        DUTY_CHNG_END_HSCH0_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The interrupt raw bit for high speed channel 1 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch1_int_raw(&self) -> DUTY_CHNG_END_HSCH1_INT_RAW_R {
        DUTY_CHNG_END_HSCH1_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The interrupt raw bit for high speed channel 2 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch2_int_raw(&self) -> DUTY_CHNG_END_HSCH2_INT_RAW_R {
        DUTY_CHNG_END_HSCH2_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The interrupt raw bit for high speed channel 3 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch3_int_raw(&self) -> DUTY_CHNG_END_HSCH3_INT_RAW_R {
        DUTY_CHNG_END_HSCH3_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The interrupt raw bit for high speed channel 4 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch4_int_raw(&self) -> DUTY_CHNG_END_HSCH4_INT_RAW_R {
        DUTY_CHNG_END_HSCH4_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The interrupt raw bit for high speed channel 5 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch5_int_raw(&self) -> DUTY_CHNG_END_HSCH5_INT_RAW_R {
        DUTY_CHNG_END_HSCH5_INT_RAW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The interrupt raw bit for high speed channel 6 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch6_int_raw(&self) -> DUTY_CHNG_END_HSCH6_INT_RAW_R {
        DUTY_CHNG_END_HSCH6_INT_RAW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - The interrupt raw bit for high speed channel 7 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch7_int_raw(&self) -> DUTY_CHNG_END_HSCH7_INT_RAW_R {
        DUTY_CHNG_END_HSCH7_INT_RAW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - The interrupt raw bit for low speed channel 0 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_raw(&self) -> DUTY_CHNG_END_LSCH0_INT_RAW_R {
        DUTY_CHNG_END_LSCH0_INT_RAW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - The interrupt raw bit for low speed channel 1 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_raw(&self) -> DUTY_CHNG_END_LSCH1_INT_RAW_R {
        DUTY_CHNG_END_LSCH1_INT_RAW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - The interrupt raw bit for low speed channel 2 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_raw(&self) -> DUTY_CHNG_END_LSCH2_INT_RAW_R {
        DUTY_CHNG_END_LSCH2_INT_RAW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The interrupt raw bit for low speed channel 3 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_raw(&self) -> DUTY_CHNG_END_LSCH3_INT_RAW_R {
        DUTY_CHNG_END_LSCH3_INT_RAW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - The interrupt raw bit for low speed channel 4 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_raw(&self) -> DUTY_CHNG_END_LSCH4_INT_RAW_R {
        DUTY_CHNG_END_LSCH4_INT_RAW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The interrupt raw bit for low speed channel 5 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_raw(&self) -> DUTY_CHNG_END_LSCH5_INT_RAW_R {
        DUTY_CHNG_END_LSCH5_INT_RAW_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - The interrupt raw bit for low speed channel 6 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch6_int_raw(&self) -> DUTY_CHNG_END_LSCH6_INT_RAW_R {
        DUTY_CHNG_END_LSCH6_INT_RAW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - The interrupt raw bit for low speed channel 7 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch7_int_raw(&self) -> DUTY_CHNG_END_LSCH7_INT_RAW_R {
        DUTY_CHNG_END_LSCH7_INT_RAW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
