# [doc = "Register `FSIZE` reader"] pub struct R (crate :: R < FSIZE_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < FSIZE_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < FSIZE_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < FSIZE_SPEC >) -> Self { R (reader) } } # [doc = "Register `FSIZE` writer"] pub struct W (crate :: W < FSIZE_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < FSIZE_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < FSIZE_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < FSIZE_SPEC >) -> Self { W (writer) } } # [doc = "Field `FLASH_FSIZE_SIZE` reader - Flash Size"] pub type FLASH_FSIZE_SIZE_R = crate :: FieldReader < u16 , FLASH_FSIZE_SIZE_A > ; # [doc = "Flash Size\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u16)] pub enum FLASH_FSIZE_SIZE_A { # [doc = "127: 256 KB of Flash"] FLASH_FSIZE_SIZE_256KB = 127 , } impl From < FLASH_FSIZE_SIZE_A > for u16 { # [inline (always)] fn from (variant : FLASH_FSIZE_SIZE_A) -> Self { variant as _ } } impl FLASH_FSIZE_SIZE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub fn variant (& self) -> Option < FLASH_FSIZE_SIZE_A > { match self . bits { 127 => Some (FLASH_FSIZE_SIZE_A :: FLASH_FSIZE_SIZE_256KB) , _ => None , } } # [doc = "Checks if the value of the field is `FLASH_FSIZE_SIZE_256KB`"] # [inline (always)] pub fn is_flash_fsize_size_256kb (& self) -> bool { * self == FLASH_FSIZE_SIZE_A :: FLASH_FSIZE_SIZE_256KB } } # [doc = "Field `FLASH_FSIZE_SIZE` writer - Flash Size"] pub type FLASH_FSIZE_SIZE_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , FSIZE_SPEC , u16 , FLASH_FSIZE_SIZE_A , 16 , O > ; impl < 'a , const O : u8 > FLASH_FSIZE_SIZE_W < 'a , O > { # [doc = "256 KB of Flash"] # [inline (always)] pub fn flash_fsize_size_256kb (self) -> & 'a mut W { self . variant (FLASH_FSIZE_SIZE_A :: FLASH_FSIZE_SIZE_256KB) } } impl R { # [doc = "Bits 0:15 - Flash Size"] # [inline (always)] pub fn flash_fsize_size (& self) -> FLASH_FSIZE_SIZE_R { FLASH_FSIZE_SIZE_R :: new ((self . bits & 0xffff) as u16) } } impl W { # [doc = "Bits 0:15 - Flash Size"] # [inline (always)] # [must_use] pub fn flash_fsize_size (& mut self) -> FLASH_FSIZE_SIZE_W < 0 > { FLASH_FSIZE_SIZE_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "Flash Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsize](index.html) module"] pub struct FSIZE_SPEC ; impl crate :: RegisterSpec for FSIZE_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [fsize::R](R) reader structure"] impl crate :: Readable for FSIZE_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [fsize::W](W) writer structure"] impl crate :: Writable for FSIZE_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets FSIZE to value 0"] impl crate :: Resettable for FSIZE_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }