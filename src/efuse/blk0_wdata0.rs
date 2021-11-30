#[doc = "Register `BLK0_WDATA0` reader"]
pub struct R(crate::R<BLK0_WDATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_WDATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_WDATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_WDATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK0_WDATA0` writer"]
pub struct W(crate::W<BLK0_WDATA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK0_WDATA0_SPEC>;
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
impl From<crate::W<BLK0_WDATA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK0_WDATA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR_DIS` reader - program for efuse_wr_disable"]
pub struct WR_DIS_R(crate::FieldReader<u16, u16>);
impl WR_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WR_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_DIS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_DIS` writer - program for efuse_wr_disable"]
pub struct WR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `RD_DIS` reader - program for efuse_rd_disable"]
pub struct RD_DIS_R(crate::FieldReader<u8, u8>);
impl RD_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_DIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_DIS` writer - program for efuse_rd_disable"]
pub struct RD_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `FLASH_CRYPT_CNT` reader - program for flash_crypt_cnt"]
pub struct FLASH_CRYPT_CNT_R(crate::FieldReader<u8, u8>);
impl FLASH_CRYPT_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLASH_CRYPT_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_CRYPT_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_CRYPT_CNT` writer - program for flash_crypt_cnt"]
pub struct FLASH_CRYPT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_CRYPT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 20)) | ((value as u32 & 0x7f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - program for efuse_wr_disable"]
    #[inline(always)]
    pub fn wr_dis(&self) -> WR_DIS_R {
        WR_DIS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - program for efuse_rd_disable"]
    #[inline(always)]
    pub fn rd_dis(&self) -> RD_DIS_R {
        RD_DIS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:26 - program for flash_crypt_cnt"]
    #[inline(always)]
    pub fn flash_crypt_cnt(&self) -> FLASH_CRYPT_CNT_R {
        FLASH_CRYPT_CNT_R::new(((self.bits >> 20) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - program for efuse_wr_disable"]
    #[inline(always)]
    pub fn wr_dis(&mut self) -> WR_DIS_W {
        WR_DIS_W { w: self }
    }
    #[doc = "Bits 16:19 - program for efuse_rd_disable"]
    #[inline(always)]
    pub fn rd_dis(&mut self) -> RD_DIS_W {
        RD_DIS_W { w: self }
    }
    #[doc = "Bits 20:26 - program for flash_crypt_cnt"]
    #[inline(always)]
    pub fn flash_crypt_cnt(&mut self) -> FLASH_CRYPT_CNT_W {
        FLASH_CRYPT_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_wdata0](index.html) module"]
pub struct BLK0_WDATA0_SPEC;
impl crate::RegisterSpec for BLK0_WDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_wdata0::R](R) reader structure"]
impl crate::Readable for BLK0_WDATA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk0_wdata0::W](W) writer structure"]
impl crate::Writable for BLK0_WDATA0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLK0_WDATA0 to value 0"]
impl crate::Resettable for BLK0_WDATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
