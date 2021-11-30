#[doc = "Register `WAKEUP_STATE` reader"]
pub struct R(crate::R<WAKEUP_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEUP_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEUP_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEUP_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEUP_STATE` writer"]
pub struct W(crate::W<WAKEUP_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEUP_STATE_SPEC>;
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
impl From<crate::W<WAKEUP_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEUP_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUP_CAUSE` reader - wakeup cause"]
pub struct WAKEUP_CAUSE_R(crate::FieldReader<u16, u16>);
impl WAKEUP_CAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WAKEUP_CAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_CAUSE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP_ENA` reader - wakeup enable bitmap"]
pub struct WAKEUP_ENA_R(crate::FieldReader<u16, u16>);
impl WAKEUP_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WAKEUP_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_ENA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP_ENA` writer - wakeup enable bitmap"]
pub struct WAKEUP_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | ((value as u32 & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Field `GPIO_WAKEUP_FILTER` reader - enable filter for gpio wakeup event"]
pub struct GPIO_WAKEUP_FILTER_R(crate::FieldReader<bool, bool>);
impl GPIO_WAKEUP_FILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_WAKEUP_FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_WAKEUP_FILTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_WAKEUP_FILTER` writer - enable filter for gpio wakeup event"]
pub struct GPIO_WAKEUP_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_WAKEUP_FILTER_W<'a> {
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
impl R {
    #[doc = "Bits 0:10 - wakeup cause"]
    #[inline(always)]
    pub fn wakeup_cause(&self) -> WAKEUP_CAUSE_R {
        WAKEUP_CAUSE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - wakeup enable bitmap"]
    #[inline(always)]
    pub fn wakeup_ena(&self) -> WAKEUP_ENA_R {
        WAKEUP_ENA_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bit 22 - enable filter for gpio wakeup event"]
    #[inline(always)]
    pub fn gpio_wakeup_filter(&self) -> GPIO_WAKEUP_FILTER_R {
        GPIO_WAKEUP_FILTER_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 11:21 - wakeup enable bitmap"]
    #[inline(always)]
    pub fn wakeup_ena(&mut self) -> WAKEUP_ENA_W {
        WAKEUP_ENA_W { w: self }
    }
    #[doc = "Bit 22 - enable filter for gpio wakeup event"]
    #[inline(always)]
    pub fn gpio_wakeup_filter(&mut self) -> GPIO_WAKEUP_FILTER_W {
        GPIO_WAKEUP_FILTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup_state](index.html) module"]
pub struct WAKEUP_STATE_SPEC;
impl crate::RegisterSpec for WAKEUP_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeup_state::R](R) reader structure"]
impl crate::Readable for WAKEUP_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeup_state::W](W) writer structure"]
impl crate::Writable for WAKEUP_STATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKEUP_STATE to value 0x6000"]
impl crate::Resettable for WAKEUP_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6000
    }
}
