use elrond_wasm::elrond_codec::*;

/// Copied from elrond-wasm serialization tests. 
pub enum SerExample2 {
    Unit,
    Newtype(u32),
    Tuple(u32, u32),
    Struct { a: u32 },
}

impl NestedEncode for SerExample2 {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        match self {
            SerExample2::Unit => {
                0u32.dep_encode(dest)?;
            },
            SerExample2::Newtype(arg1) => {
                1u32.dep_encode(dest)?;
                arg1.dep_encode(dest)?;
            },
            SerExample2::Tuple(arg1, arg2) => {
                2u32.dep_encode(dest)?;
                arg1.dep_encode(dest)?;
                arg2.dep_encode(dest)?;
            },
            SerExample2::Struct { a } => {
                3u32.dep_encode(dest)?;
                a.dep_encode(dest)?;
            },
        }
        Ok(())
    }
}

impl TopEncode for SerExample2 {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        top_encode_from_nested(self, output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(&self, output: O, c: ExitCtx, exit: fn(ExitCtx, EncodeError) -> !) {
        top_encode_from_nested_or_exit(self, output, c, exit);
    }
}

impl NestedDecode for SerExample2 {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        match u32::dep_decode(input)? {
            0 => Ok(SerExample2::Unit),
            1 => Ok(SerExample2::Newtype(u32::dep_decode(input)?)),
            2 => Ok(SerExample2::Tuple(u32::dep_decode(input)?, u32::dep_decode(input)?)),
            3 => Ok(SerExample2::Struct{ a: u32::dep_decode(input)? }),
            _ => Err(DecodeError::INVALID_VALUE),
        }
    }
}

impl TopDecode for SerExample2 {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        top_decode_from_nested(input)
    }
}
