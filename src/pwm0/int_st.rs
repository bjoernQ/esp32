#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER0_STOP_INT_ST` reader - "]
pub struct TIMER0_STOP_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER0_STOP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_STOP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_STOP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_STOP_INT_ST` reader - "]
pub struct TIMER1_STOP_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER1_STOP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_STOP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_STOP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER2_STOP_INT_ST` reader - "]
pub struct TIMER2_STOP_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER2_STOP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER2_STOP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_STOP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_TEZ_INT_ST` reader - "]
pub struct TIMER0_TEZ_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER0_TEZ_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_TEZ_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_TEZ_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_TEZ_INT_ST` reader - "]
pub struct TIMER1_TEZ_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER1_TEZ_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_TEZ_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_TEZ_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER2_TEZ_INT_ST` reader - "]
pub struct TIMER2_TEZ_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER2_TEZ_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER2_TEZ_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_TEZ_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_TEP_INT_ST` reader - "]
pub struct TIMER0_TEP_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER0_TEP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_TEP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_TEP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_TEP_INT_ST` reader - "]
pub struct TIMER1_TEP_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER1_TEP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_TEP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_TEP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER2_TEP_INT_ST` reader - "]
pub struct TIMER2_TEP_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMER2_TEP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER2_TEP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_TEP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT0_INT_ST` reader - "]
pub struct FAULT0_INT_ST_R(crate::FieldReader<bool, bool>);
impl FAULT0_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT0_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT0_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT1_INT_ST` reader - "]
pub struct FAULT1_INT_ST_R(crate::FieldReader<bool, bool>);
impl FAULT1_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT1_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT1_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT2_INT_ST` reader - "]
pub struct FAULT2_INT_ST_R(crate::FieldReader<bool, bool>);
impl FAULT2_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT0_CLR_INT_ST` reader - "]
pub struct FAULT0_CLR_INT_ST_R(crate::FieldReader<bool, bool>);
impl FAULT0_CLR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT0_CLR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT0_CLR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT1_CLR_INT_ST` reader - "]
pub struct FAULT1_CLR_INT_ST_R(crate::FieldReader<bool, bool>);
impl FAULT1_CLR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT1_CLR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT1_CLR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT2_CLR_INT_ST` reader - "]
pub struct FAULT2_CLR_INT_ST_R(crate::FieldReader<bool, bool>);
impl FAULT2_CLR_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT2_CLR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT2_CLR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP0_TEA_INT_ST` reader - "]
pub struct OP0_TEA_INT_ST_R(crate::FieldReader<bool, bool>);
impl OP0_TEA_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP0_TEA_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP0_TEA_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP1_TEA_INT_ST` reader - "]
pub struct OP1_TEA_INT_ST_R(crate::FieldReader<bool, bool>);
impl OP1_TEA_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP1_TEA_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP1_TEA_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP2_TEA_INT_ST` reader - "]
pub struct OP2_TEA_INT_ST_R(crate::FieldReader<bool, bool>);
impl OP2_TEA_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP2_TEA_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP2_TEA_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP0_TEB_INT_ST` reader - "]
pub struct OP0_TEB_INT_ST_R(crate::FieldReader<bool, bool>);
impl OP0_TEB_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP0_TEB_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP0_TEB_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP1_TEB_INT_ST` reader - "]
pub struct OP1_TEB_INT_ST_R(crate::FieldReader<bool, bool>);
impl OP1_TEB_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP1_TEB_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP1_TEB_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OP2_TEB_INT_ST` reader - "]
pub struct OP2_TEB_INT_ST_R(crate::FieldReader<bool, bool>);
impl OP2_TEB_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OP2_TEB_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OP2_TEB_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH0_CBC_INT_ST` reader - "]
pub struct FH0_CBC_INT_ST_R(crate::FieldReader<bool, bool>);
impl FH0_CBC_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH0_CBC_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH0_CBC_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH1_CBC_INT_ST` reader - "]
pub struct FH1_CBC_INT_ST_R(crate::FieldReader<bool, bool>);
impl FH1_CBC_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH1_CBC_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH1_CBC_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_CBC_INT_ST` reader - "]
pub struct FH2_CBC_INT_ST_R(crate::FieldReader<bool, bool>);
impl FH2_CBC_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH2_CBC_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_CBC_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH0_OST_INT_ST` reader - "]
pub struct FH0_OST_INT_ST_R(crate::FieldReader<bool, bool>);
impl FH0_OST_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH0_OST_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH0_OST_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH1_OST_INT_ST` reader - "]
pub struct FH1_OST_INT_ST_R(crate::FieldReader<bool, bool>);
impl FH1_OST_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH1_OST_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH1_OST_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH2_OST_INT_ST` reader - "]
pub struct FH2_OST_INT_ST_R(crate::FieldReader<bool, bool>);
impl FH2_OST_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH2_OST_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH2_OST_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP0_INT_ST` reader - "]
pub struct CAP0_INT_ST_R(crate::FieldReader<bool, bool>);
impl CAP0_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAP0_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP0_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP1_INT_ST` reader - "]
pub struct CAP1_INT_ST_R(crate::FieldReader<bool, bool>);
impl CAP1_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAP1_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP1_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP2_INT_ST` reader - "]
pub struct CAP2_INT_ST_R(crate::FieldReader<bool, bool>);
impl CAP2_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAP2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer0_stop_int_st(&self) -> TIMER0_STOP_INT_ST_R {
        TIMER0_STOP_INT_ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer1_stop_int_st(&self) -> TIMER1_STOP_INT_ST_R {
        TIMER1_STOP_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer2_stop_int_st(&self) -> TIMER2_STOP_INT_ST_R {
        TIMER2_STOP_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timer0_tez_int_st(&self) -> TIMER0_TEZ_INT_ST_R {
        TIMER0_TEZ_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timer1_tez_int_st(&self) -> TIMER1_TEZ_INT_ST_R {
        TIMER1_TEZ_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn timer2_tez_int_st(&self) -> TIMER2_TEZ_INT_ST_R {
        TIMER2_TEZ_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn timer0_tep_int_st(&self) -> TIMER0_TEP_INT_ST_R {
        TIMER0_TEP_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn timer1_tep_int_st(&self) -> TIMER1_TEP_INT_ST_R {
        TIMER1_TEP_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn timer2_tep_int_st(&self) -> TIMER2_TEP_INT_ST_R {
        TIMER2_TEP_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fault0_int_st(&self) -> FAULT0_INT_ST_R {
        FAULT0_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fault1_int_st(&self) -> FAULT1_INT_ST_R {
        FAULT1_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn fault2_int_st(&self) -> FAULT2_INT_ST_R {
        FAULT2_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fault0_clr_int_st(&self) -> FAULT0_CLR_INT_ST_R {
        FAULT0_CLR_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fault1_clr_int_st(&self) -> FAULT1_CLR_INT_ST_R {
        FAULT1_CLR_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fault2_clr_int_st(&self) -> FAULT2_CLR_INT_ST_R {
        FAULT2_CLR_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn op0_tea_int_st(&self) -> OP0_TEA_INT_ST_R {
        OP0_TEA_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn op1_tea_int_st(&self) -> OP1_TEA_INT_ST_R {
        OP1_TEA_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn op2_tea_int_st(&self) -> OP2_TEA_INT_ST_R {
        OP2_TEA_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn op0_teb_int_st(&self) -> OP0_TEB_INT_ST_R {
        OP0_TEB_INT_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn op1_teb_int_st(&self) -> OP1_TEB_INT_ST_R {
        OP1_TEB_INT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn op2_teb_int_st(&self) -> OP2_TEB_INT_ST_R {
        OP2_TEB_INT_ST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn fh0_cbc_int_st(&self) -> FH0_CBC_INT_ST_R {
        FH0_CBC_INT_ST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn fh1_cbc_int_st(&self) -> FH1_CBC_INT_ST_R {
        FH1_CBC_INT_ST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn fh2_cbc_int_st(&self) -> FH2_CBC_INT_ST_R {
        FH2_CBC_INT_ST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn fh0_ost_int_st(&self) -> FH0_OST_INT_ST_R {
        FH0_OST_INT_ST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn fh1_ost_int_st(&self) -> FH1_OST_INT_ST_R {
        FH1_OST_INT_ST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn fh2_ost_int_st(&self) -> FH2_OST_INT_ST_R {
        FH2_OST_INT_ST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cap0_int_st(&self) -> CAP0_INT_ST_R {
        CAP0_INT_ST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cap1_int_st(&self) -> CAP1_INT_ST_R {
        CAP1_INT_ST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cap2_int_st(&self) -> CAP2_INT_ST_R {
        CAP2_INT_ST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
