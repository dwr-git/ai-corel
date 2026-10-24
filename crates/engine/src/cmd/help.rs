//! Help links. AI-Corel works offline from a folder on the user's own server, so it sends nobody to
//! outside sites: there are no link commands (the original project's Discord, website and GitHub
//! links were removed). `help.links` stays for callers that ask, and answers with no links.

use serde_json::json;

use super::*;

pub fn specs() -> Vec<CommandSpec> {
    vec![cmd!(query "help.links", "Links", [], None, "{} → {} (no outside links: AI-Corel works offline)", always, |_, _| Ok(json!({})))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_outside_links() {
        let mut s = Session::new();
        for id in ["help.discord", "help.website", "help.appPage", "help.github"] {
            assert!(s.execute(id, &json!({})).is_err(), "{id}");
        }
        assert_eq!(s.execute("help.links", &json!({})).unwrap(), json!({}));
    }
}
