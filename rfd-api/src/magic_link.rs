// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use async_trait::async_trait;
use minijinja::{context, Environment};
use reqwest::Url;
use resend_rs::{types::CreateEmailBaseOptions, Resend};
use v_api::{
    messenger::{Message, Messenger, MessengerError},
    MagicLinkMessage,
};

pub struct MagicLinkMessageBuilder {
    pub env: Environment<'static>,
}

impl MagicLinkMessage for MagicLinkMessageBuilder {
    fn create_message(&self, recipient: &str, token: &str, url: &Url) -> Option<Message> {
        let subject = self.env.get_template("subject").ok();
        let text = self.env.get_template("text").ok();
        let html = self.env.get_template("html").ok();

        if let (Some(subject), Some(text)) = (subject, text) {
            Some(Message {
                recipient: recipient.to_string(),
                subject: subject
                    .render(context! {
                        recipient => recipient,
                        url => url,
                    })
                    .ok(),
                text: text
                    .render(context! {
                        recipient => recipient,
                        token => token,
                        url => url,
                    })
                    .ok()?,
                html: html.and_then(|html| {
                    html.render(context! {
                        recipient => recipient,
                        token => token,
                        url => url,
                    })
                    .ok()
                }),
            })
        } else {
            None
        }
    }
}

pub struct ResendMagicLink {
    client: Resend,
    from: String,
}

impl ResendMagicLink {
    pub fn new(key: String, from: String) -> Self {
        Self {
            client: Resend::new(&key),
            from,
        }
    }
}

#[async_trait]
impl Messenger for ResendMagicLink {
    async fn send(&self, message: Message) -> Result<(), MessengerError> {
        let mut email = CreateEmailBaseOptions::new(
            &self.from,
            [&message.recipient],
            message.subject.unwrap_or_default(),
        );
        email = email.with_text(&message.text);

        if let Some(html) = &message.html {
            email = email.with_html(html);
        }

        self.client.emails.send(email).await?;

        Ok(())
    }
}
