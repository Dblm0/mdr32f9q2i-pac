#[doc = "Register `CH1_DTG` reader"]
pub struct R(crate::R<CH1_DTG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1_DTG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1_DTG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1_DTG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1_DTG` writer"]
pub struct W(crate::W<CH1_DTG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1_DTG_SPEC>;
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
impl From<crate::W<CH1_DTG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1_DTG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTGx` reader - "]
pub type DTGX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTGx` writer - "]
pub type DTGX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1_DTG_SPEC, u8, u8, 4, O>;
#[doc = "Field `EDTS` reader - "]
pub type EDTS_R = crate::BitReader<bool>;
#[doc = "Field `EDTS` writer - "]
pub type EDTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1_DTG_SPEC, bool, O>;
#[doc = "Field `DTG` reader - "]
pub type DTG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTG` writer - "]
pub type DTG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1_DTG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dtgx(&self) -> DTGX_R {
        DTGX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn edts(&self) -> EDTS_R {
        EDTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn dtgx(&mut self) -> DTGX_W<0> {
        DTGX_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn edts(&mut self) -> EDTS_W<4> {
        EDTS_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn dtg(&mut self) -> DTG_W<8> {
        DTG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel DTG Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_dtg](index.html) module"]
pub struct CH1_DTG_SPEC;
impl crate::RegisterSpec for CH1_DTG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1_dtg::R](R) reader structure"]
impl crate::Readable for CH1_DTG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1_dtg::W](W) writer structure"]
impl crate::Writable for CH1_DTG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1_DTG to value 0"]
impl crate::Resettable for CH1_DTG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
