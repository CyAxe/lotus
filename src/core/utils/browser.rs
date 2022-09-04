use thirtyfour_sync::prelude::*;
use thirtyfour_sync::ChromeCapabilities;

pub struct Browser {
    pub driver_url: String,
    caps: ChromeCapabilities,
}

impl Browser {
    pub fn init(driver_url: String) -> Browser {
        let mut caps = DesiredCapabilities::chrome();
        caps.set_headless().unwrap();
        caps.set_ignore_certificate_errors().unwrap();
        caps.set_disable_web_security().unwrap();
        Browser { driver_url, caps }
    }

    pub fn open(&self, url: &str) -> WebDriverResult<()> {
        let driver = WebDriver::new(self.driver_url.as_str(), &self.caps)?;

        // Navigate to https://wikipedia.org.
        driver.get(url)?;
        let elem_form = driver.find_element(By::Id("search-form"))?;

        // Find element from element.
        let elem_text = elem_form.find_element(By::Id("searchInput"))?;

        // Type in the search terms.
        elem_text.send_keys("selenium")?;

        // Click the search button.
        let elem_button = elem_form.find_element(By::Css("button[type='submit']"))?;
        elem_button.click()?;

        // Look for header to implicitly wait for the page to load.
        driver.find_element(By::ClassName("firstHeading"))?;
        assert_eq!(driver.title()?, "Selenium - Wikipedia");

        // Close the browser.
        driver.quit()?;
        Ok(())
    }
}
