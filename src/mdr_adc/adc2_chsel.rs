#[doc = "Register `ADC2_CHSEL` reader"]
pub struct R(crate::R<ADC2_CHSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC2_CHSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC2_CHSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC2_CHSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC2_CHSEL` writer"]
pub struct W(crate::W<ADC2_CHSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC2_CHSEL_SPEC>;
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
impl From<crate::W<ADC2_CHSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC2_CHSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Sl_Ch_Ch_REF` reader - "]
pub type SL_CH_CH_REF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `Sl_Ch_Ch_REF` writer - "]
pub type SL_CH_CH_REF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC2_CHSEL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sl_ch_ch_ref(&self) -> SL_CH_CH_REF_R {
        SL_CH_CH_REF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn sl_ch_ch_ref(&mut self) -> SL_CH_CH_REF_W<0> {
        SL_CH_CH_REF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC2 Channel Selector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc2_chsel](index.html) module"]
pub struct ADC2_CHSEL_SPEC;
impl crate::RegisterSpec for ADC2_CHSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc2_chsel::R](R) reader structure"]
impl crate::Readable for ADC2_CHSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc2_chsel::W](W) writer structure"]
impl crate::Writable for ADC2_CHSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC2_CHSEL to value 0"]
impl crate::Resettable for ADC2_CHSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
