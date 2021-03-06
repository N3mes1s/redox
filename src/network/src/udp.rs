use core::mem::size_of;
use core::option::Option;

use common::debug::*;
use common::vec::*;

use network::common::*;

#[derive(Copy, Clone)]
#[repr(packed)]
pub struct UDPHeader {
    pub src: n16,
    pub dst: n16,
    pub len: n16,
    pub checksum: Checksum,
}

pub struct UDP {
    pub header: UDPHeader,
    pub data: Vec<u8>,
}

impl FromBytes for UDP {
    fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        if bytes.len() >= size_of::<UDPHeader>() {
            unsafe {
                return Option::Some(UDP {
                    header: *(bytes.as_ptr() as *const UDPHeader),
                    data: bytes.sub(size_of::<UDPHeader>(),
                                    bytes.len() - size_of::<UDPHeader>()),
                });
            }
        }
        Option::None
    }
}

impl ToBytes for UDP {
    fn to_bytes(&self) -> Vec<u8> {
        unsafe {
            let header_ptr: *const UDPHeader = &self.header;
            let mut ret = Vec::from_raw_buf(header_ptr as *const u8, size_of::<UDPHeader>());
            ret.push_all(&self.data);
            ret
        }
    }
}

impl UDP {
    pub fn d(&self) {
        d("UDP from ");
        dd(self.header.src.get() as usize);
        d(" to ");
        dd(self.header.dst.get() as usize);
        d(" data ");
        dd(self.data.len());
    }
}
