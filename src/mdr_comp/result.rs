#[doc = "Register `RESULT` reader"]
pub struct R(crate::R<RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESULT` writer"]
pub struct W(crate::W<RESULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESULT_SPEC>;
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
impl From<crate::W<RESULT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Rslt_Sy` reader - "]
pub type RSLT_SY_R = crate::BitReader<bool>;
#[doc = "Field `Rslt_Sy` writer - "]
pub type RSLT_SY_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESULT_SPEC, bool, O>;
#[doc = "Field `Rslt_As` reader - "]
pub type RSLT_AS_R = crate::BitReader<bool>;
#[doc = "Field `Rslt_As` writer - "]
pub type RSLT_AS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESULT_SPEC, bool, O>;
#[doc = "Field `Rst_lch` reader - "]
pub type RST_LCH_R = crate::BitReader<bool>;
#[doc = "Field `Rst_lch` writer - "]
pub type RST_LCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESULT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rslt_sy(&self) -> RSLT_SY_R {
        RSLT_SY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rslt_as(&self) -> RSLT_AS_R {
        RSLT_AS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rst_lch(&self) -> RST_LCH_R {
        RST_LCH_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rslt_sy(&mut self) -> RSLT_SY_W<0> {
        RSLT_SY_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rslt_as(&mut self) -> RSLT_AS_W<1> {
        RSLT_AS_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rst_lch(&mut self) -> RST_LCH_W<2> {
        RST_LCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP Result Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](index.html) module"]
pub struct RESULT_SPEC;
impl crate::RegisterSpec for RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result::R](R) reader structure"]
impl crate::Readable for RESULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [result::W](W) writer structure"]
impl crate::Writable for RESULT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
