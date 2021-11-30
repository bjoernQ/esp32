#[doc = "Register `CARDTHRCTL` reader"]
pub struct R(crate::R<CARDTHRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CARDTHRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CARDTHRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CARDTHRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CARDTHRCTL` writer"]
pub struct W(crate::W<CARDTHRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CARDTHRCTL_SPEC>;
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
impl From<crate::W<CARDTHRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CARDTHRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARDRDTHREN` reader - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled."]
pub struct CARDRDTHREN_R(crate::FieldReader<bool, bool>);
impl CARDRDTHREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARDRDTHREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARDRDTHREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARDRDTHREN` writer - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled."]
pub struct CARDRDTHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDRDTHREN_W<'a> {
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
#[doc = "Field `CARDCLRINTEN` reader - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled."]
pub struct CARDCLRINTEN_R(crate::FieldReader<bool, bool>);
impl CARDCLRINTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARDCLRINTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARDCLRINTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARDCLRINTEN` writer - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled."]
pub struct CARDCLRINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDCLRINTEN_W<'a> {
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
#[doc = "Field `CARDWRTHREN` reader - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled."]
pub struct CARDWRTHREN_R(crate::FieldReader<bool, bool>);
impl CARDWRTHREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARDWRTHREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARDWRTHREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARDWRTHREN` writer - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled."]
pub struct CARDWRTHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDWRTHREN_W<'a> {
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
#[doc = "Field `CARDTHRESHOLD` reader - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1."]
pub struct CARDTHRESHOLD_R(crate::FieldReader<u16, u16>);
impl CARDTHRESHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CARDTHRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARDTHRESHOLD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARDTHRESHOLD` writer - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1."]
pub struct CARDTHRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDTHRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled."]
    #[inline(always)]
    pub fn cardrdthren(&self) -> CARDRDTHREN_R {
        CARDRDTHREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled."]
    #[inline(always)]
    pub fn cardclrinten(&self) -> CARDCLRINTEN_R {
        CARDCLRINTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled."]
    #[inline(always)]
    pub fn cardwrthren(&self) -> CARDWRTHREN_R {
        CARDWRTHREN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1."]
    #[inline(always)]
    pub fn cardthreshold(&self) -> CARDTHRESHOLD_R {
        CARDTHRESHOLD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled."]
    #[inline(always)]
    pub fn cardrdthren(&mut self) -> CARDRDTHREN_W {
        CARDRDTHREN_W { w: self }
    }
    #[doc = "Bit 1 - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled."]
    #[inline(always)]
    pub fn cardclrinten(&mut self) -> CARDCLRINTEN_W {
        CARDCLRINTEN_W { w: self }
    }
    #[doc = "Bit 2 - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled."]
    #[inline(always)]
    pub fn cardwrthren(&mut self) -> CARDWRTHREN_W {
        CARDWRTHREN_W { w: self }
    }
    #[doc = "Bits 16:31 - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1."]
    #[inline(always)]
    pub fn cardthreshold(&mut self) -> CARDTHRESHOLD_W {
        CARDTHRESHOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Card Threshold Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cardthrctl](index.html) module"]
pub struct CARDTHRCTL_SPEC;
impl crate::RegisterSpec for CARDTHRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cardthrctl::R](R) reader structure"]
impl crate::Readable for CARDTHRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cardthrctl::W](W) writer structure"]
impl crate::Writable for CARDTHRCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CARDTHRCTL to value 0"]
impl crate::Resettable for CARDTHRCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
