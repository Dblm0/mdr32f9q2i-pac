#[doc = "Register `ADC_MCO_CLOCK` reader"]
pub struct R(crate::R<ADC_MCO_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_MCO_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_MCO_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_MCO_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_MCO_CLOCK` writer"]
pub struct W(crate::W<ADC_MCO_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_MCO_CLOCK_SPEC>;
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
impl From<crate::W<ADC_MCO_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_MCO_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_C1_SEL` reader - "]
pub type ADC_C1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_C1_SEL` writer - "]
pub type ADC_C1_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC_MCO_CLOCK_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADC_C2_SEL` reader - "]
pub type ADC_C2_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_C2_SEL` writer - "]
pub type ADC_C2_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC_MCO_CLOCK_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADC_C3_SEL` reader - "]
pub type ADC_C3_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_C3_SEL` writer - "]
pub type ADC_C3_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC_MCO_CLOCK_SPEC, u8, u8, 4, O>;
#[doc = "Field `MCO_EN` reader - "]
pub type MCO_EN_R = crate::BitReader<bool>;
#[doc = "Field `MCO_EN` writer - "]
pub type MCO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_MCO_CLOCK_SPEC, bool, O>;
#[doc = "Field `ADC_CLK_EN` reader - "]
pub type ADC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC_CLK_EN` writer - "]
pub type ADC_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_MCO_CLOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adc_c1_sel(&self) -> ADC_C1_SEL_R {
        ADC_C1_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc_c2_sel(&self) -> ADC_C2_SEL_R {
        ADC_C2_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn adc_c3_sel(&self) -> ADC_C3_SEL_R {
        ADC_C3_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mco_en(&self) -> MCO_EN_R {
        MCO_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_clk_en(&self) -> ADC_CLK_EN_R {
        ADC_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn adc_c1_sel(&mut self) -> ADC_C1_SEL_W<0> {
        ADC_C1_SEL_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn adc_c2_sel(&mut self) -> ADC_C2_SEL_W<4> {
        ADC_C2_SEL_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn adc_c3_sel(&mut self) -> ADC_C3_SEL_W<8> {
        ADC_C3_SEL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn mco_en(&mut self) -> MCO_EN_W<12> {
        MCO_EN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn adc_clk_en(&mut self) -> ADC_CLK_EN_W<13> {
        ADC_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_mco_clock](index.html) module"]
pub struct ADC_MCO_CLOCK_SPEC;
impl crate::RegisterSpec for ADC_MCO_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_mco_clock::R](R) reader structure"]
impl crate::Readable for ADC_MCO_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_mco_clock::W](W) writer structure"]
impl crate::Writable for ADC_MCO_CLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_MCO_CLOCK to value 0"]
impl crate::Resettable for ADC_MCO_CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
