#[doc = "Register `HOST_SLCHOST_GPIO_STATUS1` reader"]
pub struct R(crate::R<HOST_SLCHOST_GPIO_STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_GPIO_STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_GPIO_STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_GPIO_STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOST_GPIO_SDIO_INT1` reader - "]
pub struct HOST_GPIO_SDIO_INT1_R(crate::FieldReader<u8, u8>);
impl HOST_GPIO_SDIO_INT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOST_GPIO_SDIO_INT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_GPIO_SDIO_INT1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_gpio_sdio_int1(&self) -> HOST_GPIO_SDIO_INT1_R {
        HOST_GPIO_SDIO_INT1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_gpio_status1](index.html) module"]
pub struct HOST_SLCHOST_GPIO_STATUS1_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_GPIO_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_gpio_status1::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_GPIO_STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOST_SLCHOST_GPIO_STATUS1 to value 0"]
impl crate::Resettable for HOST_SLCHOST_GPIO_STATUS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}