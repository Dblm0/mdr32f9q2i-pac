#[doc = "Register `ADC1_STATUS` reader"]
pub struct R(crate::R<ADC1_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1_STATUS` writer"]
pub struct W(crate::W<ADC1_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1_STATUS_SPEC>;
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
impl From<crate::W<ADC1_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Flg_REG_OVERWRITE` reader - "]
pub type FLG_REG_OVERWRITE_R = crate::BitReader<bool>;
#[doc = "Field `Flg_REG_OVERWRITE` writer - "]
pub type FLG_REG_OVERWRITE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC1_STATUS_SPEC, bool, O>;
#[doc = "Field `Flg_REG_AWOIFEN` reader - "]
pub type FLG_REG_AWOIFEN_R = crate::BitReader<bool>;
#[doc = "Field `Flg_REG_AWOIFEN` writer - "]
pub type FLG_REG_AWOIFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC1_STATUS_SPEC, bool, O>;
#[doc = "Field `Flg_REG_EOCIF` reader - "]
pub type FLG_REG_EOCIF_R = crate::BitReader<bool>;
#[doc = "Field `Flg_REG_EOCIF` writer - "]
pub type FLG_REG_EOCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC1_STATUS_SPEC, bool, O>;
#[doc = "Field `AWOIF_IE` reader - "]
pub type AWOIF_IE_R = crate::BitReader<bool>;
#[doc = "Field `AWOIF_IE` writer - "]
pub type AWOIF_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC1_STATUS_SPEC, bool, O>;
#[doc = "Field `ECOIF_IE` reader - "]
pub type ECOIF_IE_R = crate::BitReader<bool>;
#[doc = "Field `ECOIF_IE` writer - "]
pub type ECOIF_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC1_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flg_reg_overwrite(&self) -> FLG_REG_OVERWRITE_R {
        FLG_REG_OVERWRITE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn flg_reg_awoifen(&self) -> FLG_REG_AWOIFEN_R {
        FLG_REG_AWOIFEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn flg_reg_eocif(&self) -> FLG_REG_EOCIF_R {
        FLG_REG_EOCIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn awoif_ie(&self) -> AWOIF_IE_R {
        AWOIF_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ecoif_ie(&self) -> ECOIF_IE_R {
        ECOIF_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn flg_reg_overwrite(&mut self) -> FLG_REG_OVERWRITE_W<0> {
        FLG_REG_OVERWRITE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn flg_reg_awoifen(&mut self) -> FLG_REG_AWOIFEN_W<1> {
        FLG_REG_AWOIFEN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn flg_reg_eocif(&mut self) -> FLG_REG_EOCIF_W<2> {
        FLG_REG_EOCIF_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn awoif_ie(&mut self) -> AWOIF_IE_W<3> {
        AWOIF_IE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ecoif_ie(&mut self) -> ECOIF_IE_W<4> {
        ECOIF_IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1_status](index.html) module"]
pub struct ADC1_STATUS_SPEC;
impl crate::RegisterSpec for ADC1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1_status::R](R) reader structure"]
impl crate::Readable for ADC1_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1_status::W](W) writer structure"]
impl crate::Writable for ADC1_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1_STATUS to value 0"]
impl crate::Resettable for ADC1_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
