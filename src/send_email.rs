use crate::email::Email;
use anyhow::Context;
use rusoto_ses::{Body, Content, Destination, Message, SendEmailRequest, Ses, SesClient};

pub async fn send_mock_email(email: Email) -> Result<(), anyhow::Error> {
    print!("Send email with mock client: {:#?}", email);
    Ok(())
}

pub async fn send_real_email(ses_client: &SesClient, email: Email) -> Result<(), anyhow::Error> {
    let destination = Destination {
        to_addresses: Some(vec![email.receiver]),
        bcc_addresses: None,
        cc_addresses: None,
    };
    let message = Message {
        subject: Content {
            charset: None,
            data: email.subject,
        },
        body: Body {
            text: Some(Content {
                data: email.plaintext,
                charset: None,
            }),
            html: Some(Content {
                data: email.html,
                charset: None,
            }),
        },
    };
    let request = SendEmailRequest {
        source: email.sender,
        destination,
        message,
        ..Default::default()
    };
    let _response = ses_client
        .send_email(request)
        .await
        .context("Can't send mail")?;
    Ok(())
}
