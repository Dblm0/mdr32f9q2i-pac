#[doc = "Register `PULL` reader"]
pub struct R(crate::R<PULL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PULL` writer"]
pub struct W(crate::W<PULL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PULL_SPEC>;
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
impl From<crate::W<PULL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PULL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PULL_DOWN_0` reader - "]
pub type PULL_DOWN_0_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_0` writer - "]
pub type PULL_DOWN_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_1` reader - "]
pub type PULL_DOWN_1_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_1` writer - "]
pub type PULL_DOWN_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_2` reader - "]
pub type PULL_DOWN_2_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_2` writer - "]
pub type PULL_DOWN_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_3` reader - "]
pub type PULL_DOWN_3_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_3` writer - "]
pub type PULL_DOWN_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_4` reader - "]
pub type PULL_DOWN_4_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_4` writer - "]
pub type PULL_DOWN_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_5` reader - "]
pub type PULL_DOWN_5_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_5` writer - "]
pub type PULL_DOWN_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_6` reader - "]
pub type PULL_DOWN_6_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_6` writer - "]
pub type PULL_DOWN_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_7` reader - "]
pub type PULL_DOWN_7_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_7` writer - "]
pub type PULL_DOWN_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_8` reader - "]
pub type PULL_DOWN_8_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_8` writer - "]
pub type PULL_DOWN_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_9` reader - "]
pub type PULL_DOWN_9_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_9` writer - "]
pub type PULL_DOWN_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_10` reader - "]
pub type PULL_DOWN_10_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_10` writer - "]
pub type PULL_DOWN_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_11` reader - "]
pub type PULL_DOWN_11_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_11` writer - "]
pub type PULL_DOWN_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_12` reader - "]
pub type PULL_DOWN_12_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_12` writer - "]
pub type PULL_DOWN_12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_13` reader - "]
pub type PULL_DOWN_13_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_13` writer - "]
pub type PULL_DOWN_13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_14` reader - "]
pub type PULL_DOWN_14_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_14` writer - "]
pub type PULL_DOWN_14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_DOWN_15` reader - "]
pub type PULL_DOWN_15_R = crate::BitReader<bool>;
#[doc = "Field `PULL_DOWN_15` writer - "]
pub type PULL_DOWN_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_0` reader - "]
pub type PULL_UP_0_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_0` writer - "]
pub type PULL_UP_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_1` reader - "]
pub type PULL_UP_1_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_1` writer - "]
pub type PULL_UP_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_2` reader - "]
pub type PULL_UP_2_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_2` writer - "]
pub type PULL_UP_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_3` reader - "]
pub type PULL_UP_3_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_3` writer - "]
pub type PULL_UP_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_4` reader - "]
pub type PULL_UP_4_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_4` writer - "]
pub type PULL_UP_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_5` reader - "]
pub type PULL_UP_5_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_5` writer - "]
pub type PULL_UP_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_6` reader - "]
pub type PULL_UP_6_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_6` writer - "]
pub type PULL_UP_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_7` reader - "]
pub type PULL_UP_7_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_7` writer - "]
pub type PULL_UP_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_8` reader - "]
pub type PULL_UP_8_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_8` writer - "]
pub type PULL_UP_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_9` reader - "]
pub type PULL_UP_9_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_9` writer - "]
pub type PULL_UP_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_10` reader - "]
pub type PULL_UP_10_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_10` writer - "]
pub type PULL_UP_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_11` reader - "]
pub type PULL_UP_11_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_11` writer - "]
pub type PULL_UP_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_12` reader - "]
pub type PULL_UP_12_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_12` writer - "]
pub type PULL_UP_12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_13` reader - "]
pub type PULL_UP_13_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_13` writer - "]
pub type PULL_UP_13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_14` reader - "]
pub type PULL_UP_14_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_14` writer - "]
pub type PULL_UP_14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
#[doc = "Field `PULL_UP_15` reader - "]
pub type PULL_UP_15_R = crate::BitReader<bool>;
#[doc = "Field `PULL_UP_15` writer - "]
pub type PULL_UP_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PULL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pull_down_0(&self) -> PULL_DOWN_0_R {
        PULL_DOWN_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pull_down_1(&self) -> PULL_DOWN_1_R {
        PULL_DOWN_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pull_down_2(&self) -> PULL_DOWN_2_R {
        PULL_DOWN_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pull_down_3(&self) -> PULL_DOWN_3_R {
        PULL_DOWN_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pull_down_4(&self) -> PULL_DOWN_4_R {
        PULL_DOWN_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pull_down_5(&self) -> PULL_DOWN_5_R {
        PULL_DOWN_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pull_down_6(&self) -> PULL_DOWN_6_R {
        PULL_DOWN_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pull_down_7(&self) -> PULL_DOWN_7_R {
        PULL_DOWN_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pull_down_8(&self) -> PULL_DOWN_8_R {
        PULL_DOWN_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pull_down_9(&self) -> PULL_DOWN_9_R {
        PULL_DOWN_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pull_down_10(&self) -> PULL_DOWN_10_R {
        PULL_DOWN_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pull_down_11(&self) -> PULL_DOWN_11_R {
        PULL_DOWN_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pull_down_12(&self) -> PULL_DOWN_12_R {
        PULL_DOWN_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pull_down_13(&self) -> PULL_DOWN_13_R {
        PULL_DOWN_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pull_down_14(&self) -> PULL_DOWN_14_R {
        PULL_DOWN_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pull_down_15(&self) -> PULL_DOWN_15_R {
        PULL_DOWN_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pull_up_0(&self) -> PULL_UP_0_R {
        PULL_UP_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pull_up_1(&self) -> PULL_UP_1_R {
        PULL_UP_1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pull_up_2(&self) -> PULL_UP_2_R {
        PULL_UP_2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pull_up_3(&self) -> PULL_UP_3_R {
        PULL_UP_3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pull_up_4(&self) -> PULL_UP_4_R {
        PULL_UP_4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pull_up_5(&self) -> PULL_UP_5_R {
        PULL_UP_5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pull_up_6(&self) -> PULL_UP_6_R {
        PULL_UP_6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pull_up_7(&self) -> PULL_UP_7_R {
        PULL_UP_7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pull_up_8(&self) -> PULL_UP_8_R {
        PULL_UP_8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pull_up_9(&self) -> PULL_UP_9_R {
        PULL_UP_9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pull_up_10(&self) -> PULL_UP_10_R {
        PULL_UP_10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pull_up_11(&self) -> PULL_UP_11_R {
        PULL_UP_11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pull_up_12(&self) -> PULL_UP_12_R {
        PULL_UP_12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pull_up_13(&self) -> PULL_UP_13_R {
        PULL_UP_13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pull_up_14(&self) -> PULL_UP_14_R {
        PULL_UP_14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pull_up_15(&self) -> PULL_UP_15_R {
        PULL_UP_15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_0(&mut self) -> PULL_DOWN_0_W<0> {
        PULL_DOWN_0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_1(&mut self) -> PULL_DOWN_1_W<1> {
        PULL_DOWN_1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_2(&mut self) -> PULL_DOWN_2_W<2> {
        PULL_DOWN_2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_3(&mut self) -> PULL_DOWN_3_W<3> {
        PULL_DOWN_3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_4(&mut self) -> PULL_DOWN_4_W<4> {
        PULL_DOWN_4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_5(&mut self) -> PULL_DOWN_5_W<5> {
        PULL_DOWN_5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_6(&mut self) -> PULL_DOWN_6_W<6> {
        PULL_DOWN_6_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_7(&mut self) -> PULL_DOWN_7_W<7> {
        PULL_DOWN_7_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_8(&mut self) -> PULL_DOWN_8_W<8> {
        PULL_DOWN_8_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_9(&mut self) -> PULL_DOWN_9_W<9> {
        PULL_DOWN_9_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_10(&mut self) -> PULL_DOWN_10_W<10> {
        PULL_DOWN_10_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_11(&mut self) -> PULL_DOWN_11_W<11> {
        PULL_DOWN_11_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_12(&mut self) -> PULL_DOWN_12_W<12> {
        PULL_DOWN_12_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_13(&mut self) -> PULL_DOWN_13_W<13> {
        PULL_DOWN_13_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_14(&mut self) -> PULL_DOWN_14_W<14> {
        PULL_DOWN_14_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_15(&mut self) -> PULL_DOWN_15_W<15> {
        PULL_DOWN_15_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_0(&mut self) -> PULL_UP_0_W<16> {
        PULL_UP_0_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_1(&mut self) -> PULL_UP_1_W<17> {
        PULL_UP_1_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_2(&mut self) -> PULL_UP_2_W<18> {
        PULL_UP_2_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_3(&mut self) -> PULL_UP_3_W<19> {
        PULL_UP_3_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_4(&mut self) -> PULL_UP_4_W<20> {
        PULL_UP_4_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_5(&mut self) -> PULL_UP_5_W<21> {
        PULL_UP_5_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_6(&mut self) -> PULL_UP_6_W<22> {
        PULL_UP_6_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_7(&mut self) -> PULL_UP_7_W<23> {
        PULL_UP_7_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_8(&mut self) -> PULL_UP_8_W<24> {
        PULL_UP_8_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_9(&mut self) -> PULL_UP_9_W<25> {
        PULL_UP_9_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_10(&mut self) -> PULL_UP_10_W<26> {
        PULL_UP_10_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_11(&mut self) -> PULL_UP_11_W<27> {
        PULL_UP_11_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_12(&mut self) -> PULL_UP_12_W<28> {
        PULL_UP_12_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_13(&mut self) -> PULL_UP_13_W<29> {
        PULL_UP_13_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_14(&mut self) -> PULL_UP_14_W<30> {
        PULL_UP_14_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pull_up_15(&mut self) -> PULL_UP_15_W<31> {
        PULL_UP_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Pull Up/Down Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pull](index.html) module"]
pub struct PULL_SPEC;
impl crate::RegisterSpec for PULL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pull::R](R) reader structure"]
impl crate::Readable for PULL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pull::W](W) writer structure"]
impl crate::Writable for PULL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PULL to value 0"]
impl crate::Resettable for PULL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
