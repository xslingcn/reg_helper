use thirtyfour::By;

use crate::{
    config::{Switch, CONFIG},
    error::{RegError, RegResult},
    register::{match_section, parse_status},
    webdrive,
};

pub fn match_switch(sln: &str) -> Option<(&String, &Switch)> {
    for (name, switch) in CONFIG.switch.iter() {
        if switch.to_add == sln.parse::<u32>().unwrap() {
            return Some((name, switch));
        }
    }
    None
}

pub(crate) async fn switch(switch: &Switch) -> RegResult<String> {
    let driver = webdrive::create_switch_webdriver().await?;

    // drop classes
    for sln in switch.to_drop.iter() {
        let checkbox_xpath = format!(
            "//tr[td/tt[contains(text(), '{}')]]/td/input[@type='CHECKBOX']",
            sln
        );
        if let Ok(checkbox) = driver.find(By::XPath(&checkbox_xpath)).await {
            checkbox.click().await?;
            println!("Dropping {} selected", sln);
        } else {
            println!("Could not find SLN to drop: {}", sln);
        }
    }

    // add class
    let sln_input_xpath = "//input[contains(@name,'sln') and not(@type='HIDDEN') and @value='']";
    let sln_inputs = driver.find_all(By::XPath(&sln_input_xpath)).await?;

    if sln_inputs.len() < 2 {
        Err(RegError::RegFailedError("Could not find SLN inputs".to_string()))?
    }
    if let Some((name, section)) = match_section(&switch.to_add.to_string()).await {
        sln_inputs[0].send_keys(section.lecture_sln.to_string()).await?;
        sln_inputs[1].send_keys(section.section_sln.to_string()).await?;
        println!("Switching to {}", name);
    } else {
        sln_inputs[0].send_keys(switch.to_add.to_string()).await?;
        println!("Switching to SLN: {}", switch.to_add);
    }

    // submit
    let submit_button = driver.find(By::XPath("//input[@type='submit']")).await?;
    submit_button.click().await?;

    let body = driver.source().await?;

    if body.contains("Schedule updated.") {
        Ok("Schedule updated.".to_string())
    } else {
        let reason = parse_status(&body)?;
        Err(RegError::RegFailedError(reason))?
    }
}
