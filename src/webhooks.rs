use serenity::builder::{
    CreateAttachment, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, ExecuteWebhook,
};
use serenity::http::Http;
use serenity::model::webhook::Webhook;
use serenity::model::Colour;
use std::path::Path;

pub async fn send_webhook(id: &str, token: &str, file: &str, username : &str, msg : &str) {
    // You don't need a token when you are only dealing with webhooks.
    let http: Http = Http::new("");
    let url: String = format!("https://discord.com/api/webhooks/{}/{}", id, token);

    let webhook: Webhook = Webhook::from_url(&http, &url)
        .await
        .expect("Replace the webhook with your own");
    let embed: CreateEmbed = CreateEmbed::new()
        //.title("title")
        .color(Colour::BLUE)
        .author(
            CreateEmbedAuthor::new("marioANTUNES")
                .icon_url("https://i.imgur.com/k6AZJWr.png")
                .url("https://multibot.run/"),
        )
        //.description("This is an example")
        .fields(vec![
            (":loudspeaker::point_down:", msg, false),
        ])
        .footer(
            CreateEmbedFooter::new("marioANTUNES @ 2024")
                .icon_url("https://i.imgur.com/5ij0FTh.png"),
        );

    // https://github.com/serenity-rs/serenity/blob/current/examples/e17_message_components/src/main.rs
    // let emb: CreateEmbed = embed.clone().field("33333", "3333", false);
    let emb: CreateEmbed = embed.clone();

    let builder: ExecuteWebhook = ExecuteWebhook::new()
            //.content("hello there")
            .username(username)
            .embed(emb)
            .avatar_url("https://i.imgur.com/wCbH4UN.png");

    if Path::new(file).exists() {
        let file_path: CreateAttachment = CreateAttachment::path(file).await.unwrap();

        let builder2: ExecuteWebhook = builder.clone()
            .add_file(file_path);

        webhook
            .execute(&http, false, builder2)
            .await
            .expect("Could not execute webhook.");
    } else {

        webhook
            .execute(&http, false, builder)
            .await
            .expect("Could not execute webhook.");
    }
}
