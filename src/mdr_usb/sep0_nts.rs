#[doc = "Register `SEP0_NTS` reader"]
pub struct R(crate::R<SEP0_NTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEP0_NTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEP0_NTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEP0_NTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEP0_NTS` writer"]
pub struct W(crate::W<SEP0_NTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEP0_NTS_SPEC>;
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
impl From<crate::W<SEP0_NTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEP0_NTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NTTYPE` reader - "]
pub type NTTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NTTYPE` writer - "]
pub type NTTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEP0_NTS_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn nttype(&self) -> NTTYPE_R {
        NTTYPE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn nttype(&mut self) -> NTTYPE_W<0> {
        NTTYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sep0_nts](index.html) module"]
pub struct SEP0_NTS_SPEC;
impl crate::RegisterSpec for SEP0_NTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sep0_nts::R](R) reader structure"]
impl crate::Readable for SEP0_NTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sep0_nts::W](W) writer structure"]
impl crate::Writable for SEP0_NTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEP0_NTS to value 0"]
impl crate::Resettable for SEP0_NTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
