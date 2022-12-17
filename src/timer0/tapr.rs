# [doc = "Register `TAPR` reader"] pub struct R (crate :: R < TAPR_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < TAPR_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < TAPR_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < TAPR_SPEC >) -> Self { R (reader) } } # [doc = "Register `TAPR` writer"] pub struct W (crate :: W < TAPR_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < TAPR_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < TAPR_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < TAPR_SPEC >) -> Self { W (writer) } } # [doc = "Field `TIMER_TAPR_TAPSR` reader - GPTM Timer A Prescale"] pub type TIMER_TAPR_TAPSR_R = crate :: FieldReader < u8 , u8 > ; # [doc = "Field `TIMER_TAPR_TAPSR` writer - GPTM Timer A Prescale"] pub type TIMER_TAPR_TAPSR_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , TAPR_SPEC , u8 , u8 , 8 , O > ; # [doc = "Field `TIMER_TAPR_TAPSRH` reader - GPTM Timer A Prescale High Byte"] pub type TIMER_TAPR_TAPSRH_R = crate :: FieldReader < u8 , u8 > ; # [doc = "Field `TIMER_TAPR_TAPSRH` writer - GPTM Timer A Prescale High Byte"] pub type TIMER_TAPR_TAPSRH_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , TAPR_SPEC , u8 , u8 , 8 , O > ; impl R { # [doc = "Bits 0:7 - GPTM Timer A Prescale"] # [inline (always)] pub fn timer_tapr_tapsr (& self) -> TIMER_TAPR_TAPSR_R { TIMER_TAPR_TAPSR_R :: new ((self . bits & 0xff) as u8) } # [doc = "Bits 8:15 - GPTM Timer A Prescale High Byte"] # [inline (always)] pub fn timer_tapr_tapsrh (& self) -> TIMER_TAPR_TAPSRH_R { TIMER_TAPR_TAPSRH_R :: new (((self . bits >> 8) & 0xff) as u8) } } impl W { # [doc = "Bits 0:7 - GPTM Timer A Prescale"] # [inline (always)] # [must_use] pub fn timer_tapr_tapsr (& mut self) -> TIMER_TAPR_TAPSR_W < 0 > { TIMER_TAPR_TAPSR_W :: new (self) } # [doc = "Bits 8:15 - GPTM Timer A Prescale High Byte"] # [inline (always)] # [must_use] pub fn timer_tapr_tapsrh (& mut self) -> TIMER_TAPR_TAPSRH_W < 8 > { TIMER_TAPR_TAPSRH_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "GPTM Timer A Prescale\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tapr](index.html) module"] pub struct TAPR_SPEC ; impl crate :: RegisterSpec for TAPR_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [tapr::R](R) reader structure"] impl crate :: Readable for TAPR_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [tapr::W](W) writer structure"] impl crate :: Writable for TAPR_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets TAPR to value 0"] impl crate :: Resettable for TAPR_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }