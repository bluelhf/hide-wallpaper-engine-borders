mod utils;

use std::error::Error;
use xcb::x;

pub trait WindowBlockPolicy {
    fn should_block(&self, conn: &xcb::Connection, window: x::Window) -> Result<bool, Box<dyn Error>>;
}

pub struct WallpaperEngineBorderBlockPolicy;

impl WindowBlockPolicy for WallpaperEngineBorderBlockPolicy {
    fn should_block(&self, conn: &xcb::Connection, window: x::Window) -> Result<bool, Box<dyn Error>> {
        let class_property = utils::get_property(conn, window, x::ATOM_WM_CLASS, x::ATOM_ANY)?;
        let name_property = utils::get_property(conn, window, x::ATOM_WM_NAME, x::ATOM_ANY)?;

        let class = String::from_utf8_lossy(&*class_property);
        let name = String::from_utf8_lossy(&*name_property);

        let geometry = utils::get_geometry(conn, x::Drawable::Window(window))?;

        let aspect_ratio: f32 = geometry.width() as f32 / geometry.height() as f32;

        Ok(
            name.is_empty()
                && class.starts_with("steam_app_431960\0")
                && (aspect_ratio < 0.33 || aspect_ratio > 3.0)
        )
    }
}

pub struct WindowBlocker<P: WindowBlockPolicy> {
    conn: xcb::Connection,
    policy: P,
}

impl<P: WindowBlockPolicy> WindowBlocker<P> {
    pub fn new(policy: P) -> Result<Self, Box<dyn Error>> {
        let (conn, screen_num) = xcb::Connection::connect(None)?;
        let screen = conn.get_setup().roots().nth(screen_num as usize).ok_or("No screen")?;
        let root = screen.root();

        let values = [x::Cw::EventMask(x::EventMask::SUBSTRUCTURE_NOTIFY)];
        conn.send_request(&x::ChangeWindowAttributes { window: root, value_list: &values });
        conn.flush()?;

        Ok(Self { conn, policy })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        loop {
            self.conn.flush()?;
            if let xcb::Event::X(x::Event::MapNotify(map_event)) = self.conn.wait_for_event()? {
                let window = map_event.window();
                if self.policy.should_block(&self.conn, window)? {
                    self.conn.send_request(&x::UnmapWindow { window });
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let blocker = WindowBlocker::new(WallpaperEngineBorderBlockPolicy)?;
    blocker.run()
}
