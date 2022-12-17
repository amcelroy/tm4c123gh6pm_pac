# [doc = "Register `PP` reader"] pub struct R (crate :: R < PP_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < PP_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < PP_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < PP_SPEC >) -> Self { R (reader) } } # [doc = "Register `PP` writer"] pub struct W (crate :: W < PP_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < PP_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < PP_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < PP_SPEC >) -> Self { W (writer) } } # [doc = "Field `TIMER_PP_SIZE` reader - Count Size"] pub type TIMER_PP_SIZE_R = crate :: FieldReader < u8 , TIMER_PP_SIZE_A > ; # [doc = "Count Size\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum TIMER_PP_SIZE_A { # [doc = "0: Timer A and Timer B counters are 16 bits each with an 8-bit prescale counter"] TIMER_PP_SIZE_16 = 0 , # [doc = "1: Timer A and Timer B counters are 32 bits each with a 16-bit prescale counter"] TIMER_PP_SIZE_32 = 1 , } impl From < TIMER_PP_SIZE_A > for u8 { # [inline (always)] fn from (variant : TIMER_PP_SIZE_A) -> Self { variant as _ } } impl TIMER_PP_SIZE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub fn variant (& self) -> Option < TIMER_PP_SIZE_A > { match self . bits { 0 => Some (TIMER_PP_SIZE_A :: TIMER_PP_SIZE_16) , 1 => Some (TIMER_PP_SIZE_A :: TIMER_PP_SIZE_32) , _ => None , } } # [doc = "Checks if the value of the field is `TIMER_PP_SIZE_16`"] # [inline (always)] pub fn is_timer_pp_size_16 (& self) -> bool { * self == TIMER_PP_SIZE_A :: TIMER_PP_SIZE_16 } # [doc = "Checks if the value of the field is `TIMER_PP_SIZE_32`"] # [inline (always)] pub fn is_timer_pp_size_32 (& self) -> bool { * self == TIMER_PP_SIZE_A :: TIMER_PP_SIZE_32 } } # [doc = "Field `TIMER_PP_SIZE` writer - Count Size"] pub type TIMER_PP_SIZE_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , PP_SPEC , u8 , TIMER_PP_SIZE_A , 4 , O > ; impl < 'a , const O : u8 > TIMER_PP_SIZE_W < 'a , O > { # [doc = "Timer A and Timer B counters are 16 bits each with an 8-bit prescale counter"] # [inline (always)] pub fn timer_pp_size_16 (self) -> & 'a mut W { self . variant (TIMER_PP_SIZE_A :: TIMER_PP_SIZE_16) } # [doc = "Timer A and Timer B counters are 32 bits each with a 16-bit prescale counter"] # [inline (always)] pub fn timer_pp_size_32 (self) -> & 'a mut W { self . variant (TIMER_PP_SIZE_A :: TIMER_PP_SIZE_32) } } impl R { # [doc = "Bits 0:3 - Count Size"] # [inline (always)] pub fn timer_pp_size (& self) -> TIMER_PP_SIZE_R { TIMER_PP_SIZE_R :: new ((self . bits & 0x0f) as u8) } } impl W { # [doc = "Bits 0:3 - Count Size"] # [inline (always)] # [must_use] pub fn timer_pp_size (& mut self) -> TIMER_PP_SIZE_W < 0 > { TIMER_PP_SIZE_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "GPTM Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](index.html) module"] pub struct PP_SPEC ; impl crate :: RegisterSpec for PP_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [pp::R](R) reader structure"] impl crate :: Readable for PP_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [pp::W](W) writer structure"] impl crate :: Writable for PP_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets PP to value 0"] impl crate :: Resettable for PP_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }