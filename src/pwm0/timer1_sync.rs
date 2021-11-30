#[doc = "Register `TIMER1_SYNC` reader"]
pub struct R(crate::R<TIMER1_SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1_SYNC` writer"]
pub struct W(crate::W<TIMER1_SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_SYNC_SPEC>;
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
impl From<crate::W<TIMER1_SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER1_SYNCI_EN` reader - "]
pub struct TIMER1_SYNCI_EN_R(crate::FieldReader<bool, bool>);
impl TIMER1_SYNCI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_SYNCI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_SYNCI_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_SYNCI_EN` writer - "]
pub struct TIMER1_SYNCI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_SYNCI_EN_W<'a> {
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
#[doc = "Field `SW` reader - "]
pub struct SW_R(crate::FieldReader<bool, bool>);
impl SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW` writer - "]
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
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
#[doc = "Field `TIMER1_SYNCO_SEL` reader - "]
pub struct TIMER1_SYNCO_SEL_R(crate::FieldReader<u8, u8>);
impl TIMER1_SYNCO_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER1_SYNCO_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_SYNCO_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_SYNCO_SEL` writer - "]
pub struct TIMER1_SYNCO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_SYNCO_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `TIMER1_PHASE` reader - "]
pub struct TIMER1_PHASE_R(crate::FieldReader<u16, u16>);
impl TIMER1_PHASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMER1_PHASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_PHASE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_PHASE` writer - "]
pub struct TIMER1_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_PHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 4)) | ((value as u32 & 0xffff) << 4);
        self.w
    }
}
#[doc = "Field `TIMER1_PHASE_DIRECTION` reader - "]
pub struct TIMER1_PHASE_DIRECTION_R(crate::FieldReader<bool, bool>);
impl TIMER1_PHASE_DIRECTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_PHASE_DIRECTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_PHASE_DIRECTION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_PHASE_DIRECTION` writer - "]
pub struct TIMER1_PHASE_DIRECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_PHASE_DIRECTION_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer1_synci_en(&self) -> TIMER1_SYNCI_EN_R {
        TIMER1_SYNCI_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn timer1_synco_sel(&self) -> TIMER1_SYNCO_SEL_R {
        TIMER1_SYNCO_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn timer1_phase(&self) -> TIMER1_PHASE_R {
        TIMER1_PHASE_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn timer1_phase_direction(&self) -> TIMER1_PHASE_DIRECTION_R {
        TIMER1_PHASE_DIRECTION_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer1_synci_en(&mut self) -> TIMER1_SYNCI_EN_W {
        TIMER1_SYNCI_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn timer1_synco_sel(&mut self) -> TIMER1_SYNCO_SEL_W {
        TIMER1_SYNCO_SEL_W { w: self }
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn timer1_phase(&mut self) -> TIMER1_PHASE_W {
        TIMER1_PHASE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn timer1_phase_direction(&mut self) -> TIMER1_PHASE_DIRECTION_W {
        TIMER1_PHASE_DIRECTION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_sync](index.html) module"]
pub struct TIMER1_SYNC_SPEC;
impl crate::RegisterSpec for TIMER1_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_sync::R](R) reader structure"]
impl crate::Readable for TIMER1_SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_sync::W](W) writer structure"]
impl crate::Writable for TIMER1_SYNC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1_SYNC to value 0"]
impl crate::Resettable for TIMER1_SYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
