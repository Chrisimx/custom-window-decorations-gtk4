// Copyright © 2025 Christian Nagel
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the “Software”), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies
// or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
// INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE
// AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use gtk::prelude::*;
use relm4::gtk::gdk::{Cursor, Display, SurfaceEdge};
use relm4::gtk::{gdk, CssProvider};
use relm4::prelude::*;

struct App {
    counter: u8,
}

#[derive(Debug)]
enum Msg {
    Increment,
    Decrement,
}

const BORDER_SIZE: f64 = 20f64;

fn select_edge(
    x: f64,
    y: f64,
    actual_window_width: f64,
    actual_window_height: f64,
) -> Option<SurfaceEdge> {
    let edge = if x < 0.0 && y < 0.0 {
        Some(SurfaceEdge::NorthWest)
    } else if x > actual_window_width && y < 0.0 {
        Some(SurfaceEdge::NorthEast)
    } else if x < 0.0 && y > actual_window_height {
        Some(SurfaceEdge::SouthWest)
    } else if x > actual_window_width && y > actual_window_height {
        Some(SurfaceEdge::SouthEast)
    } else if x < 0.0 {
        Some(SurfaceEdge::West)
    } else if x > actual_window_width {
        Some(SurfaceEdge::East)
    } else if y < 0.0 {
        Some(SurfaceEdge::North)
    } else if y > actual_window_height {
        Some(SurfaceEdge::South)
    } else {
        None
    };
    edge
}
fn cursor_name_from_edge(edge: &Option<SurfaceEdge>) -> &str {
    match edge {
        Some(SurfaceEdge::NorthWest) => "nw-resize",
        Some(SurfaceEdge::NorthEast) => "ne-resize",
        Some(SurfaceEdge::SouthWest) => "sw-resize",
        Some(SurfaceEdge::SouthEast) => "se-resize",
        Some(SurfaceEdge::West) => "w-resize",
        Some(SurfaceEdge::East) => "e-resize",
        Some(SurfaceEdge::North) => "n-resize",
        Some(SurfaceEdge::South) => "s-resize",
        Some(_) => "default",
        None => "default",
    }
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = u8;
    type Input = Msg;
    type Output = ();

    view! {
        #[name = "window"]
        gtk::Window {
            set_title: Some("Demo app"),
            set_default_size: (300, 200),
            set_decorated: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    gtk::WindowControls {
                        set_side: gtk::PackType::Start,
                    },
                    gtk::WindowControls {
                        set_side: gtk::PackType::End,
                    },
                    gtk::WindowHandle {
                        set_hexpand: true,
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    set_margin_all: 5,

                    gtk::Button {
                        set_label: "Increment",
                        connect_clicked => Msg::Increment,
                    },

                    gtk::Button {
                        set_label: "Decrement",
                        connect_clicked => Msg::Decrement,
                    },

                    gtk::Label {
                        #[watch]
                        set_label: &format!("Counter: {}", model.counter),
                        set_margin_all: 5,
                    }
                }
            }
        }
    }

    // Initialize the component.
    fn init(
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App { counter };

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        let window = &widgets.window;

        let window_clone1 = window.clone();

        let motion_controller = gtk::EventControllerMotion::new();
        motion_controller.connect_motion(move |controller, x, y| {
            let surface = window_clone1.native().unwrap().surface().unwrap();
            let toplevel = surface.downcast_ref::<gdk::Toplevel>().unwrap();

            let width = toplevel.width() as f64;
            let height = toplevel.height() as f64;

            let actual_window_width = width - BORDER_SIZE * 2.0;
            let actual_window_height = height - BORDER_SIZE * 2.0;

            let edge = select_edge(x, y, actual_window_width, actual_window_height);

            let cursor_name = cursor_name_from_edge(&edge);

            let cursor = Cursor::from_name(cursor_name, None);

            surface.set_cursor(cursor.as_ref())
        });

        let gesture_click = gtk::GestureClick::new();
        let window_clone2 = window.clone();
        gesture_click.connect_pressed(move |event_controller, button, x, y| {
            let surface = window_clone2.native().unwrap().surface().unwrap();
            let toplevel = surface.downcast_ref::<gdk::Toplevel>().unwrap();

            let width = toplevel.width() as f64;
            let height = toplevel.height() as f64;

            let actual_window_width = width - BORDER_SIZE * 2.0;
            let actual_window_height = height - BORDER_SIZE * 2.0;

            let edge = select_edge(x, y, actual_window_width, actual_window_height);

            if edge.is_none() {
                return;
            }

            toplevel.begin_resize(
                edge.unwrap(),
                event_controller.device().as_ref(),
                button,
                x,
                y,
                event_controller.current_event_time(),
            )
        });

        apply_css();

        window.add_controller(motion_controller);
        window.add_controller(gesture_click);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            Msg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}

fn apply_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("../assets/style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    let app = RelmApp::new("relm4.example.simple");
    app.run::<App>(0);
}