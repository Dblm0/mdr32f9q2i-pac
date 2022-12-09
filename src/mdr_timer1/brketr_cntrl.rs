#[doc = "Register `BRKETR_CNTRL` reader"]
pub struct R(crate::R<BRKETR_CNTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRKETR_CNTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRKETR_CNTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRKETR_CNTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRKETR_CNTRL` writer"]
pub struct W(crate::W<BRKETR_CNTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRKETR_CNTRL_SPEC>;
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
impl From<crate::W<BRKETR_CNTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRKETR_CNTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRK_INV` reader - "]
pub type BRK_INV_R = crate::BitReader<bool>;
#[doc = "Field `BRK_INV` writer - "]
pub type BRK_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKETR_CNTRL_SPEC, bool, O>;
#[doc = "Field `ETR_INV` reader - "]
pub type ETR_INV_R = crate::BitReader<bool>;
#[doc = "Field `ETR_INV` writer - "]
pub type ETR_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRKETR_CNTRL_SPEC, bool, O>;
#[doc = "Field `ETR_PSC` reader - "]
pub type ETR_PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETR_PSC` writer - "]
pub type ETR_PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRKETR_CNTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `ETR_FILTER` reader - "]
pub type ETR_FILTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETR_FILTER` writer - "]
pub type ETR_FILTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BRKETR_CNTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn brk_inv(&self) -> BRK_INV_R {
        BRK_INV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn etr_inv(&self) -> ETR_INV_R {
        ETR_INV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn etr_psc(&self) -> ETR_PSC_R {
        ETR_PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn etr_filter(&self) -> ETR_FILTER_R {
        ETR_FILTER_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn brk_inv(&mut self) -> BRK_INV_W<0> {
        BRK_INV_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn etr_inv(&mut self) -> ETR_INV_W<1> {
        ETR_INV_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn etr_psc(&mut self) -> ETR_PSC_W<2> {
        ETR_PSC_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn etr_filter(&mut self) -> ETR_FILTER_W<4> {
        ETR_FILTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer BRK/ETR Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brketr_cntrl](index.html) module"]
pub struct BRKETR_CNTRL_SPEC;
impl crate::RegisterSpec for BRKETR_CNTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brketr_cntrl::R](R) reader structure"]
impl crate::Readable for BRKETR_CNTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brketr_cntrl::W](W) writer structure"]
impl crate::Writable for BRKETR_CNTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRKETR_CNTRL to value 0"]
impl crate::Resettable for BRKETR_CNTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
