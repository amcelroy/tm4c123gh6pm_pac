# [doc = "Register `RXCOUNT3` reader"] pub struct R (crate :: R < RXCOUNT3_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < RXCOUNT3_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < RXCOUNT3_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < RXCOUNT3_SPEC >) -> Self { R (reader) } } # [doc = "Register `RXCOUNT3` writer"] pub struct W (crate :: W < RXCOUNT3_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < RXCOUNT3_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < RXCOUNT3_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < RXCOUNT3_SPEC >) -> Self { W (writer) } } # [doc = "Field `USB_RXCOUNT3_COUNT` reader - Receive Packet Count"] pub type USB_RXCOUNT3_COUNT_R = crate :: FieldReader < u16 , u16 > ; # [doc = "Field `USB_RXCOUNT3_COUNT` writer - Receive Packet Count"] pub type USB_RXCOUNT3_COUNT_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u16 , RXCOUNT3_SPEC , u16 , u16 , 13 , O > ; impl R { # [doc = "Bits 0:12 - Receive Packet Count"] # [inline (always)] pub fn usb_rxcount3_count (& self) -> USB_RXCOUNT3_COUNT_R { USB_RXCOUNT3_COUNT_R :: new (self . bits & 0x1fff) } } impl W { # [doc = "Bits 0:12 - Receive Packet Count"] # [inline (always)] # [must_use] pub fn usb_rxcount3_count (& mut self) -> USB_RXCOUNT3_COUNT_W < 0 > { USB_RXCOUNT3_COUNT_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u16) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "USB Receive Byte Count Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcount3](index.html) module"] pub struct RXCOUNT3_SPEC ; impl crate :: RegisterSpec for RXCOUNT3_SPEC { type Ux = u16 ; } # [doc = "`read()` method returns [rxcount3::R](R) reader structure"] impl crate :: Readable for RXCOUNT3_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [rxcount3::W](W) writer structure"] impl crate :: Writable for RXCOUNT3_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets RXCOUNT3 to value 0"] impl crate :: Resettable for RXCOUNT3_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }