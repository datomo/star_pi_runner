// #[macro_use]
// extern crate serde_derive;
extern crate serde_json;
// extern crate web_view;

// use web_view::*;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::workflow::SensorStatus;

/// struct should hold all available ?sensors? and expose them to the GUI
pub struct GuiManager {
    sender: Sender<SensorStatus>,
    receiver: Arc<Mutex<Receiver<SensorStatus>>>,
}

impl GuiManager {
    pub(crate) fn new(has_gui: bool) -> Self {
        let (sender, receiver) = channel::<SensorStatus>();
        let gui = GuiManager { sender, receiver: Arc::new(Mutex::new(receiver)) };
        gui.loop_debug();
        if has_gui {}
        println!("starting gui");
        gui
    }

    pub(crate) fn get_sender(&self) -> Sender<SensorStatus> {
        self.sender.clone()
    }

    fn loop_debug(&self) {
        let local_sender = self.receiver.clone();
        thread::spawn(move || loop {
            let msg = local_sender.lock().unwrap().recv().unwrap();
            println!("{}", msg);
        });
    }
}

/*
pub(crate) fn build() {
    let html = format!(
        r#"
		<!doctype html>
		<html>
			<head>
				{styles}
			</head>
			<body>
				<!--[if lt IE 9]>
				<div class="ie-upgrade-container">
					<p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
					<a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
				</div>
				<![endif]-->
				<!--[if gte IE 9 | !IE ]> <!-->
				<div id="app"></div>
				{scripts}
				<![endif]-->
			</body>
		</html>
		"#,
        styles = inline_style(include_str!("gui/build/app.css")),
        scripts = inline_script(include_str!("gui/build/app.js"))
    );

    let mut webview = web_view::builder()
        .title("Rust Todo App")
        .content(Content::Html(html))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(vec![])
        .invoke_handler(|webview, arg| {
            use Cmd::*;

            let tasks_len = {
                let tasks = webview.user_data_mut();

                match serde_json::from_str(arg).unwrap() {
                    Init => (),
                    Log { text } => println!("{}", text),
                    AddTask { name } => tasks.push(Task { name, done: false }),
                    MarkTask { index, done } => tasks[index].done = done,
                    ClearDoneTasks => tasks.retain(|t| !t.done),
                }

                tasks.len()
            };

            webview.set_title(&format!("Rust Todo App ({} Tasks)", tasks_len))?;
            render(webview)
        })
        .build()
        .unwrap();

    webview.set_color((156, 39, 176));

    let res = webview.run().unwrap();

    println!("final state: {:?}", res);
}


fn render(webview: &mut WebView<Vec<Task>>) -> WVResult {
    let render_tasks = {
        let tasks = webview.user_data();
        println!("{:#?}", tasks);
        format!("app.fromRust({})", serde_json::to_string(tasks).unwrap())
    };
    webview.eval(&render_tasks)
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    done: bool,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init,
    Log { text: String },
    AddTask { name: String },
    MarkTask { index: usize, done: bool },
    ClearDoneTasks,
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

 */