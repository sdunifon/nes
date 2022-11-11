use std::fmt::{Debug, Display, Formatter, LowerHex};

#[derive(PartialEq,Clone, Copy)]
pub enum OpCode {
    _0x00 = 0x00,
    _0xA9
}

impl LowerHex for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x}", *self as u8)
    }
}
impl Debug for OpCode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:#04x}", self)
    }
}


//todo do we need a macro to make enum varients and the equivalent string?
impl ToString for OpCode {
    fn to_string(&self) -> String {
        match self {
            OpCode::_0x00 => "0x00".to_string(),
            OpCode::_0xA9 => "0xA9".to_string()
        }
    }
}

impl TryFrom<u8> for OpCode {
    type Error = OpCodeTranslateError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let l = match value {
            0x00 => Ok(OpCode::_0x00),
            // v @ _ => panic!("bad"),
           v @ _ => Err(OpCodeTranslateError(v)),
        };
        l
    }
}


#[derive(Debug)]
pub struct OpCodeTranslateError(u8);

impl Display for OpCodeTranslateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Unknown OpCode: {}", self.0)
    }
}


#[derive(Debug, PartialEq)]
struct OpCodes(Vec<OpCode>);

impl From<Vec<u8>> for OpCodes{
    fn from(_: Vec<u8>) -> Self {
        todo!()
    }
}


impl OpCodes {
    fn parse(input: Vec<u8>) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests{
    use crate::op_code::{OpCode, OpCodes};

    #[test]
   fn from_vec_u8() {
        let opcode = OpCode::try_from(0x07);
         let opcodes = OpCodes::from(vec![0xA9, 0x05, 0x00]);
         assert_eq!(opcodes, OpCodes(vec![OpCode::_0xA9, OpCode::_0x00]));

   }

    #[test]
    fn test_from_u8() {
        let bytes = vec![0xA9, 0x05, 0x00];
        for byte in bytes{
            assert_eq!(OpCode::try_from(byte).unwrap(), OpCode::_0xA9);
        }
    }

    #[test]
    fn test_from() {
        assert!(OpCode::try_from(0xA9).is_ok(), "OpCode::try_from(0xA9) should be ok");
        assert!(OpCode::try_from(0x00).is_ok(), "OpCode::try_from(0x00) should be ok");
    }

    #[test]
    fn test_from_err_invalid() {
        if let Err(e) = OpCode::try_from(0x99){
            println!("error: {:?}", e)
        }else{
            panic!("should have errored on an invalid opcode")
        }
    }

}