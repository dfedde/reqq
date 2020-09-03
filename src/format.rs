use reqwest::blocking::Response;
use anyhow::Result;

// TODO: Look at the content-type header and attempt to parse based on content.
pub fn format_response(resp: Response, raw: bool) -> Result<String> {
    if raw {
        Ok(format!("{}", resp.text()?))
    } else {
        let status = resp.status();

        let header_lines: Vec<String> = resp.headers().iter().map(|(k, v)| {
            format!("{}: {}", k, v.to_str().unwrap())
        }).collect();

        let body = resp.text()?;

        Ok(format!("{}\n{}\n{}", status.as_str(), header_lines.join("\n"), body))
    }
}