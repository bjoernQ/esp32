#[doc = "Register `LC_HUNG_CONF` reader"]
pub struct R(crate::R<LC_HUNG_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LC_HUNG_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LC_HUNG_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LC_HUNG_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LC_HUNG_CONF` writer"]
pub struct W(crate::W<LC_HUNG_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LC_HUNG_CONF_SPEC>;
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
impl From<crate::W<LC_HUNG_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LC_HUNG_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LC_FIFO_TIMEOUT` reader - "]
pub struct LC_FIFO_TIMEOUT_R(crate::FieldReader<u8, u8>);
impl LC_FIFO_TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LC_FIFO_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LC_FIFO_TIMEOUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LC_FIFO_TIMEOUT` writer - "]
pub struct LC_FIFO_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> LC_FIFO_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `LC_FIFO_TIMEOUT_SHIFT` reader - "]
pub struct LC_FIFO_TIMEOUT_SHIFT_R(crate::FieldReader<u8, u8>);
impl LC_FIFO_TIMEOUT_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LC_FIFO_TIMEOUT_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LC_FIFO_TIMEOUT_SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LC_FIFO_TIMEOUT_SHIFT` writer - "]
pub struct LC_FIFO_TIMEOUT_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> LC_FIFO_TIMEOUT_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `LC_FIFO_TIMEOUT_ENA` reader - "]
pub struct LC_FIFO_TIMEOUT_ENA_R(crate::FieldReader<bool, bool>);
impl LC_FIFO_TIMEOUT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LC_FIFO_TIMEOUT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LC_FIFO_TIMEOUT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LC_FIFO_TIMEOUT_ENA` writer - "]
pub struct LC_FIFO_TIMEOUT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LC_FIFO_TIMEOUT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lc_fifo_timeout(&self) -> LC_FIFO_TIMEOUT_R {
        LC_FIFO_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn lc_fifo_timeout_shift(&self) -> LC_FIFO_TIMEOUT_SHIFT_R {
        LC_FIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lc_fifo_timeout_ena(&self) -> LC_FIFO_TIMEOUT_ENA_R {
        LC_FIFO_TIMEOUT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lc_fifo_timeout(&mut self) -> LC_FIFO_TIMEOUT_W {
        LC_FIFO_TIMEOUT_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn lc_fifo_timeout_shift(&mut self) -> LC_FIFO_TIMEOUT_SHIFT_W {
        LC_FIFO_TIMEOUT_SHIFT_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lc_fifo_timeout_ena(&mut self) -> LC_FIFO_TIMEOUT_ENA_W {
        LC_FIFO_TIMEOUT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc_hung_conf](index.html) module"]
pub struct LC_HUNG_CONF_SPEC;
impl crate::RegisterSpec for LC_HUNG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lc_hung_conf::R](R) reader structure"]
impl crate::Readable for LC_HUNG_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lc_hung_conf::W](W) writer structure"]
impl crate::Writable for LC_HUNG_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LC_HUNG_CONF to value 0x0810"]
impl crate::Resettable for LC_HUNG_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0810
    }
}
