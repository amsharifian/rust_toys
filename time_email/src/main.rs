extern crate chrono;
extern crate lettre;
extern crate lettre_email;
extern crate mime;


use chrono::prelude::*;
use chrono::Duration;
use std::thread::sleep;
use std::time;

use lettre::{SmtpClient, Transport};
use lettre_email::Email;

use lettre::smtp::authentication::IntoCredentials;

mod logging;

fn send_email(email_msg: &str){

    let smtp_address = "smtp.gmail.com";
    let user_name = "am.sharifian@gmail.com";
    let pass = "fujaibuoymyjruyg";

    let email = Email::builder()
    // Addresses can be specified by the tuple (email, alias)
    .to(("amiralis@sfu.ca", "Amirali Sharifian"))
    // ... or by an address only
    .from((user_name, "Amirali Sharifian"))
    .subject("[Dandelion-AWS] Update")
    .text(email_msg)
    .build()
    .unwrap();

    let credentials = (user_name, pass).into_credentials();

    // Open a local connection on port 25
    // let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();
    let mut client = SmtpClient::new_simple(smtp_address)
    .unwrap()
    .credentials(credentials)
    .transport();

    // Send the email
    let result = client.send(email.into());

    if result.is_ok() {
        log::info!("Email sent to {}", user_name);
    } else {
        log::info!("Could not send email: {:?}", result);
    }

    assert!(result.is_ok());
}


/**
 * Check if system is running for a custom period
 */
fn check_run(){
    let start: DateTime<Local> = Local::now();
    let mut next = start + Duration::hours(10);
    let message = "The amazon instance is on, please consider to turn off the machine if the system is IDLE!";

    loop{
        sleep(time::Duration::from_secs(4 * 60 * 60));
        if Local::now() > next {
            log::info!("Sent an update email!");
            send_email(message);
            next = Local::now() + Duration::hours(10);
        }

    }
}

fn main() {
    logging::log_setup();
    check_run();
    log::info!("Sent");

}
