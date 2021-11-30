#[doc = "Register `NRXPD_CTRL` reader"]
pub struct R(crate::R<NRXPD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRXPD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NRXPD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NRXPD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NRXPD_CTRL` writer"]
pub struct W(crate::W<NRXPD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NRXPD_CTRL_SPEC>;
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
impl From<crate::W<NRXPD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NRXPD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEMAP_FORCE_PD` reader - "]
pub struct DEMAP_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl DEMAP_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEMAP_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEMAP_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEMAP_FORCE_PD` writer - "]
pub struct DEMAP_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEMAP_FORCE_PD_W<'a> {
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
#[doc = "Field `DEMAP_FORCE_PU` reader - "]
pub struct DEMAP_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl DEMAP_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEMAP_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEMAP_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEMAP_FORCE_PU` writer - "]
pub struct DEMAP_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DEMAP_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `VIT_FORCE_PD` reader - "]
pub struct VIT_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl VIT_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VIT_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VIT_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VIT_FORCE_PD` writer - "]
pub struct VIT_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> VIT_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `VIT_FORCE_PU` reader - "]
pub struct VIT_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl VIT_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VIT_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VIT_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VIT_FORCE_PU` writer - "]
pub struct VIT_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> VIT_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RX_ROT_FORCE_PD` reader - "]
pub struct RX_ROT_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl RX_ROT_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_ROT_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ROT_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ROT_FORCE_PD` writer - "]
pub struct RX_ROT_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ROT_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RX_ROT_FORCE_PU` reader - "]
pub struct RX_ROT_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl RX_ROT_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_ROT_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ROT_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ROT_FORCE_PU` writer - "]
pub struct RX_ROT_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ROT_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CHAN_EST_FORCE_PD` reader - "]
pub struct CHAN_EST_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl CHAN_EST_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHAN_EST_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHAN_EST_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHAN_EST_FORCE_PD` writer - "]
pub struct CHAN_EST_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAN_EST_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CHAN_EST_FORCE_PU` reader - "]
pub struct CHAN_EST_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl CHAN_EST_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHAN_EST_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHAN_EST_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHAN_EST_FORCE_PU` writer - "]
pub struct CHAN_EST_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAN_EST_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn demap_force_pd(&self) -> DEMAP_FORCE_PD_R {
        DEMAP_FORCE_PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn demap_force_pu(&self) -> DEMAP_FORCE_PU_R {
        DEMAP_FORCE_PU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vit_force_pd(&self) -> VIT_FORCE_PD_R {
        VIT_FORCE_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vit_force_pu(&self) -> VIT_FORCE_PU_R {
        VIT_FORCE_PU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_rot_force_pd(&self) -> RX_ROT_FORCE_PD_R {
        RX_ROT_FORCE_PD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_rot_force_pu(&self) -> RX_ROT_FORCE_PU_R {
        RX_ROT_FORCE_PU_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn chan_est_force_pd(&self) -> CHAN_EST_FORCE_PD_R {
        CHAN_EST_FORCE_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn chan_est_force_pu(&self) -> CHAN_EST_FORCE_PU_R {
        CHAN_EST_FORCE_PU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn demap_force_pd(&mut self) -> DEMAP_FORCE_PD_W {
        DEMAP_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn demap_force_pu(&mut self) -> DEMAP_FORCE_PU_W {
        DEMAP_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vit_force_pd(&mut self) -> VIT_FORCE_PD_W {
        VIT_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vit_force_pu(&mut self) -> VIT_FORCE_PU_W {
        VIT_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_rot_force_pd(&mut self) -> RX_ROT_FORCE_PD_W {
        RX_ROT_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_rot_force_pu(&mut self) -> RX_ROT_FORCE_PU_W {
        RX_ROT_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn chan_est_force_pd(&mut self) -> CHAN_EST_FORCE_PD_W {
        CHAN_EST_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn chan_est_force_pu(&mut self) -> CHAN_EST_FORCE_PU_W {
        CHAN_EST_FORCE_PU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WiFi RX control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrxpd_ctrl](index.html) module"]
pub struct NRXPD_CTRL_SPEC;
impl crate::RegisterSpec for NRXPD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nrxpd_ctrl::R](R) reader structure"]
impl crate::Readable for NRXPD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nrxpd_ctrl::W](W) writer structure"]
impl crate::Writable for NRXPD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NRXPD_CTRL to value 0"]
impl crate::Resettable for NRXPD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
