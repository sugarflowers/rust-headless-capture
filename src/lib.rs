use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser, LaunchOptions};
use headless_chrome::protocol::cdp::Target::CreateTarget;
use anyhow::Result;
use std::fs;

pub fn capture() -> Result<()> {


    let options = LaunchOptions::default_builder()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");


    let browser = Browser::new(options)?;
    
    let tab = browser.new_tab_with_options(CreateTarget {
        url: "".to_string(),
        width: Some(1280),
        height: Some(800),
        browser_context_id: None,
        enable_begin_frame_control: None,
        background: None,
        new_window: None,
    })?;

    let png_data = tab
        .navigate_to("http://USER:PASS@ADDR/")?
        .wait_until_navigated()?
        .capture_screenshot(CaptureScreenshotFormatOption::Png, Some(80), None, true)?;

    fs::write("screenshot.png", png_data)?;

    Ok(())
}

