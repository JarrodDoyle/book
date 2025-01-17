/* ANCHOR: all */
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

struct AppModel {
    counter: u8,
}

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}

// ANCHOR: macro
#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = u8;

    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 100,

            // ANCHOR: widget_assign
            gtk::Box {
            // ANCHOR_END: widget_assign
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Increment",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Increment);
                    }
                },

                // ANCHOR: widget_assign_fn
                gtk::Button::with_label("Decrement") {
                // ANCHOR_END: widget_assign_fn
                // ANCHOR: connect
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Decrement);
                    }
                // ANCHOR_END: connect
                },

                gtk::Label {
                    // ANCHOR: watch
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter),
                    // ANCHOR_END: watch
                    set_margin_all: 5,
                }
            }
        }
    }

    // Initialize the UI.
    fn init(
        counter: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel { counter };

        // ANCHOR: view_output
        // Insert the macro code generation here
        let widgets = view_output!();
        // ANCHOR_END: view_output

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}
// ANCHOR_END: macro

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    app.run::<AppModel>(0);
}
/* ANCHOR_END: all */
