#[doc = "Register `ANALOG` reader"]
pub struct R(crate::R<ANALOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANALOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANALOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANALOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANALOG` writer"]
pub struct W(crate::W<ANALOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANALOG_SPEC>;
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
impl From<crate::W<ANALOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANALOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANALOG_EN_0` reader - "]
pub type ANALOG_EN_0_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_0` writer - "]
pub type ANALOG_EN_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_1` reader - "]
pub type ANALOG_EN_1_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_1` writer - "]
pub type ANALOG_EN_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_2` reader - "]
pub type ANALOG_EN_2_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_2` writer - "]
pub type ANALOG_EN_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_3` reader - "]
pub type ANALOG_EN_3_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_3` writer - "]
pub type ANALOG_EN_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_4` reader - "]
pub type ANALOG_EN_4_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_4` writer - "]
pub type ANALOG_EN_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_5` reader - "]
pub type ANALOG_EN_5_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_5` writer - "]
pub type ANALOG_EN_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_6` reader - "]
pub type ANALOG_EN_6_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_6` writer - "]
pub type ANALOG_EN_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_7` reader - "]
pub type ANALOG_EN_7_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_7` writer - "]
pub type ANALOG_EN_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_8` reader - "]
pub type ANALOG_EN_8_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_8` writer - "]
pub type ANALOG_EN_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_9` reader - "]
pub type ANALOG_EN_9_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_9` writer - "]
pub type ANALOG_EN_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_10` reader - "]
pub type ANALOG_EN_10_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_10` writer - "]
pub type ANALOG_EN_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_11` reader - "]
pub type ANALOG_EN_11_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_11` writer - "]
pub type ANALOG_EN_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_12` reader - "]
pub type ANALOG_EN_12_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_12` writer - "]
pub type ANALOG_EN_12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_13` reader - "]
pub type ANALOG_EN_13_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_13` writer - "]
pub type ANALOG_EN_13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_14` reader - "]
pub type ANALOG_EN_14_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_14` writer - "]
pub type ANALOG_EN_14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
#[doc = "Field `ANALOG_EN_15` reader - "]
pub type ANALOG_EN_15_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG_EN_15` writer - "]
pub type ANALOG_EN_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANALOG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn analog_en_0(&self) -> ANALOG_EN_0_R {
        ANALOG_EN_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn analog_en_1(&self) -> ANALOG_EN_1_R {
        ANALOG_EN_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn analog_en_2(&self) -> ANALOG_EN_2_R {
        ANALOG_EN_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn analog_en_3(&self) -> ANALOG_EN_3_R {
        ANALOG_EN_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn analog_en_4(&self) -> ANALOG_EN_4_R {
        ANALOG_EN_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn analog_en_5(&self) -> ANALOG_EN_5_R {
        ANALOG_EN_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn analog_en_6(&self) -> ANALOG_EN_6_R {
        ANALOG_EN_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn analog_en_7(&self) -> ANALOG_EN_7_R {
        ANALOG_EN_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn analog_en_8(&self) -> ANALOG_EN_8_R {
        ANALOG_EN_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn analog_en_9(&self) -> ANALOG_EN_9_R {
        ANALOG_EN_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn analog_en_10(&self) -> ANALOG_EN_10_R {
        ANALOG_EN_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn analog_en_11(&self) -> ANALOG_EN_11_R {
        ANALOG_EN_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn analog_en_12(&self) -> ANALOG_EN_12_R {
        ANALOG_EN_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn analog_en_13(&self) -> ANALOG_EN_13_R {
        ANALOG_EN_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn analog_en_14(&self) -> ANALOG_EN_14_R {
        ANALOG_EN_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn analog_en_15(&self) -> ANALOG_EN_15_R {
        ANALOG_EN_15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_0(&mut self) -> ANALOG_EN_0_W<0> {
        ANALOG_EN_0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_1(&mut self) -> ANALOG_EN_1_W<1> {
        ANALOG_EN_1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_2(&mut self) -> ANALOG_EN_2_W<2> {
        ANALOG_EN_2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_3(&mut self) -> ANALOG_EN_3_W<3> {
        ANALOG_EN_3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_4(&mut self) -> ANALOG_EN_4_W<4> {
        ANALOG_EN_4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_5(&mut self) -> ANALOG_EN_5_W<5> {
        ANALOG_EN_5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_6(&mut self) -> ANALOG_EN_6_W<6> {
        ANALOG_EN_6_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_7(&mut self) -> ANALOG_EN_7_W<7> {
        ANALOG_EN_7_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_8(&mut self) -> ANALOG_EN_8_W<8> {
        ANALOG_EN_8_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_9(&mut self) -> ANALOG_EN_9_W<9> {
        ANALOG_EN_9_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_10(&mut self) -> ANALOG_EN_10_W<10> {
        ANALOG_EN_10_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_11(&mut self) -> ANALOG_EN_11_W<11> {
        ANALOG_EN_11_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_12(&mut self) -> ANALOG_EN_12_W<12> {
        ANALOG_EN_12_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_13(&mut self) -> ANALOG_EN_13_W<13> {
        ANALOG_EN_13_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_14(&mut self) -> ANALOG_EN_14_W<14> {
        ANALOG_EN_14_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn analog_en_15(&mut self) -> ANALOG_EN_15_W<15> {
        ANALOG_EN_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Analog Function Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [analog](index.html) module"]
pub struct ANALOG_SPEC;
impl crate::RegisterSpec for ANALOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [analog::R](R) reader structure"]
impl crate::Readable for ANALOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [analog::W](W) writer structure"]
impl crate::Writable for ANALOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANALOG to value 0"]
impl crate::Resettable for ANALOG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
