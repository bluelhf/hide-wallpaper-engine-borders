use std::error::Error;
use xcb::x;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Words(pub u32);

#[allow(dead_code)]
impl Words {
    pub fn to_bytes(self) -> u32 {
        self.0 * 4
    }
}

pub fn get_property<T: x::PropEl + Clone>(
    conn: &xcb::Connection,
    window: x::Window,
    property: x::Atom,
    r#type: x::Atom,
) -> Result<Vec<T>, Box<dyn Error>> {
    let mut offset = Words(0);
    let mut value = Vec::new();

    /// Size of the property read buffer in 32-bit words.
    const BUFFER_SIZE: Words = Words(1024);

    loop {
        let cookie = conn.send_request(&x::GetProperty {
            delete: false,
            window,
            property,
            r#type,
            long_offset: offset.0,
            long_length: BUFFER_SIZE.0,
        });

        let reply = conn.wait_for_reply(cookie)?;
        if reply.r#type() == x::ATOM_NONE {
            return Ok(Vec::new());
        }

        value.extend_from_slice(reply.value());
        
        if reply.bytes_after() == 0 {
            break;
        }

        offset.0 += BUFFER_SIZE.0;
    }

    Ok(value)
}

pub fn get_geometry(
    conn: &xcb::Connection,
    drawable: x::Drawable,
) -> Result<x::GetGeometryReply, Box<dyn Error>> {
    let cookie = conn.send_request(&x::GetGeometry { drawable });
    Ok(conn.wait_for_reply(cookie)?)
}
