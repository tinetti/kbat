
use byteorder::{BigEndian, WriteBytesExt};

use crate::error::Result;

pub trait Request {
    fn into_bytes(self) -> Result<Vec<u8>>;
}

pub struct GetMetadataRequest {}

impl GetMetadataRequest {}

impl Request for GetMetadataRequest {
    fn into_bytes(self) -> Result<Vec<u8>> {
        let mut v = vec![];
        v.write_i32::<BigEndian>(-1)?;
        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;
    use crate::request::GetMetadataRequest;

    #[test]
    fn get_metadata() -> Result<()> {
        let req = GetMetadataRequest {};
        let buf: Vec<u8> = Vec::new();
        let buf = Box::new(buf);
        // req.write(buf);
        assert_eq!(buf.as_slice(), vec![0, 0, 0, 21, 0, 3, 0, 5, 0, 0, 0, 0, 0, 6, 115, 97, 114, 97, 109, 97, 255, 255, 255, 255, 0]);
        Ok(())
    }
}
