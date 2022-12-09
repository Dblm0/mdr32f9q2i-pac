#[doc = "Register `CH1_CNTRL2` reader"]
pub struct R(crate::R<CH1_CNTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1_CNTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1_CNTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1_CNTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1_CNTRL2` writer"]
pub struct W(crate::W<CH1_CNTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1_CNTRL2_SPEC>;
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
impl From<crate::W<CH1_CNTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1_CNTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSEL1` reader - "]
pub type CHSEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHSEL1` writer - "]
pub type CHSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1_CNTRL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `CCR1_EN` reader - "]
pub type CCR1_EN_R = crate::BitReader<bool>;
#[doc = "Field `CCR1_EN` writer - "]
pub type CCR1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1_CNTRL2_SPEC, bool, O>;
#[doc = "Field `CCRRLD` reader - "]
pub type CCRRLD_R = crate::BitReader<bool>;
#[doc = "Field `CCRRLD` writer - "]
pub type CCRRLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1_CNTRL2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ccr1_en(&self) -> CCR1_EN_R {
        CCR1_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ccrrld(&self) -> CCRRLD_R {
        CCRRLD_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn chsel1(&mut self) -> CHSEL1_W<0> {
        CHSEL1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ccr1_en(&mut self) -> CCR1_EN_W<2> {
        CCR1_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ccrrld(&mut self) -> CCRRLD_W<3> {
        CCRRLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Control2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_cntrl2](index.html) module"]
pub struct CH1_CNTRL2_SPEC;
impl crate::RegisterSpec for CH1_CNTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1_cntrl2::R](R) reader structure"]
impl crate::Readable for CH1_CNTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1_cntrl2::W](W) writer structure"]
impl crate::Writable for CH1_CNTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1_CNTRL2 to value 0"]
impl crate::Resettable for CH1_CNTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
