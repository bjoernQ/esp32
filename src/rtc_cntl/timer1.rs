#[doc = "Register `TIMER1` reader"]
pub struct R(crate::R<TIMER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1` writer"]
pub struct W(crate::W<TIMER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TIMER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_STALL_EN` reader - CPU stall enable bit"]
pub struct CPU_STALL_EN_R(crate::FieldReader<bool, bool>);
impl CPU_STALL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPU_STALL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_STALL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_STALL_EN` writer - CPU stall enable bit"]
pub struct CPU_STALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_STALL_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CPU_STALL_WAIT` reader - CPU stall wait cycles in fast_clk_rtc"]
pub struct CPU_STALL_WAIT_R(crate::FieldReader<u8, u8>);
impl CPU_STALL_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPU_STALL_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_STALL_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_STALL_WAIT` writer - CPU stall wait cycles in fast_clk_rtc"]
pub struct CPU_STALL_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_STALL_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | ((value as u32 & 0x1f) << 1);
        self.w
    }
}
#[doc = "Field `CK8M_WAIT` reader - CK8M wait cycles in slow_clk_rtc"]
pub struct CK8M_WAIT_R(crate::FieldReader<u8, u8>);
impl CK8M_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CK8M_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_WAIT` writer - CK8M wait cycles in slow_clk_rtc"]
pub struct CK8M_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 6)) | ((value as u32 & 0xff) << 6);
        self.w
    }
}
#[doc = "Field `XTL_BUF_WAIT` reader - XTAL wait cycles in slow_clk_rtc"]
pub struct XTL_BUF_WAIT_R(crate::FieldReader<u16, u16>);
impl XTL_BUF_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        XTL_BUF_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_BUF_WAIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_BUF_WAIT` writer - XTAL wait cycles in slow_clk_rtc"]
pub struct XTL_BUF_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_BUF_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 14)) | ((value as u32 & 0x03ff) << 14);
        self.w
    }
}
#[doc = "Field `PLL_BUF_WAIT` reader - PLL wait cycles in slow_clk_rtc"]
pub struct PLL_BUF_WAIT_R(crate::FieldReader<u8, u8>);
impl PLL_BUF_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_BUF_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_BUF_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_BUF_WAIT` writer - PLL wait cycles in slow_clk_rtc"]
pub struct PLL_BUF_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_BUF_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CPU stall enable bit"]
    #[inline(always)]
    pub fn cpu_stall_en(&self) -> CPU_STALL_EN_R {
        CPU_STALL_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:5 - CPU stall wait cycles in fast_clk_rtc"]
    #[inline(always)]
    pub fn cpu_stall_wait(&self) -> CPU_STALL_WAIT_R {
        CPU_STALL_WAIT_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:13 - CK8M wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn ck8m_wait(&self) -> CK8M_WAIT_R {
        CK8M_WAIT_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:23 - XTAL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn xtl_buf_wait(&self) -> XTL_BUF_WAIT_R {
        XTL_BUF_WAIT_R::new(((self.bits >> 14) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:31 - PLL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn pll_buf_wait(&self) -> PLL_BUF_WAIT_R {
        PLL_BUF_WAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CPU stall enable bit"]
    #[inline(always)]
    pub fn cpu_stall_en(&mut self) -> CPU_STALL_EN_W {
        CPU_STALL_EN_W { w: self }
    }
    #[doc = "Bits 1:5 - CPU stall wait cycles in fast_clk_rtc"]
    #[inline(always)]
    pub fn cpu_stall_wait(&mut self) -> CPU_STALL_WAIT_W {
        CPU_STALL_WAIT_W { w: self }
    }
    #[doc = "Bits 6:13 - CK8M wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn ck8m_wait(&mut self) -> CK8M_WAIT_W {
        CK8M_WAIT_W { w: self }
    }
    #[doc = "Bits 14:23 - XTAL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn xtl_buf_wait(&mut self) -> XTL_BUF_WAIT_W {
        XTL_BUF_WAIT_W { w: self }
    }
    #[doc = "Bits 24:31 - PLL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn pll_buf_wait(&mut self) -> PLL_BUF_WAIT_W {
        PLL_BUF_WAIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1](index.html) module"]
pub struct TIMER1_SPEC;
impl crate::RegisterSpec for TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1::R](R) reader structure"]
impl crate::Readable for TIMER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1::W](W) writer structure"]
impl crate::Writable for TIMER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1 to value 0x2814_0403"]
impl crate::Resettable for TIMER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2814_0403
    }
}
