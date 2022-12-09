#[doc = "Register `CFR` reader"]
pub struct R(crate::R<CFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFR` writer"]
pub struct W(crate::W<CFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFR_SPEC>;
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
impl From<crate::W<CFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `W` reader - "]
pub type W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `W` writer - "]
pub type W_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFR_SPEC, u8, u8, 7, O>;
#[doc = "Field `WGTB` reader - "]
pub type WGTB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WGTB` writer - "]
pub type WGTB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFR_SPEC, u8, u8, 2, O>;
#[doc = "Field `EWI` reader - "]
pub type EWI_R = crate::BitReader<bool>;
#[doc = "Field `EWI` writer - "]
pub type EWI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn wgtb(&self) -> WGTB_R {
        WGTB_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn w(&mut self) -> W_W<0> {
        W_W::new(self)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    #[must_use]
    pub fn wgtb(&mut self) -> WGTB_W<7> {
        WGTB_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ewi(&mut self) -> EWI_W<9> {
        EWI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WWDG Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfr](index.html) module"]
pub struct CFR_SPEC;
impl crate::RegisterSpec for CFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfr::R](R) reader structure"]
impl crate::Readable for CFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfr::W](W) writer structure"]
impl crate::Writable for CFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFR to value 0"]
impl crate::Resettable for CFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
