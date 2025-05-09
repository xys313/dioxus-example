use crate::SharedRoutes;
use dioxus::prelude::*;
use ui::Navbar;

#[component]
pub fn SharedNavbar() -> Element {
    rsx! {
        Navbar {
            Link {
                to: SharedRoutes::Home {},
                "Home"
            }
            Link {
                to: SharedRoutes::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<SharedRoutes> {}
    }
}
