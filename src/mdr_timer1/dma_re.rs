#[doc = "Register `DMA_RE` reader"]
pub struct R(crate::R<DMA_RE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_RE` writer"]
pub struct W(crate::W<DMA_RE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_RE_SPEC>;
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
impl From<crate::W<DMA_RE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_RE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_ZERO_EVENT_RE` reader - "]
pub type CNT_ZERO_EVENT_RE_R = crate::BitReader<bool>;
#[doc = "Field `CNT_ZERO_EVENT_RE` writer - "]
pub type CNT_ZERO_EVENT_RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_RE_SPEC, bool, O>;
#[doc = "Field `CNT_ARR_EVENT_RE` reader - "]
pub type CNT_ARR_EVENT_RE_R = crate::BitReader<bool>;
#[doc = "Field `CNT_ARR_EVENT_RE` writer - "]
pub type CNT_ARR_EVENT_RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_RE_SPEC, bool, O>;
#[doc = "Field `ETR_RE_EVENT_RE` reader - "]
pub type ETR_RE_EVENT_RE_R = crate::BitReader<bool>;
#[doc = "Field `ETR_RE_EVENT_RE` writer - "]
pub type ETR_RE_EVENT_RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_RE_SPEC, bool, O>;
#[doc = "Field `ETR_FE_EVENT_RE` reader - "]
pub type ETR_FE_EVENT_RE_R = crate::BitReader<bool>;
#[doc = "Field `ETR_FE_EVENT_RE` writer - "]
pub type ETR_FE_EVENT_RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_RE_SPEC, bool, O>;
#[doc = "Field `BRK_EVENT_RE` reader - "]
pub type BRK_EVENT_RE_R = crate::BitReader<bool>;
#[doc = "Field `BRK_EVENT_RE` writer - "]
pub type BRK_EVENT_RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_RE_SPEC, bool, O>;
#[doc = "Field `CCR_CAP_EVENT_RE` reader - "]
pub type CCR_CAP_EVENT_RE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCR_CAP_EVENT_RE` writer - "]
pub type CCR_CAP_EVENT_RE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_RE_SPEC, u8, u8, 4, O>;
#[doc = "Field `CCR_REF_EVENT_RE` reader - "]
pub type CCR_REF_EVENT_RE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCR_REF_EVENT_RE` writer - "]
pub type CCR_REF_EVENT_RE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_RE_SPEC, u8, u8, 4, O>;
#[doc = "Field `CCR1_CAP_EVENT_RE` reader - "]
pub type CCR1_CAP_EVENT_RE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCR1_CAP_EVENT_RE` writer - "]
pub type CCR1_CAP_EVENT_RE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_RE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnt_zero_event_re(&self) -> CNT_ZERO_EVENT_RE_R {
        CNT_ZERO_EVENT_RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnt_arr_event_re(&self) -> CNT_ARR_EVENT_RE_R {
        CNT_ARR_EVENT_RE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn etr_re_event_re(&self) -> ETR_RE_EVENT_RE_R {
        ETR_RE_EVENT_RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn etr_fe_event_re(&self) -> ETR_FE_EVENT_RE_R {
        ETR_FE_EVENT_RE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn brk_event_re(&self) -> BRK_EVENT_RE_R {
        BRK_EVENT_RE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8"]
    #[inline(always)]
    pub fn ccr_cap_event_re(&self) -> CCR_CAP_EVENT_RE_R {
        CCR_CAP_EVENT_RE_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn ccr_ref_event_re(&self) -> CCR_REF_EVENT_RE_R {
        CCR_REF_EVENT_RE_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16"]
    #[inline(always)]
    pub fn ccr1_cap_event_re(&self) -> CCR1_CAP_EVENT_RE_R {
        CCR1_CAP_EVENT_RE_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_zero_event_re(&mut self) -> CNT_ZERO_EVENT_RE_W<0> {
        CNT_ZERO_EVENT_RE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_arr_event_re(&mut self) -> CNT_ARR_EVENT_RE_W<1> {
        CNT_ARR_EVENT_RE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn etr_re_event_re(&mut self) -> ETR_RE_EVENT_RE_W<2> {
        ETR_RE_EVENT_RE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn etr_fe_event_re(&mut self) -> ETR_FE_EVENT_RE_W<3> {
        ETR_FE_EVENT_RE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn brk_event_re(&mut self) -> BRK_EVENT_RE_W<4> {
        BRK_EVENT_RE_W::new(self)
    }
    #[doc = "Bits 5:8"]
    #[inline(always)]
    #[must_use]
    pub fn ccr_cap_event_re(&mut self) -> CCR_CAP_EVENT_RE_W<5> {
        CCR_CAP_EVENT_RE_W::new(self)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    #[must_use]
    pub fn ccr_ref_event_re(&mut self) -> CCR_REF_EVENT_RE_W<9> {
        CCR_REF_EVENT_RE_W::new(self)
    }
    #[doc = "Bits 13:16"]
    #[inline(always)]
    #[must_use]
    pub fn ccr1_cap_event_re(&mut self) -> CCR1_CAP_EVENT_RE_W<13> {
        CCR1_CAP_EVENT_RE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer DMA Request Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_re](index.html) module"]
pub struct DMA_RE_SPEC;
impl crate::RegisterSpec for DMA_RE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_re::R](R) reader structure"]
impl crate::Readable for DMA_RE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_re::W](W) writer structure"]
impl crate::Writable for DMA_RE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_RE to value 0"]
impl crate::Resettable for DMA_RE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
