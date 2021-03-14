use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    date: String,
    label: String,
}
fn main() {
    let json_data: Vec<Todo> = get_data();
    // println!("Json Data = {:?}", json_data[0]["date"]);
    let application =
        Application::new(Some("com.github.prajwalprabhu.todo-rs"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(move |app| {
        let data = &json_data;
        let window = ApplicationWindow::new(app);
        let mut labels: Vec<gtk::Label> = Vec::new();
        window.set_title("Todo");
        window.set_default_size(350, 70);
        let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
        for i in data {
            let output = format!("| Date : {:?} | Label : {:?} |", i.date, i.label);

            let mut output2 = String::new();
            for _ in 0..output.len() {
                output2.push('_');
            }
            let label = gtk::Label::new(Some(&output));
            let label2 = gtk::Label::new(Some(&output2));
            layout.add(&label);
            layout.add(&label2);
            labels.push(label);
            labels.push(label2);
        }
        let new_button = gtk::Button::with_label("New");
        let remove_button = gtk::Button::with_label("Remove");
        // let json_data2 = &data;
        new_button.connect_clicked(|_| {
            println!("New ");
            let toplevel = gtk::Window::new(gtk::WindowType::Toplevel);
            let message = gtk::Label::new(Some("Enter Label"));
            let message2 = gtk::Label::new(Some("Enter Date"));
            let label_entry = gtk::Entry::with_buffer(&gtk::EntryBuffer::new(None));
            let date_entry = gtk::Entry::with_buffer(&gtk::EntryBuffer::new(None));
            let create_button = gtk::Button::with_label("Create");
            let date_entry2 = date_entry.clone();
            let label_entry2 = label_entry.clone();

            create_button.connect_clicked(move |_| {
                let date = date_entry2.get_text().to_string();
                let label_ = label_entry2.get_text().to_string();
                let new = Todo {
                    label: label_,
                    date: date,
                };
                // data.push(new);
                println!("Clicked {:?}", new);
            });
            let box_ = gtk::Box::new(gtk::Orientation::Vertical, 0);
            box_.add(&message);
            box_.add(&label_entry);
            box_.add(&message2);
            box_.add(&date_entry);
            box_.add(&create_button);
            toplevel.add(&box_);
            toplevel.show_all();
        });
        let button_box = gtk::Box::new(gtk::Orientation::Vertical, 1);
        button_box.add(&new_button);
        button_box.add(&remove_button);
        layout.add(&button_box);
        window.add(&layout);

        window.show_all();
    });

    application.run(&[]);

    // run();
}
fn get_data() -> Vec<Todo> {
    let data = File::open("/tmp/data.json");
    let mut json_string = String::new();
    match data {
        Ok(mut file) => {
            file.read_to_string(&mut json_string)
                .expect("Failed to read data");
        }
        Err(e) => {
            println!("Error {:?}", e);
            use std::fs::DirBuilder;
            let path = "/tmp";
            DirBuilder::new().recursive(true).create(path).unwrap();
            let mut new_file: File = File::create("/tmp/data.json").expect("Failed to creat file");
            new_file
                .write_all(
                    br#"[	
	{
		"label":"Test",
		"date":"Test"

	}

	]"#,
                )
                .expect("Failed to write");
            new_file.sync_all().expect("Failed");
            return get_data();
        }
    }
    let json_data: Vec<Todo> =
        serde_json::from_str(&json_string).expect("Failed to read Json data");
    json_data
}
