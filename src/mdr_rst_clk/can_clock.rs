#[doc = "Register `CAN_CLOCK` reader"]
pub struct R(crate::R<CAN_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_CLOCK` writer"]
pub struct W(crate::W<CAN_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_CLOCK_SPEC>;
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
impl From<crate::W<CAN_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN1_BRG` reader - "]
pub type CAN1_BRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN1_BRG` writer - "]
pub type CAN1_BRG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAN_CLOCK_SPEC, u8, u8, 8, O>;
#[doc = "Field `CAN2_BRG` reader - "]
pub type CAN2_BRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN2_BRG` writer - "]
pub type CAN2_BRG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAN_CLOCK_SPEC, u8, u8, 8, O>;
#[doc = "Field `CAN1_CLK_EN` reader - "]
pub type CAN1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CAN1_CLK_EN` writer - "]
pub type CAN1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAN_CLOCK_SPEC, bool, O>;
#[doc = "Field `CAN2_CLK_EN` reader - "]
pub type CAN2_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CAN2_CLK_EN` writer - "]
pub type CAN2_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAN_CLOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn can1_brg(&self) -> CAN1_BRG_R {
        CAN1_BRG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn can2_brg(&self) -> CAN2_BRG_R {
        CAN2_BRG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn can1_clk_en(&self) -> CAN1_CLK_EN_R {
        CAN1_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn can2_clk_en(&self) -> CAN2_CLK_EN_R {
        CAN2_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn can1_brg(&mut self) -> CAN1_BRG_W<0> {
        CAN1_BRG_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn can2_brg(&mut self) -> CAN2_BRG_W<8> {
        CAN2_BRG_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn can1_clk_en(&mut self) -> CAN1_CLK_EN_W<24> {
        CAN1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn can2_clk_en(&mut self) -> CAN2_CLK_EN_W<25> {
        CAN2_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_clock](index.html) module"]
pub struct CAN_CLOCK_SPEC;
impl crate::RegisterSpec for CAN_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_clock::R](R) reader structure"]
impl crate::Readable for CAN_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_clock::W](W) writer structure"]
impl crate::Writable for CAN_CLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAN_CLOCK to value 0"]
impl crate::Resettable for CAN_CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
