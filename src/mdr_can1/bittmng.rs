#[doc = "Register `BITTMNG` reader"]
pub struct R(crate::R<BITTMNG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BITTMNG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BITTMNG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BITTMNG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BITTMNG` writer"]
pub struct W(crate::W<BITTMNG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BITTMNG_SPEC>;
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
impl From<crate::W<BITTMNG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BITTMNG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRP` reader - "]
pub type BRP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRP` writer - "]
pub type BRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BITTMNG_SPEC, u16, u16, 16, O>;
#[doc = "Field `PSEG` reader - "]
pub type PSEG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSEG` writer - "]
pub type PSEG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BITTMNG_SPEC, u8, u8, 3, O>;
#[doc = "Field `SEG1` reader - "]
pub type SEG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEG1` writer - "]
pub type SEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BITTMNG_SPEC, u8, u8, 3, O>;
#[doc = "Field `SEG2` reader - "]
pub type SEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEG2` writer - "]
pub type SEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BITTMNG_SPEC, u8, u8, 3, O>;
#[doc = "Field `SJW` reader - "]
pub type SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SJW` writer - "]
pub type SJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BITTMNG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SB` reader - "]
pub type SB_R = crate::BitReader<bool>;
#[doc = "Field `SB` writer - "]
pub type SB_W<'a, const O: u8> = crate::BitWriter<'a, u32, BITTMNG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn pseg(&self) -> PSEG_R {
        PSEG_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn seg1(&self) -> SEG1_R {
        SEG1_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn seg2(&self) -> SEG2_R {
        SEG2_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<0> {
        BRP_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn pseg(&mut self) -> PSEG_W<16> {
        PSEG_W::new(self)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    #[must_use]
    pub fn seg1(&mut self) -> SEG1_W<19> {
        SEG1_W::new(self)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    #[must_use]
    pub fn seg2(&mut self) -> SEG2_W<22> {
        SEG2_W::new(self)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<25> {
        SJW_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn sb(&mut self) -> SB_W<27> {
        SB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Bittiming Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bittmng](index.html) module"]
pub struct BITTMNG_SPEC;
impl crate::RegisterSpec for BITTMNG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bittmng::R](R) reader structure"]
impl crate::Readable for BITTMNG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bittmng::W](W) writer structure"]
impl crate::Writable for BITTMNG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BITTMNG to value 0"]
impl crate::Resettable for BITTMNG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
