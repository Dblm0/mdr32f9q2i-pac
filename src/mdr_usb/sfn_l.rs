#[doc = "Register `SFN_L` reader"]
pub struct R(crate::R<SFN_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFN_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFN_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFN_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFN_L` writer"]
pub struct W(crate::W<SFN_L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFN_L_SPEC>;
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
impl From<crate::W<SFN_L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFN_L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMENUM` reader - "]
pub type FRAMENUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAMENUM` writer - "]
pub type FRAMENUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SFN_L_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn framenum(&self) -> FRAMENUM_R {
        FRAMENUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn framenum(&mut self) -> FRAMENUM_W<0> {
        FRAMENUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfn_l](index.html) module"]
pub struct SFN_L_SPEC;
impl crate::RegisterSpec for SFN_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfn_l::R](R) reader structure"]
impl crate::Readable for SFN_L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfn_l::W](W) writer structure"]
impl crate::Writable for SFN_L_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFN_L to value 0"]
impl crate::Resettable for SFN_L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
