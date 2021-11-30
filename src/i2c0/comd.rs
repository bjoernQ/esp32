#[doc = "Register `COMD%s` reader"]
pub struct R(crate::R<COMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMD%s` writer"]
pub struct W(crate::W<COMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMD_SPEC>;
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
impl From<crate::W<COMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND` reader - This is the content of command. It consists of three part. op_code is the command : RSTART 1: WRITE 2: READ 3: STOP . 4:END. Byte_num represent the number of data need to be send or data need to be received. ack_check_en ack_exp and ack value are used to control the ack bit."]
pub struct COMMAND_R(crate::FieldReader<u16, u16>);
impl COMMAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COMMAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND` writer - This is the content of command. It consists of three part. op_code is the command : RSTART 1: WRITE 2: READ 3: STOP . 4:END. Byte_num represent the number of data need to be send or data need to be received. ack_check_en ack_exp and ack value are used to control the ack bit."]
pub struct COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `COMMAND_DONE` reader - When command is done in I2C Master mode this bit changes to high level."]
pub struct COMMAND_DONE_R(crate::FieldReader<bool, bool>);
impl COMMAND_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND_DONE` writer - When command is done in I2C Master mode this bit changes to high level."]
pub struct COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - This is the content of command. It consists of three part. op_code is the command : RSTART 1: WRITE 2: READ 3: STOP . 4:END. Byte_num represent the number of data need to be send or data need to be received. ack_check_en ack_exp and ack value are used to control the ack bit."]
    #[inline(always)]
    pub fn command(&self) -> COMMAND_R {
        COMMAND_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - When command is done in I2C Master mode this bit changes to high level."]
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - This is the content of command. It consists of three part. op_code is the command : RSTART 1: WRITE 2: READ 3: STOP . 4:END. Byte_num represent the number of data need to be send or data need to be received. ack_check_en ack_exp and ack value are used to control the ack bit."]
    #[inline(always)]
    pub fn command(&mut self) -> COMMAND_W {
        COMMAND_W { w: self }
    }
    #[doc = "Bit 31 - When command is done in I2C Master mode this bit changes to high level."]
    #[inline(always)]
    pub fn command_done(&mut self) -> COMMAND_DONE_W {
        COMMAND_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd](index.html) module"]
pub struct COMD_SPEC;
impl crate::RegisterSpec for COMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comd::R](R) reader structure"]
impl crate::Readable for COMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comd::W](W) writer structure"]
impl crate::Writable for COMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMD%s to value 0"]
impl crate::Resettable for COMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
