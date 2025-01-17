#[doc = "Register `BLK0_RDATA5` reader"]
pub struct R(crate::R<BLK0_RDATA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_RDATA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_RDATA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_RDATA5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_SPI_PAD_CONFIG_CLK` reader - read for SPI_pad_config_clk"]
pub struct RD_SPI_PAD_CONFIG_CLK_R(crate::FieldReader<u8, u8>);
impl RD_SPI_PAD_CONFIG_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_SPI_PAD_CONFIG_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_SPI_PAD_CONFIG_CLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_SPI_PAD_CONFIG_Q` reader - read for SPI_pad_config_q"]
pub struct RD_SPI_PAD_CONFIG_Q_R(crate::FieldReader<u8, u8>);
impl RD_SPI_PAD_CONFIG_Q_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_SPI_PAD_CONFIG_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_SPI_PAD_CONFIG_Q_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_SPI_PAD_CONFIG_D` reader - read for SPI_pad_config_d"]
pub struct RD_SPI_PAD_CONFIG_D_R(crate::FieldReader<u8, u8>);
impl RD_SPI_PAD_CONFIG_D_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_SPI_PAD_CONFIG_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_SPI_PAD_CONFIG_D_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_SPI_PAD_CONFIG_CS0` reader - read for SPI_pad_config_cs0"]
pub struct RD_SPI_PAD_CONFIG_CS0_R(crate::FieldReader<u8, u8>);
impl RD_SPI_PAD_CONFIG_CS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_SPI_PAD_CONFIG_CS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_SPI_PAD_CONFIG_CS0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_INST_CONFIG` reader - "]
pub struct RD_INST_CONFIG_R(crate::FieldReader<u8, u8>);
impl RD_INST_CONFIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_INST_CONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_INST_CONFIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_VOL_LEVEL_HP_INV` reader - This field stores the voltage level for CPU to run at 240 MHz, or for flash/PSRAM to run at 80 MHz.0x0: level 7; 0x1: level 6; 0x2: level 5; 0x3: level 4. (RO)"]
pub struct RD_VOL_LEVEL_HP_INV_R(crate::FieldReader<u8, u8>);
impl RD_VOL_LEVEL_HP_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_VOL_LEVEL_HP_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_VOL_LEVEL_HP_INV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_DIG_VOL_L6` reader - "]
pub struct RD_DIG_VOL_L6_R(crate::FieldReader<u8, u8>);
impl RD_DIG_VOL_L6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_DIG_VOL_L6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_DIG_VOL_L6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_FLASH_CRYPT_CONFIG` reader - read for flash_crypt_config"]
pub struct RD_FLASH_CRYPT_CONFIG_R(crate::FieldReader<u8, u8>);
impl RD_FLASH_CRYPT_CONFIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_FLASH_CRYPT_CONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_FLASH_CRYPT_CONFIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - read for SPI_pad_config_clk"]
    #[inline(always)]
    pub fn rd_spi_pad_config_clk(&self) -> RD_SPI_PAD_CONFIG_CLK_R {
        RD_SPI_PAD_CONFIG_CLK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - read for SPI_pad_config_q"]
    #[inline(always)]
    pub fn rd_spi_pad_config_q(&self) -> RD_SPI_PAD_CONFIG_Q_R {
        RD_SPI_PAD_CONFIG_Q_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - read for SPI_pad_config_d"]
    #[inline(always)]
    pub fn rd_spi_pad_config_d(&self) -> RD_SPI_PAD_CONFIG_D_R {
        RD_SPI_PAD_CONFIG_D_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - read for SPI_pad_config_cs0"]
    #[inline(always)]
    pub fn rd_spi_pad_config_cs0(&self) -> RD_SPI_PAD_CONFIG_CS0_R {
        RD_SPI_PAD_CONFIG_CS0_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:27"]
    #[inline(always)]
    pub fn rd_inst_config(&self) -> RD_INST_CONFIG_R {
        RD_INST_CONFIG_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 22:23 - This field stores the voltage level for CPU to run at 240 MHz, or for flash/PSRAM to run at 80 MHz.0x0: level 7; 0x1: level 6; 0x2: level 5; 0x3: level 4. (RO)"]
    #[inline(always)]
    pub fn rd_vol_level_hp_inv(&self) -> RD_VOL_LEVEL_HP_INV_R {
        RD_VOL_LEVEL_HP_INV_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rd_dig_vol_l6(&self) -> RD_DIG_VOL_L6_R {
        RD_DIG_VOL_L6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - read for flash_crypt_config"]
    #[inline(always)]
    pub fn rd_flash_crypt_config(&self) -> RD_FLASH_CRYPT_CONFIG_R {
        RD_FLASH_CRYPT_CONFIG_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_rdata5](index.html) module"]
pub struct BLK0_RDATA5_SPEC;
impl crate::RegisterSpec for BLK0_RDATA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_rdata5::R](R) reader structure"]
impl crate::Readable for BLK0_RDATA5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK0_RDATA5 to value 0"]
impl crate::Resettable for BLK0_RDATA5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
