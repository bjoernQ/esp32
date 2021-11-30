#[doc = "Register `AT_CMD_CHAR` reader"]
pub struct R(crate::R<AT_CMD_CHAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AT_CMD_CHAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AT_CMD_CHAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AT_CMD_CHAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AT_CMD_CHAR` writer"]
pub struct W(crate::W<AT_CMD_CHAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AT_CMD_CHAR_SPEC>;
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
impl From<crate::W<AT_CMD_CHAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AT_CMD_CHAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AT_CMD_CHAR` reader - This register is used to configure the content of at_cmd char."]
pub struct AT_CMD_CHAR_R(crate::FieldReader<u8, u8>);
impl AT_CMD_CHAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AT_CMD_CHAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AT_CMD_CHAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AT_CMD_CHAR` writer - This register is used to configure the content of at_cmd char."]
pub struct AT_CMD_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> AT_CMD_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CHAR_NUM` reader - This register is used to configure the num of continous at_cmd chars received by receiver."]
pub struct CHAR_NUM_R(crate::FieldReader<u8, u8>);
impl CHAR_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHAR_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHAR_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHAR_NUM` writer - This register is used to configure the num of continous at_cmd chars received by receiver."]
pub struct CHAR_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAR_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the content of at_cmd char."]
    #[inline(always)]
    pub fn at_cmd_char(&self) -> AT_CMD_CHAR_R {
        AT_CMD_CHAR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register is used to configure the num of continous at_cmd chars received by receiver."]
    #[inline(always)]
    pub fn char_num(&self) -> CHAR_NUM_R {
        CHAR_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the content of at_cmd char."]
    #[inline(always)]
    pub fn at_cmd_char(&mut self) -> AT_CMD_CHAR_W {
        AT_CMD_CHAR_W { w: self }
    }
    #[doc = "Bits 8:15 - This register is used to configure the num of continous at_cmd chars received by receiver."]
    #[inline(always)]
    pub fn char_num(&mut self) -> CHAR_NUM_W {
        CHAR_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [at_cmd_char](index.html) module"]
pub struct AT_CMD_CHAR_SPEC;
impl crate::RegisterSpec for AT_CMD_CHAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [at_cmd_char::R](R) reader structure"]
impl crate::Readable for AT_CMD_CHAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [at_cmd_char::W](W) writer structure"]
impl crate::Writable for AT_CMD_CHAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AT_CMD_CHAR to value 0x032b"]
impl crate::Resettable for AT_CMD_CHAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x032b
    }
}
