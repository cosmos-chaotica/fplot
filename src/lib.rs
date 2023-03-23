extern crate gtk;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea,Menu, MenuBar, MenuItem, Label, IconSize, FileChooserAction,
FileChooserDialog, FileFilter, ResponseType};
use gtk::glib;
use gtk::gdk_pixbuf::{Pixbuf, Colorspace};
use gtk::prelude::GdkContextExt;

pub struct Surface {
    name: String,
    width: i32, /* pixel for Pixbuf*/
    height: i32,
    x_size: f64, /* float size for cairo */
    y_size: f64
}

impl Surface {
    pub fn new_with_pixel(name: String, width: i32, height: i32) -> Self {
        Self::new_with_pixel_size(name, width, height, 1.0)
    }

    pub fn new_with_pixel_size(name: String, width: i32, height: i32, x_size: f64) -> Self {
        let ratio = width as f64 / height as f64;
        let y_size = x_size / ratio;
        Surface {name, width, height, x_size, y_size}
    }

    pub fn new_with_size (name: String, x_size: f64, y_size: f64) -> Self {
        Self::new_with_size_width(name, x_size, y_size, 300)
    }

    pub fn new_with_size_width (name: String, x_size: f64, y_size: f64, width: i32) -> Self {
        let ratio = x_size / y_size;
        let height = (width as f64 / ratio) as i32;
        Surface {name, width, height, x_size, y_size}
    }

    pub fn draw_bytes(&self, bytes: Vec<u8>) {
        let application = Application::builder()
        .application_id("com.example.fplot")
        .build();

        let name = self.name.clone();
        let width = self.width;
        let height = self.height;

        application.connect_activate(move |app| {
            let name = name.clone();
            let bytes = bytes.clone();
            let window = ApplicationWindow::builder()
                .application(app)
                .title(name)
                .default_width(width)
                .default_height(height)
                .build();

            let drawing_area = DrawingArea::builder()
                .width_request(width)
                .height_request(height)
                .build();

            let pixbuf = Pixbuf::from_bytes(
                &glib::Bytes::from_owned(bytes),
                Colorspace::Rgb,
                false,
                8,
                width,
                height,
                width * 3
            );
            

    /*
            drawing_area.connect("expose_event", || {

            },
            pixbuf);
    */
            let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

            let menu = Menu::new();
            let menu_bar = MenuBar::new();
            let file = MenuItem::with_label("File");
            let save_item  = MenuItem::new();
            let save_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            let save_image = gtk::Image::from_icon_name(Some("folder-music-symbolic"), IconSize::Menu);
            let save_label = Label::new(Some("Save"));
            file.set_submenu(Some(&menu));
            save_box.pack_start(&save_image, false, false, 0);
            save_box.pack_start(&save_label, true, true, 0);
            menu_bar.append(&file);
            save_item.add(&save_box);
            menu.append(&save_item);
            v_box.pack_start(&menu_bar, false, false, 0);
            v_box.pack_start(&drawing_area, true, true, 0);
        
            let dialog = FileChooserDialog::new(Some("Save Image File"), Some(&window), FileChooserAction::Save);
            let filter = FileFilter::new();
            filter.add_mime_type("image/png");
            filter.set_name(Some("PNG image file"));
            dialog.add_filter(filter);
            dialog.add_button("Cancel", ResponseType::Cancel);
            dialog.add_button("Accept", ResponseType::Accept);
            dialog.connect_response(glib::clone!(@weak pixbuf => move |dialog, response| {
                let filename = dialog.filename().expect("Couldn't get filename");
                pixbuf.savev(filename, "png", &([] as [(&str, &str);0]));
                dialog.close();

            }));
            
            save_item.connect_activate( glib::clone!(@weak dialog=> move |_| {
                dialog.run();   
            }));

            drawing_area.connect_draw(move |widget, cairo_context| {
                cairo_context.set_source_pixbuf(&pixbuf, 0.0, 0.0);
                cairo_context.paint();
                cairo_context.stroke();
                Inhibit(true)
            });

            window.add(&v_box);
            
            window.show_all();
            
        });
        
        application.run();
    }


}