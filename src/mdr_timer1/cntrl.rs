#[doc = "Register `CNTRL` reader"]
pub struct R(crate::R<CNTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTRL` writer"]
pub struct W(crate::W<CNTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTRL_SPEC>;
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
impl From<crate::W<CNTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_EN` reader - "]
pub type CNT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CNT_EN` writer - "]
pub type CNT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTRL_SPEC, bool, O>;
#[doc = "Field `ARRB_EN` reader - "]
pub type ARRB_EN_R = crate::BitReader<bool>;
#[doc = "Field `ARRB_EN` writer - "]
pub type ARRB_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTRL_SPEC, bool, O>;
#[doc = "Field `WR_CMPL` reader - "]
pub type WR_CMPL_R = crate::BitReader<bool>;
#[doc = "Field `WR_CMPL` writer - "]
pub type WR_CMPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTRL_SPEC, bool, O>;
#[doc = "Field `DIR` reader - "]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - "]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTRL_SPEC, bool, O>;
#[doc = "Field `FDTS` reader - "]
pub type FDTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FDTS` writer - "]
pub type FDTS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CNT_MODE` reader - "]
pub type CNT_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNT_MODE` writer - "]
pub type CNT_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `EVENT_SEL` reader - "]
pub type EVENT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EVENT_SEL` writer - "]
pub type EVENT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnt_en(&self) -> CNT_EN_R {
        CNT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn arrb_en(&self) -> ARRB_EN_R {
        ARRB_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wr_cmpl(&self) -> WR_CMPL_R {
        WR_CMPL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fdts(&self) -> FDTS_R {
        FDTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cnt_mode(&self) -> CNT_MODE_R {
        CNT_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn event_sel(&self) -> EVENT_SEL_R {
        EVENT_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_en(&mut self) -> CNT_EN_W<0> {
        CNT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn arrb_en(&mut self) -> ARRB_EN_W<1> {
        ARRB_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn wr_cmpl(&mut self) -> WR_CMPL_W<2> {
        WR_CMPL_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<3> {
        DIR_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn fdts(&mut self) -> FDTS_W<4> {
        FDTS_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_mode(&mut self) -> CNT_MODE_W<6> {
        CNT_MODE_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn event_sel(&mut self) -> EVENT_SEL_W<8> {
        EVENT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntrl](index.html) module"]
pub struct CNTRL_SPEC;
impl crate::RegisterSpec for CNTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntrl::R](R) reader structure"]
impl crate::Readable for CNTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntrl::W](W) writer structure"]
impl crate::Writable for CNTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTRL to value 0"]
impl crate::Resettable for CNTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
