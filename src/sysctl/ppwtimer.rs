# [doc = "Register `PPWTIMER` reader"] pub struct R (crate :: R < PPWTIMER_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < PPWTIMER_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < PPWTIMER_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < PPWTIMER_SPEC >) -> Self { R (reader) } } # [doc = "Register `PPWTIMER` writer"] pub struct W (crate :: W < PPWTIMER_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < PPWTIMER_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < PPWTIMER_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < PPWTIMER_SPEC >) -> Self { W (writer) } } # [doc = "Field `SYSCTL_PPWTIMER_P0` reader - 32/64-Bit Wide General-Purpose Timer 0 Present"] pub type SYSCTL_PPWTIMER_P0_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_PPWTIMER_P0` writer - 32/64-Bit Wide General-Purpose Timer 0 Present"] pub type SYSCTL_PPWTIMER_P0_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , PPWTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_PPWTIMER_P1` reader - 32/64-Bit Wide General-Purpose Timer 1 Present"] pub type SYSCTL_PPWTIMER_P1_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_PPWTIMER_P1` writer - 32/64-Bit Wide General-Purpose Timer 1 Present"] pub type SYSCTL_PPWTIMER_P1_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , PPWTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_PPWTIMER_P2` reader - 32/64-Bit Wide General-Purpose Timer 2 Present"] pub type SYSCTL_PPWTIMER_P2_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_PPWTIMER_P2` writer - 32/64-Bit Wide General-Purpose Timer 2 Present"] pub type SYSCTL_PPWTIMER_P2_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , PPWTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_PPWTIMER_P3` reader - 32/64-Bit Wide General-Purpose Timer 3 Present"] pub type SYSCTL_PPWTIMER_P3_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_PPWTIMER_P3` writer - 32/64-Bit Wide General-Purpose Timer 3 Present"] pub type SYSCTL_PPWTIMER_P3_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , PPWTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_PPWTIMER_P4` reader - 32/64-Bit Wide General-Purpose Timer 4 Present"] pub type SYSCTL_PPWTIMER_P4_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_PPWTIMER_P4` writer - 32/64-Bit Wide General-Purpose Timer 4 Present"] pub type SYSCTL_PPWTIMER_P4_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , PPWTIMER_SPEC , bool , O > ; # [doc = "Field `SYSCTL_PPWTIMER_P5` reader - 32/64-Bit Wide General-Purpose Timer 5 Present"] pub type SYSCTL_PPWTIMER_P5_R = crate :: BitReader < bool > ; # [doc = "Field `SYSCTL_PPWTIMER_P5` writer - 32/64-Bit Wide General-Purpose Timer 5 Present"] pub type SYSCTL_PPWTIMER_P5_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , PPWTIMER_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - 32/64-Bit Wide General-Purpose Timer 0 Present"] # [inline (always)] pub fn sysctl_ppwtimer_p0 (& self) -> SYSCTL_PPWTIMER_P0_R { SYSCTL_PPWTIMER_P0_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - 32/64-Bit Wide General-Purpose Timer 1 Present"] # [inline (always)] pub fn sysctl_ppwtimer_p1 (& self) -> SYSCTL_PPWTIMER_P1_R { SYSCTL_PPWTIMER_P1_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - 32/64-Bit Wide General-Purpose Timer 2 Present"] # [inline (always)] pub fn sysctl_ppwtimer_p2 (& self) -> SYSCTL_PPWTIMER_P2_R { SYSCTL_PPWTIMER_P2_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - 32/64-Bit Wide General-Purpose Timer 3 Present"] # [inline (always)] pub fn sysctl_ppwtimer_p3 (& self) -> SYSCTL_PPWTIMER_P3_R { SYSCTL_PPWTIMER_P3_R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - 32/64-Bit Wide General-Purpose Timer 4 Present"] # [inline (always)] pub fn sysctl_ppwtimer_p4 (& self) -> SYSCTL_PPWTIMER_P4_R { SYSCTL_PPWTIMER_P4_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - 32/64-Bit Wide General-Purpose Timer 5 Present"] # [inline (always)] pub fn sysctl_ppwtimer_p5 (& self) -> SYSCTL_PPWTIMER_P5_R { SYSCTL_PPWTIMER_P5_R :: new (((self . bits >> 5) & 1) != 0) } } impl W { # [doc = "Bit 0 - 32/64-Bit Wide General-Purpose Timer 0 Present"] # [inline (always)] # [must_use] pub fn sysctl_ppwtimer_p0 (& mut self) -> SYSCTL_PPWTIMER_P0_W < 0 > { SYSCTL_PPWTIMER_P0_W :: new (self) } # [doc = "Bit 1 - 32/64-Bit Wide General-Purpose Timer 1 Present"] # [inline (always)] # [must_use] pub fn sysctl_ppwtimer_p1 (& mut self) -> SYSCTL_PPWTIMER_P1_W < 1 > { SYSCTL_PPWTIMER_P1_W :: new (self) } # [doc = "Bit 2 - 32/64-Bit Wide General-Purpose Timer 2 Present"] # [inline (always)] # [must_use] pub fn sysctl_ppwtimer_p2 (& mut self) -> SYSCTL_PPWTIMER_P2_W < 2 > { SYSCTL_PPWTIMER_P2_W :: new (self) } # [doc = "Bit 3 - 32/64-Bit Wide General-Purpose Timer 3 Present"] # [inline (always)] # [must_use] pub fn sysctl_ppwtimer_p3 (& mut self) -> SYSCTL_PPWTIMER_P3_W < 3 > { SYSCTL_PPWTIMER_P3_W :: new (self) } # [doc = "Bit 4 - 32/64-Bit Wide General-Purpose Timer 4 Present"] # [inline (always)] # [must_use] pub fn sysctl_ppwtimer_p4 (& mut self) -> SYSCTL_PPWTIMER_P4_W < 4 > { SYSCTL_PPWTIMER_P4_W :: new (self) } # [doc = "Bit 5 - 32/64-Bit Wide General-Purpose Timer 5 Present"] # [inline (always)] # [must_use] pub fn sysctl_ppwtimer_p5 (& mut self) -> SYSCTL_PPWTIMER_P5_W < 5 > { SYSCTL_PPWTIMER_P5_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "32/64-Bit Wide General-Purpose Timer Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppwtimer](index.html) module"] pub struct PPWTIMER_SPEC ; impl crate :: RegisterSpec for PPWTIMER_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [ppwtimer::R](R) reader structure"] impl crate :: Readable for PPWTIMER_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [ppwtimer::W](W) writer structure"] impl crate :: Writable for PPWTIMER_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets PPWTIMER to value 0"] impl crate :: Resettable for PPWTIMER_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }