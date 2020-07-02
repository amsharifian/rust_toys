extern crate chrono;
extern crate lettre;
extern crate lettre_email;
extern crate mime;
extern crate log;
extern crate simple_logging;

use chrono::prelude::*;
use chrono::Duration;
use std::thread::sleep;
use std::time;

use lettre::{SmtpClient, Transport};
use lettre_email::Email;
use std::path::Path;

use lettre::smtp::authentication::IntoCredentials;
use log::{info, trace, warn};
use log::LevelFilter;
use log::Level;


// use simple_logging::log::LevelFilter;

fn log(){
    // simple_logging::init().unwrap();
    simple_logging::log_to_file("/tmp/dandelion_manage.log", LevelFilter::Info);
    info!("This will be logged.");


}


fn sendEmail(){

    env_logger::init();
    let smtp_address = "smtp.gmail.com";
    let user_name = "am.sharifian@gmail.com";
    let pass = "fujaibuoymyjruyg";

    let email = Email::builder()
    // Addresses can be specified by the tuple (email, alias)
    .to(("amiralis@sfu.ca", "Amirali Sharifian"))
    // ... or by an address only
    .from((user_name, "Amirali Sharifian"))
    .subject("Test Email")
    .text("Hello world.")
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
    println!("Email sent");
    } else {
    println!("Could not send email: {:?}", result);
    }

    assert!(result.is_ok());
}

fn main() {
    let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
    let tomorrow = local + Duration::days(1);

    // println!("Hour: {}", local.time().hour12().1);
    println!("Today: {}", local.to_string());
    println!("Tomorrow: {}", tomorrow.to_string());

    let mut cnt = 0;
    log();
    // sendEmail();
    // loop{
    //     sleep(time::Duration::new(2,0));
    //     if cnt == 5 {   
    //         break;
    //     }
    //     else{
    //         test();
    //         cnt += 1;
    //     }
    // }

}
