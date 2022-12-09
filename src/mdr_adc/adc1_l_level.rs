#[doc = "Register `ADC1_L_LEVEL` reader"]
pub struct R(crate::R<ADC1_L_LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1_L_LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1_L_LEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1_L_LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1_L_LEVEL` writer"]
pub struct W(crate::W<ADC1_L_LEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1_L_LEVEL_SPEC>;
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
impl From<crate::W<ADC1_L_LEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1_L_LEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_L_LEVEL` reader - "]
pub type REG_L_LEVEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REG_L_LEVEL` writer - "]
pub type REG_L_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC1_L_LEVEL_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn reg_l_level(&self) -> REG_L_LEVEL_R {
        REG_L_LEVEL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l_level(&mut self) -> REG_L_LEVEL_W<0> {
        REG_L_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1 Low Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1_l_level](index.html) module"]
pub struct ADC1_L_LEVEL_SPEC;
impl crate::RegisterSpec for ADC1_L_LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1_l_level::R](R) reader structure"]
impl crate::Readable for ADC1_L_LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1_l_level::W](W) writer structure"]
impl crate::Writable for ADC1_L_LEVEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1_L_LEVEL to value 0"]
impl crate::Resettable for ADC1_L_LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
