#[doc = "Register `SAR_MEAS_CTRL` reader"]
pub struct R(crate::R<SAR_MEAS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS_CTRL` writer"]
pub struct W(crate::W<SAR_MEAS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS_CTRL_SPEC>;
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
impl From<crate::W<SAR_MEAS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XPD_SAR_AMP_FSM` reader - "]
pub struct XPD_SAR_AMP_FSM_R(crate::FieldReader<u8, u8>);
impl XPD_SAR_AMP_FSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XPD_SAR_AMP_FSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_SAR_AMP_FSM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_SAR_AMP_FSM` writer - "]
pub struct XPD_SAR_AMP_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SAR_AMP_FSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `AMP_RST_FB_FSM` reader - "]
pub struct AMP_RST_FB_FSM_R(crate::FieldReader<u8, u8>);
impl AMP_RST_FB_FSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AMP_RST_FB_FSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMP_RST_FB_FSM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMP_RST_FB_FSM` writer - "]
pub struct AMP_RST_FB_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_RST_FB_FSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `AMP_SHORT_REF_FSM` reader - "]
pub struct AMP_SHORT_REF_FSM_R(crate::FieldReader<u8, u8>);
impl AMP_SHORT_REF_FSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AMP_SHORT_REF_FSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMP_SHORT_REF_FSM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMP_SHORT_REF_FSM` writer - "]
pub struct AMP_SHORT_REF_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_SHORT_REF_FSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `AMP_SHORT_REF_GND_FSM` reader - "]
pub struct AMP_SHORT_REF_GND_FSM_R(crate::FieldReader<u8, u8>);
impl AMP_SHORT_REF_GND_FSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AMP_SHORT_REF_GND_FSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMP_SHORT_REF_GND_FSM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMP_SHORT_REF_GND_FSM` writer - "]
pub struct AMP_SHORT_REF_GND_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_SHORT_REF_GND_FSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `XPD_SAR_FSM` reader - "]
pub struct XPD_SAR_FSM_R(crate::FieldReader<u8, u8>);
impl XPD_SAR_FSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XPD_SAR_FSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_SAR_FSM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_SAR_FSM` writer - "]
pub struct XPD_SAR_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SAR_FSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `SAR_RSTB_FSM` reader - "]
pub struct SAR_RSTB_FSM_R(crate::FieldReader<u8, u8>);
impl SAR_RSTB_FSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR_RSTB_FSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_RSTB_FSM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_RSTB_FSM` writer - "]
pub struct SAR_RSTB_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_RSTB_FSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `SAR2_XPD_WAIT` reader - "]
pub struct SAR2_XPD_WAIT_R(crate::FieldReader<u8, u8>);
impl SAR2_XPD_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_XPD_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_XPD_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_XPD_WAIT` writer - "]
pub struct SAR2_XPD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_XPD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm(&self) -> XPD_SAR_AMP_FSM_R {
        XPD_SAR_AMP_FSM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm(&self) -> AMP_RST_FB_FSM_R {
        AMP_RST_FB_FSM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn amp_short_ref_fsm(&self) -> AMP_SHORT_REF_FSM_R {
        AMP_SHORT_REF_FSM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm(&self) -> AMP_SHORT_REF_GND_FSM_R {
        AMP_SHORT_REF_GND_FSM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn xpd_sar_fsm(&self) -> XPD_SAR_FSM_R {
        XPD_SAR_FSM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn sar_rstb_fsm(&self) -> SAR_RSTB_FSM_R {
        SAR_RSTB_FSM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sar2_xpd_wait(&self) -> SAR2_XPD_WAIT_R {
        SAR2_XPD_WAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm(&mut self) -> XPD_SAR_AMP_FSM_W {
        XPD_SAR_AMP_FSM_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm(&mut self) -> AMP_RST_FB_FSM_W {
        AMP_RST_FB_FSM_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn amp_short_ref_fsm(&mut self) -> AMP_SHORT_REF_FSM_W {
        AMP_SHORT_REF_FSM_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm(&mut self) -> AMP_SHORT_REF_GND_FSM_W {
        AMP_SHORT_REF_GND_FSM_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn xpd_sar_fsm(&mut self) -> XPD_SAR_FSM_W {
        XPD_SAR_FSM_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn sar_rstb_fsm(&mut self) -> SAR_RSTB_FSM_W {
        SAR_RSTB_FSM_W { w: self }
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sar2_xpd_wait(&mut self) -> SAR2_XPD_WAIT_W {
        SAR2_XPD_WAIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas_ctrl](index.html) module"]
pub struct SAR_MEAS_CTRL_SPEC;
impl crate::RegisterSpec for SAR_MEAS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas_ctrl::R](R) reader structure"]
impl crate::Readable for SAR_MEAS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas_ctrl::W](W) writer structure"]
impl crate::Writable for SAR_MEAS_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_MEAS_CTRL to value 0x0707_338f"]
impl crate::Resettable for SAR_MEAS_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0707_338f
    }
}
