use std::{fs, thread};

use serde::export::fmt::Error;
use web_view::{Content, Handle, WebView, WVResult};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn get_layout() -> Result<String, Error> {
    const PATH: &str = "layout.json";

    let contents: String = fs::read_to_string(PATH).expect("Something went wrong reading the file!");

    serde_json::from_str(contents.as_str())?
}

struct Gui {
    sender: Sender<Update>
}

impl Gui {
    pub(crate) fn new() -> Gui {
        let (sender, receiver) = channel::<Update>();
        thread::spawn(move || {
            build(receiver);
        });
        Gui{sender}
    }

    pub(crate) fn get_sender(&self) -> Sender<Update> {
        return self.sender.clone();
    }


    fn build(receiver: Receiver<Update>) {
        let html = get_html();
        let mut counter = 0;
        let receiver = Arc::new(Mutex::new(receiver));
        let receiver_inner = receiver.clone();

        let mut webview = web_view::builder()
            .title("Rust Todo App")
            .content(Content::Html(html))
            //.size(320, 480)
            .resizable(true)
            //.debug(true)
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
                if counter == 0 {
                    init_layout(counter, webview);
                }

                webview.set_title(&format!("Rust Todo App ({} Tasks)", tasks_len))?;

                render(webview)
            })
            .build()
            .unwrap();

        webview.set_color((156, 39, 176));
        let handle = webview.handle();
        thread::spawn(move || loop {
            {
                let mut receiver = receiver_inner.lock().unwrap();
                let msg = receiver.recv().unwrap();
                handle
                    .dispatch(move |webview| {
                        *webview.user_data_mut() -= 1;
                        webview.eval(&serde_json::to_string(&msg).unwrap())
                    })
                    .unwrap();
            }
            // thread::sleep(Duration::from_secs(1));
        });


        let res = webview.run().unwrap();
        println!("final state: {:?}", res);
    }
}

fn init_layout(mut counter: i32, webview: &mut WebView<Vec<Task>>) {
    webview.eval(&format!("app.sendLayout({})", layout));
    counter += 1;
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

/// generic Update struct
#[derive(Debug, Serialize, Deserialize)]
struct Update {
    min: i32,
    max: i32,
    value: i32,
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

fn get_html() -> String {
    format!(
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
    )
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}