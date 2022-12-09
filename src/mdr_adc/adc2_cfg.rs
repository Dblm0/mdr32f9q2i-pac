#[doc = "Register `ADC2_CFG` reader"]
pub struct R(crate::R<ADC2_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC2_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC2_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC2_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC2_CFG` writer"]
pub struct W(crate::W<ADC2_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC2_CFG_SPEC>;
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
impl From<crate::W<ADC2_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC2_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Cfg_REG_ADON` reader - "]
pub type CFG_REG_ADON_R = crate::BitReader<bool>;
#[doc = "Field `Cfg_REG_ADON` writer - "]
pub type CFG_REG_ADON_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC2_CFG_SPEC, bool, O>;
#[doc = "Field `Cfg_REG_GO` reader - "]
pub type CFG_REG_GO_R = crate::BitReader<bool>;
#[doc = "Field `Cfg_REG_GO` writer - "]
pub type CFG_REG_GO_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC2_CFG_SPEC, bool, O>;
#[doc = "Field `Cfg_REG_CLKS` reader - "]
pub type CFG_REG_CLKS_R = crate::BitReader<bool>;
#[doc = "Field `Cfg_REG_CLKS` writer - "]
pub type CFG_REG_CLKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC2_CFG_SPEC, bool, O>;
#[doc = "Field `Cfg_REG_SAMPLE` reader - "]
pub type CFG_REG_SAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `Cfg_REG_SAMPLE` writer - "]
pub type CFG_REG_SAMPLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC2_CFG_SPEC, bool, O>;
#[doc = "Field `Cfg_REG_CHS` reader - "]
pub type CFG_REG_CHS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Cfg_REG_CHS` writer - "]
pub type CFG_REG_CHS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC2_CFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `Cfg_REG_CHCH` reader - "]
pub type CFG_REG_CHCH_R = crate::BitReader<bool>;
#[doc = "Field `Cfg_REG_CHCH` writer - "]
pub type CFG_REG_CHCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC2_CFG_SPEC, bool, O>;
#[doc = "Field `Cfg_REG_RNGC` reader - "]
pub type CFG_REG_RNGC_R = crate::BitReader<bool>;
#[doc = "Field `Cfg_REG_RNGC` writer - "]
pub type CFG_REG_RNGC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC2_CFG_SPEC, bool, O>;
#[doc = "Field `Cfg_M_REF` reader - "]
pub type CFG_M_REF_R = crate::BitReader<bool>;
#[doc = "Field `Cfg_M_REF` writer - "]
pub type CFG_M_REF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC2_CFG_SPEC, bool, O>;
#[doc = "Field `Cfg_REG_DIVCLK` reader - "]
pub type CFG_REG_DIVCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Cfg_REG_DIVCLK` writer - "]
pub type CFG_REG_DIVCLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC2_CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADC1_OP` reader - "]
pub type ADC1_OP_R = crate::BitReader<bool>;
#[doc = "Field `ADC1_OP` writer - "]
pub type ADC1_OP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC2_CFG_SPEC, bool, O>;
#[doc = "Field `ADC2_OP` reader - "]
pub type ADC2_OP_R = crate::BitReader<bool>;
#[doc = "Field `ADC2_OP` writer - "]
pub type ADC2_OP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC2_CFG_SPEC, bool, O>;
#[doc = "Field `Delay_Go` reader - "]
pub type DELAY_GO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Delay_Go` writer - "]
pub type DELAY_GO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADC2_CFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cfg_reg_adon(&self) -> CFG_REG_ADON_R {
        CFG_REG_ADON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cfg_reg_go(&self) -> CFG_REG_GO_R {
        CFG_REG_GO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cfg_reg_clks(&self) -> CFG_REG_CLKS_R {
        CFG_REG_CLKS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cfg_reg_sample(&self) -> CFG_REG_SAMPLE_R {
        CFG_REG_SAMPLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8"]
    #[inline(always)]
    pub fn cfg_reg_chs(&self) -> CFG_REG_CHS_R {
        CFG_REG_CHS_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cfg_reg_chch(&self) -> CFG_REG_CHCH_R {
        CFG_REG_CHCH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cfg_reg_rngc(&self) -> CFG_REG_RNGC_R {
        CFG_REG_RNGC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cfg_m_ref(&self) -> CFG_M_REF_R {
        CFG_M_REF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cfg_reg_divclk(&self) -> CFG_REG_DIVCLK_R {
        CFG_REG_DIVCLK_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn adc1_op(&self) -> ADC1_OP_R {
        ADC1_OP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adc2_op(&self) -> ADC2_OP_R {
        ADC2_OP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn delay_go(&self) -> DELAY_GO_R {
        DELAY_GO_R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_reg_adon(&mut self) -> CFG_REG_ADON_W<0> {
        CFG_REG_ADON_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_reg_go(&mut self) -> CFG_REG_GO_W<1> {
        CFG_REG_GO_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_reg_clks(&mut self) -> CFG_REG_CLKS_W<2> {
        CFG_REG_CLKS_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_reg_sample(&mut self) -> CFG_REG_SAMPLE_W<3> {
        CFG_REG_SAMPLE_W::new(self)
    }
    #[doc = "Bits 4:8"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_reg_chs(&mut self) -> CFG_REG_CHS_W<4> {
        CFG_REG_CHS_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_reg_chch(&mut self) -> CFG_REG_CHCH_W<9> {
        CFG_REG_CHCH_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_reg_rngc(&mut self) -> CFG_REG_RNGC_W<10> {
        CFG_REG_RNGC_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_m_ref(&mut self) -> CFG_M_REF_W<11> {
        CFG_M_REF_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_reg_divclk(&mut self) -> CFG_REG_DIVCLK_W<12> {
        CFG_REG_DIVCLK_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_op(&mut self) -> ADC1_OP_W<17> {
        ADC1_OP_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_op(&mut self) -> ADC2_OP_W<18> {
        ADC2_OP_W::new(self)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    #[must_use]
    pub fn delay_go(&mut self) -> DELAY_GO_W<25> {
        DELAY_GO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC2 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc2_cfg](index.html) module"]
pub struct ADC2_CFG_SPEC;
impl crate::RegisterSpec for ADC2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc2_cfg::R](R) reader structure"]
impl crate::Readable for ADC2_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc2_cfg::W](W) writer structure"]
impl crate::Writable for ADC2_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC2_CFG to value 0"]
impl crate::Resettable for ADC2_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
