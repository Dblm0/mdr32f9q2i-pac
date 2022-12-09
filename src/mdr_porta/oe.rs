#[doc = "Register `OE` reader"]
pub struct R(crate::R<OE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OE` writer"]
pub struct W(crate::W<OE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OE_SPEC>;
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
impl From<crate::W<OE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OE_0` reader - "]
pub type OE_0_R = crate::BitReader<bool>;
#[doc = "Field `OE_0` writer - "]
pub type OE_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_1` reader - "]
pub type OE_1_R = crate::BitReader<bool>;
#[doc = "Field `OE_1` writer - "]
pub type OE_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_2` reader - "]
pub type OE_2_R = crate::BitReader<bool>;
#[doc = "Field `OE_2` writer - "]
pub type OE_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_3` reader - "]
pub type OE_3_R = crate::BitReader<bool>;
#[doc = "Field `OE_3` writer - "]
pub type OE_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_4` reader - "]
pub type OE_4_R = crate::BitReader<bool>;
#[doc = "Field `OE_4` writer - "]
pub type OE_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_5` reader - "]
pub type OE_5_R = crate::BitReader<bool>;
#[doc = "Field `OE_5` writer - "]
pub type OE_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_6` reader - "]
pub type OE_6_R = crate::BitReader<bool>;
#[doc = "Field `OE_6` writer - "]
pub type OE_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_7` reader - "]
pub type OE_7_R = crate::BitReader<bool>;
#[doc = "Field `OE_7` writer - "]
pub type OE_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_8` reader - "]
pub type OE_8_R = crate::BitReader<bool>;
#[doc = "Field `OE_8` writer - "]
pub type OE_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_9` reader - "]
pub type OE_9_R = crate::BitReader<bool>;
#[doc = "Field `OE_9` writer - "]
pub type OE_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_10` reader - "]
pub type OE_10_R = crate::BitReader<bool>;
#[doc = "Field `OE_10` writer - "]
pub type OE_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_11` reader - "]
pub type OE_11_R = crate::BitReader<bool>;
#[doc = "Field `OE_11` writer - "]
pub type OE_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_12` reader - "]
pub type OE_12_R = crate::BitReader<bool>;
#[doc = "Field `OE_12` writer - "]
pub type OE_12_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_13` reader - "]
pub type OE_13_R = crate::BitReader<bool>;
#[doc = "Field `OE_13` writer - "]
pub type OE_13_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_14` reader - "]
pub type OE_14_R = crate::BitReader<bool>;
#[doc = "Field `OE_14` writer - "]
pub type OE_14_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
#[doc = "Field `OE_15` reader - "]
pub type OE_15_R = crate::BitReader<bool>;
#[doc = "Field `OE_15` writer - "]
pub type OE_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, OE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn oe_0(&self) -> OE_0_R {
        OE_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn oe_1(&self) -> OE_1_R {
        OE_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn oe_2(&self) -> OE_2_R {
        OE_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn oe_3(&self) -> OE_3_R {
        OE_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn oe_4(&self) -> OE_4_R {
        OE_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn oe_5(&self) -> OE_5_R {
        OE_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn oe_6(&self) -> OE_6_R {
        OE_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oe_7(&self) -> OE_7_R {
        OE_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn oe_8(&self) -> OE_8_R {
        OE_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn oe_9(&self) -> OE_9_R {
        OE_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oe_10(&self) -> OE_10_R {
        OE_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn oe_11(&self) -> OE_11_R {
        OE_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn oe_12(&self) -> OE_12_R {
        OE_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn oe_13(&self) -> OE_13_R {
        OE_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn oe_14(&self) -> OE_14_R {
        OE_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn oe_15(&self) -> OE_15_R {
        OE_15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn oe_0(&mut self) -> OE_0_W<0> {
        OE_0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn oe_1(&mut self) -> OE_1_W<1> {
        OE_1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn oe_2(&mut self) -> OE_2_W<2> {
        OE_2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oe_3(&mut self) -> OE_3_W<3> {
        OE_3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn oe_4(&mut self) -> OE_4_W<4> {
        OE_4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn oe_5(&mut self) -> OE_5_W<5> {
        OE_5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn oe_6(&mut self) -> OE_6_W<6> {
        OE_6_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn oe_7(&mut self) -> OE_7_W<7> {
        OE_7_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn oe_8(&mut self) -> OE_8_W<8> {
        OE_8_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn oe_9(&mut self) -> OE_9_W<9> {
        OE_9_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn oe_10(&mut self) -> OE_10_W<10> {
        OE_10_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn oe_11(&mut self) -> OE_11_W<11> {
        OE_11_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn oe_12(&mut self) -> OE_12_W<12> {
        OE_12_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn oe_13(&mut self) -> OE_13_W<13> {
        OE_13_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn oe_14(&mut self) -> OE_14_W<14> {
        OE_14_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn oe_15(&mut self) -> OE_15_W<15> {
        OE_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Output Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oe](index.html) module"]
pub struct OE_SPEC;
impl crate::RegisterSpec for OE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oe::R](R) reader structure"]
impl crate::Readable for OE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oe::W](W) writer structure"]
impl crate::Writable for OE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OE to value 0"]
impl crate::Resettable for OE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
