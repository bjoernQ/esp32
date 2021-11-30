#[doc = "Register `CLK_EDGE_SEL` reader"]
pub struct R(crate::R<CLK_EDGE_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_EDGE_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_EDGE_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_EDGE_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_EDGE_SEL` writer"]
pub struct W(crate::W<CLK_EDGE_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_EDGE_SEL_SPEC>;
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
impl From<crate::W<CLK_EDGE_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_EDGE_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCLKIN_EDGE_DRV_SEL` reader - It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270."]
pub struct CCLKIN_EDGE_DRV_SEL_R(crate::FieldReader<u8, u8>);
impl CCLKIN_EDGE_DRV_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCLKIN_EDGE_DRV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLKIN_EDGE_DRV_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLKIN_EDGE_DRV_SEL` writer - It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270."]
pub struct CCLKIN_EDGE_DRV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLKIN_EDGE_DRV_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `CCLKIN_EDGE_SAM_SEL` reader - It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270."]
pub struct CCLKIN_EDGE_SAM_SEL_R(crate::FieldReader<u8, u8>);
impl CCLKIN_EDGE_SAM_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCLKIN_EDGE_SAM_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLKIN_EDGE_SAM_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLKIN_EDGE_SAM_SEL` writer - It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270."]
pub struct CCLKIN_EDGE_SAM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLKIN_EDGE_SAM_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `CCLKIN_EDGE_SLF_SEL` reader - It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270."]
pub struct CCLKIN_EDGE_SLF_SEL_R(crate::FieldReader<u8, u8>);
impl CCLKIN_EDGE_SLF_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCLKIN_EDGE_SLF_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLKIN_EDGE_SLF_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLKIN_EDGE_SLF_SEL` writer - It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270."]
pub struct CCLKIN_EDGE_SLF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLKIN_EDGE_SLF_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `CCLLKIN_EDGE_H` reader - The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L."]
pub struct CCLLKIN_EDGE_H_R(crate::FieldReader<u8, u8>);
impl CCLLKIN_EDGE_H_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCLLKIN_EDGE_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLLKIN_EDGE_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLLKIN_EDGE_H` writer - The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L."]
pub struct CCLLKIN_EDGE_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLLKIN_EDGE_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | ((value as u32 & 0x0f) << 9);
        self.w
    }
}
#[doc = "Field `CCLLKIN_EDGE_L` reader - The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H."]
pub struct CCLLKIN_EDGE_L_R(crate::FieldReader<u8, u8>);
impl CCLLKIN_EDGE_L_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCLLKIN_EDGE_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLLKIN_EDGE_L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLLKIN_EDGE_L` writer - The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H."]
pub struct CCLLKIN_EDGE_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLLKIN_EDGE_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | ((value as u32 & 0x0f) << 13);
        self.w
    }
}
#[doc = "Field `CCLLKIN_EDGE_N` reader - The value should be equal to CCLKIN_EDGE_L."]
pub struct CCLLKIN_EDGE_N_R(crate::FieldReader<u8, u8>);
impl CCLLKIN_EDGE_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCLLKIN_EDGE_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLLKIN_EDGE_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLLKIN_EDGE_N` writer - The value should be equal to CCLKIN_EDGE_L."]
pub struct CCLLKIN_EDGE_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLLKIN_EDGE_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 17)) | ((value as u32 & 0x0f) << 17);
        self.w
    }
}
#[doc = "Field `ESDIO_MODE` reader - Enable esdio mode."]
pub struct ESDIO_MODE_R(crate::FieldReader<bool, bool>);
impl ESDIO_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESDIO_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESDIO_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESDIO_MODE` writer - Enable esdio mode."]
pub struct ESDIO_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ESDIO_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `ESD_MODE` reader - Enable esd mode."]
pub struct ESD_MODE_R(crate::FieldReader<bool, bool>);
impl ESD_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESD_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESD_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESD_MODE` writer - Enable esd mode."]
pub struct ESD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ESD_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CCLK_EN` reader - Sdio clock enable"]
pub struct CCLK_EN_R(crate::FieldReader<bool, bool>);
impl CCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLK_EN` writer - Sdio clock enable"]
pub struct CCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_drv_sel(&self) -> CCLKIN_EDGE_DRV_SEL_R {
        CCLKIN_EDGE_DRV_SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_sam_sel(&self) -> CCLKIN_EDGE_SAM_SEL_R {
        CCLKIN_EDGE_SAM_SEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_slf_sel(&self) -> CCLKIN_EDGE_SLF_SEL_R {
        CCLKIN_EDGE_SLF_SEL_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:12 - The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L."]
    #[inline(always)]
    pub fn ccllkin_edge_h(&self) -> CCLLKIN_EDGE_H_R {
        CCLLKIN_EDGE_H_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H."]
    #[inline(always)]
    pub fn ccllkin_edge_l(&self) -> CCLLKIN_EDGE_L_R {
        CCLLKIN_EDGE_L_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:20 - The value should be equal to CCLKIN_EDGE_L."]
    #[inline(always)]
    pub fn ccllkin_edge_n(&self) -> CCLLKIN_EDGE_N_R {
        CCLLKIN_EDGE_N_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Enable esdio mode."]
    #[inline(always)]
    pub fn esdio_mode(&self) -> ESDIO_MODE_R {
        ESDIO_MODE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable esd mode."]
    #[inline(always)]
    pub fn esd_mode(&self) -> ESD_MODE_R {
        ESD_MODE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Sdio clock enable"]
    #[inline(always)]
    pub fn cclk_en(&self) -> CCLK_EN_R {
        CCLK_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_drv_sel(&mut self) -> CCLKIN_EDGE_DRV_SEL_W {
        CCLKIN_EDGE_DRV_SEL_W { w: self }
    }
    #[doc = "Bits 3:5 - It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_sam_sel(&mut self) -> CCLKIN_EDGE_SAM_SEL_W {
        CCLKIN_EDGE_SAM_SEL_W { w: self }
    }
    #[doc = "Bits 6:8 - It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_slf_sel(&mut self) -> CCLKIN_EDGE_SLF_SEL_W {
        CCLKIN_EDGE_SLF_SEL_W { w: self }
    }
    #[doc = "Bits 9:12 - The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L."]
    #[inline(always)]
    pub fn ccllkin_edge_h(&mut self) -> CCLLKIN_EDGE_H_W {
        CCLLKIN_EDGE_H_W { w: self }
    }
    #[doc = "Bits 13:16 - The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H."]
    #[inline(always)]
    pub fn ccllkin_edge_l(&mut self) -> CCLLKIN_EDGE_L_W {
        CCLLKIN_EDGE_L_W { w: self }
    }
    #[doc = "Bits 17:20 - The value should be equal to CCLKIN_EDGE_L."]
    #[inline(always)]
    pub fn ccllkin_edge_n(&mut self) -> CCLLKIN_EDGE_N_W {
        CCLLKIN_EDGE_N_W { w: self }
    }
    #[doc = "Bit 21 - Enable esdio mode."]
    #[inline(always)]
    pub fn esdio_mode(&mut self) -> ESDIO_MODE_W {
        ESDIO_MODE_W { w: self }
    }
    #[doc = "Bit 22 - Enable esd mode."]
    #[inline(always)]
    pub fn esd_mode(&mut self) -> ESD_MODE_W {
        ESD_MODE_W { w: self }
    }
    #[doc = "Bit 23 - Sdio clock enable"]
    #[inline(always)]
    pub fn cclk_en(&mut self) -> CCLK_EN_W {
        CCLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_edge_sel](index.html) module"]
pub struct CLK_EDGE_SEL_SPEC;
impl crate::RegisterSpec for CLK_EDGE_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_edge_sel::R](R) reader structure"]
impl crate::Readable for CLK_EDGE_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_edge_sel::W](W) writer structure"]
impl crate::Writable for CLK_EDGE_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_EDGE_SEL to value 0x0082_0200"]
impl crate::Resettable for CLK_EDGE_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0082_0200
    }
}
