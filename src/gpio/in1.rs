#[doc = "Register `IN1` reader"]
pub struct R(crate::R<IN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN1` writer"]
pub struct W(crate::W<IN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN1_SPEC>;
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
impl From<crate::W<IN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `data_next` reader - GPIO32~39 input value"]
pub struct DATA_NEXT_R(crate::FieldReader<u8, u8>);
impl DATA_NEXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_NEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_NEXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `data_next` writer - GPIO32~39 input value"]
pub struct DATA_NEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_NEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 input value"]
    #[inline(always)]
    pub fn data_next(&self) -> DATA_NEXT_R {
        DATA_NEXT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 input value"]
    #[inline(always)]
    pub fn data_next(&mut self) -> DATA_NEXT_W {
        DATA_NEXT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in1](index.html) module"]
pub struct IN1_SPEC;
impl crate::RegisterSpec for IN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in1::R](R) reader structure"]
impl crate::Readable for IN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in1::W](W) writer structure"]
impl crate::Writable for IN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN1 to value 0"]
impl crate::Resettable for IN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
