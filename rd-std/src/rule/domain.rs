use std::convert::TryFrom;

use super::config::{DomainMatcher, DomainMatcherMethod as Method};
use super::matcher::{Matcher, MaybeAsync};
use anyhow::Result;
use rd_interface::Address;

impl TryFrom<String> for Method {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_ref() {
            "keyword" => Method::Keyword,
            "suffix" => Method::Suffix,
            "match" => Method::Match,
            _ => return Err(anyhow::anyhow!("Unsupported method: {}", value)),
        })
    }
}

impl DomainMatcher {
    fn test(&self, domain: &str) -> bool {
        match self.method {
            Method::Keyword => domain.contains(&self.domain),
            Method::Match => domain == &self.domain,
            Method::Suffix => domain.ends_with(&self.domain),
        }
    }
}

impl Matcher for DomainMatcher {
    fn match_rule(&self, _ctx: &rd_interface::Context, addr: &Address) -> MaybeAsync<bool> {
        match addr {
            Address::Domain(domain, _) => self.test(domain),
            // if it's not a domain, pass it.
            _ => false,
        }
        .into()
    }
}
