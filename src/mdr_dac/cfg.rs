#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Cfg_M_REF0` reader - "]
pub type CFG_M_REF0_R = crate::BitReader<bool>;
#[doc = "Field `Cfg_M_REF0` writer - "]
pub type CFG_M_REF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `Cfg_M_REF1` reader - "]
pub type CFG_M_REF1_R = crate::BitReader<bool>;
#[doc = "Field `Cfg_M_REF1` writer - "]
pub type CFG_M_REF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `Cfg_ON_DAC0` reader - "]
pub type CFG_ON_DAC0_R = crate::BitReader<bool>;
#[doc = "Field `Cfg_ON_DAC0` writer - "]
pub type CFG_ON_DAC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `Cfg_ON_DAC1` reader - "]
pub type CFG_ON_DAC1_R = crate::BitReader<bool>;
#[doc = "Field `Cfg_ON_DAC1` writer - "]
pub type CFG_ON_DAC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `Cfg_SYNC_A` reader - "]
pub type CFG_SYNC_A_R = crate::BitReader<bool>;
#[doc = "Field `Cfg_SYNC_A` writer - "]
pub type CFG_SYNC_A_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cfg_m_ref0(&self) -> CFG_M_REF0_R {
        CFG_M_REF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cfg_m_ref1(&self) -> CFG_M_REF1_R {
        CFG_M_REF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cfg_on_dac0(&self) -> CFG_ON_DAC0_R {
        CFG_ON_DAC0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cfg_on_dac1(&self) -> CFG_ON_DAC1_R {
        CFG_ON_DAC1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cfg_sync_a(&self) -> CFG_SYNC_A_R {
        CFG_SYNC_A_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_m_ref0(&mut self) -> CFG_M_REF0_W<0> {
        CFG_M_REF0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_m_ref1(&mut self) -> CFG_M_REF1_W<1> {
        CFG_M_REF1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_on_dac0(&mut self) -> CFG_ON_DAC0_W<2> {
        CFG_ON_DAC0_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_on_dac1(&mut self) -> CFG_ON_DAC1_W<3> {
        CFG_ON_DAC1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_sync_a(&mut self) -> CFG_SYNC_A_W<4> {
        CFG_SYNC_A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
