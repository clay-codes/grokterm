use dioxus::prelude::*;

fn main() {
    // Init debug
    launch(app);
}

pub fn app() -> Element {
    // Build cool things ✌️
    // All events and their data implement Debug, so we can re-cast them as Rc<dyn Debug> instead of their specific type
    let mut usr_input = use_signal(|| "".to_string());
    //let mut future = use_resource(|| async move {});
    //let onenter = use_resource(move || async move { usr_input().read().await });
    let base_url = "http://localhost:8081/"; // Change this to your production URL as needed

    let send = move |_| {
        spawn(async move {
            let full_url = format!("{}{}", base_url, usr_input.to_string());
            let resp = reqwest::Client::new()
                .get(&full_url) // Use full_url that includes user input
                .send()
                .await;

            match resp {
                Ok(_data) => {
                    log::info!("dioxuslabs.com responded!");
                }
                Err(err) => {
                    log::info!("Request failed with error: {err:?}")
                }
            }
        });
    };
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div { id: "terminal",
        div { id: "output" }
        div { id: "input-line",
            span { "guest@grokterm:~$" }
            input { r#type: "text",
            id: "terminal-input",
            autofocus: true,
            // event on type "ssh"
            value: "{usr_input}",
            oninput: move |evt| usr_input.set(evt.value()),
            onkeydown: move |evt| {
                if evt.key()== Key::Enter {
                    //flag.set("check".into());
                    match usr_input.to_string().as_str() {
                        "vault" => {
                            send(evt);
                            usr_input.set("routing to vault".into());
                        }
                        _ => {
                            usr_input.set("invalid command".into());
                        }
                    }
                }
            },
        }
        }
        }
    }
}