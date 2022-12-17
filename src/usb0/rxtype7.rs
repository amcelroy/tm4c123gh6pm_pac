# [doc = "Register `RXTYPE7` reader"] pub struct R (crate :: R < RXTYPE7_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < RXTYPE7_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < RXTYPE7_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < RXTYPE7_SPEC >) -> Self { R (reader) } } # [doc = "Register `RXTYPE7` writer"] pub struct W (crate :: W < RXTYPE7_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < RXTYPE7_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < RXTYPE7_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < RXTYPE7_SPEC >) -> Self { W (writer) } } # [doc = "Field `USB_RXTYPE7_TEP` reader - Target Endpoint Number"] pub type USB_RXTYPE7_TEP_R = crate :: FieldReader < u8 , u8 > ; # [doc = "Field `USB_RXTYPE7_TEP` writer - Target Endpoint Number"] pub type USB_RXTYPE7_TEP_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u8 , RXTYPE7_SPEC , u8 , u8 , 4 , O > ; # [doc = "Field `USB_RXTYPE7_PROTO` reader - Protocol"] pub type USB_RXTYPE7_PROTO_R = crate :: FieldReader < u8 , USB_RXTYPE7_PROTO_A > ; # [doc = "Protocol\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum USB_RXTYPE7_PROTO_A { # [doc = "0: Control"] USB_RXTYPE7_PROTO_CTRL = 0 , # [doc = "1: Isochronous"] USB_RXTYPE7_PROTO_ISOC = 1 , # [doc = "2: Bulk"] USB_RXTYPE7_PROTO_BULK = 2 , # [doc = "3: Interrupt"] USB_RXTYPE7_PROTO_INT = 3 , } impl From < USB_RXTYPE7_PROTO_A > for u8 { # [inline (always)] fn from (variant : USB_RXTYPE7_PROTO_A) -> Self { variant as _ } } impl USB_RXTYPE7_PROTO_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub fn variant (& self) -> USB_RXTYPE7_PROTO_A { match self . bits { 0 => USB_RXTYPE7_PROTO_A :: USB_RXTYPE7_PROTO_CTRL , 1 => USB_RXTYPE7_PROTO_A :: USB_RXTYPE7_PROTO_ISOC , 2 => USB_RXTYPE7_PROTO_A :: USB_RXTYPE7_PROTO_BULK , 3 => USB_RXTYPE7_PROTO_A :: USB_RXTYPE7_PROTO_INT , _ => unreachable ! () , } } # [doc = "Checks if the value of the field is `USB_RXTYPE7_PROTO_CTRL`"] # [inline (always)] pub fn is_usb_rxtype7_proto_ctrl (& self) -> bool { * self == USB_RXTYPE7_PROTO_A :: USB_RXTYPE7_PROTO_CTRL } # [doc = "Checks if the value of the field is `USB_RXTYPE7_PROTO_ISOC`"] # [inline (always)] pub fn is_usb_rxtype7_proto_isoc (& self) -> bool { * self == USB_RXTYPE7_PROTO_A :: USB_RXTYPE7_PROTO_ISOC } # [doc = "Checks if the value of the field is `USB_RXTYPE7_PROTO_BULK`"] # [inline (always)] pub fn is_usb_rxtype7_proto_bulk (& self) -> bool { * self == USB_RXTYPE7_PROTO_A :: USB_RXTYPE7_PROTO_BULK } # [doc = "Checks if the value of the field is `USB_RXTYPE7_PROTO_INT`"] # [inline (always)] pub fn is_usb_rxtype7_proto_int (& self) -> bool { * self == USB_RXTYPE7_PROTO_A :: USB_RXTYPE7_PROTO_INT } } # [doc = "Field `USB_RXTYPE7_PROTO` writer - Protocol"] pub type USB_RXTYPE7_PROTO_W < 'a , const O : u8 > = crate :: FieldWriterSafe < 'a , u8 , RXTYPE7_SPEC , u8 , USB_RXTYPE7_PROTO_A , 2 , O > ; impl < 'a , const O : u8 > USB_RXTYPE7_PROTO_W < 'a , O > { # [doc = "Control"] # [inline (always)] pub fn usb_rxtype7_proto_ctrl (self) -> & 'a mut W { self . variant (USB_RXTYPE7_PROTO_A :: USB_RXTYPE7_PROTO_CTRL) } # [doc = "Isochronous"] # [inline (always)] pub fn usb_rxtype7_proto_isoc (self) -> & 'a mut W { self . variant (USB_RXTYPE7_PROTO_A :: USB_RXTYPE7_PROTO_ISOC) } # [doc = "Bulk"] # [inline (always)] pub fn usb_rxtype7_proto_bulk (self) -> & 'a mut W { self . variant (USB_RXTYPE7_PROTO_A :: USB_RXTYPE7_PROTO_BULK) } # [doc = "Interrupt"] # [inline (always)] pub fn usb_rxtype7_proto_int (self) -> & 'a mut W { self . variant (USB_RXTYPE7_PROTO_A :: USB_RXTYPE7_PROTO_INT) } } # [doc = "Field `USB_RXTYPE7_SPEED` reader - Operating Speed"] pub type USB_RXTYPE7_SPEED_R = crate :: FieldReader < u8 , USB_RXTYPE7_SPEED_A > ; # [doc = "Operating Speed\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum USB_RXTYPE7_SPEED_A { # [doc = "0: Default"] USB_RXTYPE7_SPEED_DFLT = 0 , # [doc = "2: Full"] USB_RXTYPE7_SPEED_FULL = 2 , # [doc = "3: Low"] USB_RXTYPE7_SPEED_LOW = 3 , } impl From < USB_RXTYPE7_SPEED_A > for u8 { # [inline (always)] fn from (variant : USB_RXTYPE7_SPEED_A) -> Self { variant as _ } } impl USB_RXTYPE7_SPEED_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub fn variant (& self) -> Option < USB_RXTYPE7_SPEED_A > { match self . bits { 0 => Some (USB_RXTYPE7_SPEED_A :: USB_RXTYPE7_SPEED_DFLT) , 2 => Some (USB_RXTYPE7_SPEED_A :: USB_RXTYPE7_SPEED_FULL) , 3 => Some (USB_RXTYPE7_SPEED_A :: USB_RXTYPE7_SPEED_LOW) , _ => None , } } # [doc = "Checks if the value of the field is `USB_RXTYPE7_SPEED_DFLT`"] # [inline (always)] pub fn is_usb_rxtype7_speed_dflt (& self) -> bool { * self == USB_RXTYPE7_SPEED_A :: USB_RXTYPE7_SPEED_DFLT } # [doc = "Checks if the value of the field is `USB_RXTYPE7_SPEED_FULL`"] # [inline (always)] pub fn is_usb_rxtype7_speed_full (& self) -> bool { * self == USB_RXTYPE7_SPEED_A :: USB_RXTYPE7_SPEED_FULL } # [doc = "Checks if the value of the field is `USB_RXTYPE7_SPEED_LOW`"] # [inline (always)] pub fn is_usb_rxtype7_speed_low (& self) -> bool { * self == USB_RXTYPE7_SPEED_A :: USB_RXTYPE7_SPEED_LOW } } # [doc = "Field `USB_RXTYPE7_SPEED` writer - Operating Speed"] pub type USB_RXTYPE7_SPEED_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u8 , RXTYPE7_SPEC , u8 , USB_RXTYPE7_SPEED_A , 2 , O > ; impl < 'a , const O : u8 > USB_RXTYPE7_SPEED_W < 'a , O > { # [doc = "Default"] # [inline (always)] pub fn usb_rxtype7_speed_dflt (self) -> & 'a mut W { self . variant (USB_RXTYPE7_SPEED_A :: USB_RXTYPE7_SPEED_DFLT) } # [doc = "Full"] # [inline (always)] pub fn usb_rxtype7_speed_full (self) -> & 'a mut W { self . variant (USB_RXTYPE7_SPEED_A :: USB_RXTYPE7_SPEED_FULL) } # [doc = "Low"] # [inline (always)] pub fn usb_rxtype7_speed_low (self) -> & 'a mut W { self . variant (USB_RXTYPE7_SPEED_A :: USB_RXTYPE7_SPEED_LOW) } } impl R { # [doc = "Bits 0:3 - Target Endpoint Number"] # [inline (always)] pub fn usb_rxtype7_tep (& self) -> USB_RXTYPE7_TEP_R { USB_RXTYPE7_TEP_R :: new (self . bits & 0x0f) } # [doc = "Bits 4:5 - Protocol"] # [inline (always)] pub fn usb_rxtype7_proto (& self) -> USB_RXTYPE7_PROTO_R { USB_RXTYPE7_PROTO_R :: new ((self . bits >> 4) & 3) } # [doc = "Bits 6:7 - Operating Speed"] # [inline (always)] pub fn usb_rxtype7_speed (& self) -> USB_RXTYPE7_SPEED_R { USB_RXTYPE7_SPEED_R :: new ((self . bits >> 6) & 3) } } impl W { # [doc = "Bits 0:3 - Target Endpoint Number"] # [inline (always)] # [must_use] pub fn usb_rxtype7_tep (& mut self) -> USB_RXTYPE7_TEP_W < 0 > { USB_RXTYPE7_TEP_W :: new (self) } # [doc = "Bits 4:5 - Protocol"] # [inline (always)] # [must_use] pub fn usb_rxtype7_proto (& mut self) -> USB_RXTYPE7_PROTO_W < 4 > { USB_RXTYPE7_PROTO_W :: new (self) } # [doc = "Bits 6:7 - Operating Speed"] # [inline (always)] # [must_use] pub fn usb_rxtype7_speed (& mut self) -> USB_RXTYPE7_SPEED_W < 6 > { USB_RXTYPE7_SPEED_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u8) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "USB Host Configure Receive Type Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtype7](index.html) module"] pub struct RXTYPE7_SPEC ; impl crate :: RegisterSpec for RXTYPE7_SPEC { type Ux = u8 ; } # [doc = "`read()` method returns [rxtype7::R](R) reader structure"] impl crate :: Readable for RXTYPE7_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [rxtype7::W](W) writer structure"] impl crate :: Writable for RXTYPE7_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets RXTYPE7 to value 0"] impl crate :: Resettable for RXTYPE7_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }