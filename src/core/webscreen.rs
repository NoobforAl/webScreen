use std::error::Error;
use std::fs;
use std::time::Duration;

use headless_chrome::protocol;
use headless_chrome::Browser;
use protocol::cdp::Page;

use Page::CaptureScreenshotFormatOption;

pub fn web_screen(
    url: &str,
    timeout: u64,
    format: &str,
    quality: u32,
) -> Result<Vec<u8>, Box<dyn Error>> {
    log::debug!("create browser and new tab");
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    log::info!("take screenshot with time out {}, URL: {}", timeout, url);

    let jpeg_data = tab
        .navigate_to(url)?
        .set_default_timeout(Duration::from_secs(timeout))
        .wait_until_navigated()?
        .capture_screenshot(getformate(format), Some(quality), None, true)?;

    log::info!("web screenshot done");
    Ok(jpeg_data)
}

fn getformate(format: &str) -> CaptureScreenshotFormatOption {
    log::debug!("choose format file.");
    match format {
        "png" => return CaptureScreenshotFormatOption::Png,
        "jpeg" => return CaptureScreenshotFormatOption::Jpeg,
        _ => return CaptureScreenshotFormatOption::Png,
    }
}

#[allow(dead_code)]
pub fn save_img(path_file: &str, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
    log::debug!("save file :{}", path_file);
    fs::write(path_file, data)?;
    Ok(())
}
