#[doc = "Register `ADC1_RESULT` reader"]
pub struct R(crate::R<ADC1_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1_RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1_RESULT` writer"]
pub struct W(crate::W<ADC1_RESULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1_RESULT_SPEC>;
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
impl From<crate::W<ADC1_RESULT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1_RESULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESULT` reader - "]
pub type RESULT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESULT` writer - "]
pub type RESULT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC1_RESULT_SPEC, u16, u16, 12, O>;
#[doc = "Field `CHANNEL` reader - "]
pub type CHANNEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CHANNEL` writer - "]
pub type CHANNEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC1_RESULT_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn result(&mut self) -> RESULT_W<0> {
        RESULT_W::new(self)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn channel(&mut self) -> CHANNEL_W<16> {
        CHANNEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1 Result Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1_result](index.html) module"]
pub struct ADC1_RESULT_SPEC;
impl crate::RegisterSpec for ADC1_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1_result::R](R) reader structure"]
impl crate::Readable for ADC1_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1_result::W](W) writer structure"]
impl crate::Writable for ADC1_RESULT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1_RESULT to value 0"]
impl crate::Resettable for ADC1_RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
