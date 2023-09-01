use slack::RtmClient;
use std::panic;

struct Handler;

#[allow(unused_variables)]
impl slack::EventHandler for Handler {
    fn on_event(&mut self, cli: &RtmClient, event: slack::Event) {
        println!("on event: {:?}", event);
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("Slack Bot has DISCONNECTED");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("Slack Bot is CONNECTED");
        let _ = cli
            .sender()
            .send_message("burrow-crashes", "BOT HAS CONNECTED");
    }
}

fn slack_auth() {
    let token = "xoxe.xoxp-1-Mi0yLTIyMTA1MzU1NjUtNTE3MDMwODYwMTc0Ny01ODI5MTU3NzgzMzQ4LTU4MTIxNzk3NzM3NjctZGQ4MGQ0ODljNmM3NDg0ZWZhNmE0YTRiM2E3OTg5MzA3NDk2ZjA1NmE1MzZkZTNlNTczZjdmMTc0NzQ2MzVjZA";
    let mut handler = Handler;
    let client = RtmClient::login_and_run(&token, &mut handler);

    match client {
        Ok(_) => {}
        Err(err) => println!("Error: {}", err),
    }
}

fn custom_panic_hook(info: &panic::PanicInfo) {
    println!("{info}");
}

pub async fn custon_panic() {
    panic::set_hook(Box::new(custom_panic_hook));
    slack_auth();

    panic!("Normal panic");
}
