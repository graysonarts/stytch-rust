// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::discovery::Discovery;
use crate::b2b::magic_links::MagicLinks;
use crate::b2b::oauth::OAuth;
use crate::b2b::organizations::Organizations;
use crate::b2b::otp::OTPs;
use crate::b2b::passwords::Passwords;
use crate::b2b::rbac::RBAC;
use crate::b2b::recovery_codes::RecoveryCodes;
use crate::b2b::scim::SCIM;
use crate::b2b::sessions::Sessions;
use crate::b2b::sso::SSO;
use crate::b2b::totps::TOTPs;
use crate::consumer::m2m::M2M;
use crate::consumer::project::Project;

pub struct Client {
    pub discovery: Discovery,
    pub m2m: M2M,
    pub magic_links: MagicLinks,
    pub oauth: OAuth,
    pub otps: OTPs,
    pub organizations: Organizations,
    pub passwords: Passwords,
    pub project: Project,
    pub rbac: RBAC,
    pub recovery_codes: RecoveryCodes,
    pub scim: SCIM,
    pub sso: SSO,
    pub sessions: Sessions,
    pub totps: TOTPs,
}

impl Client {
    pub fn new(project_id: &str, secret: &str) -> crate::Result<Self> {
        Ok(Client::new_with_http_client(
          crate::client::Client::new_b2b(project_id, secret)?,
        ))
    }

    pub fn new_with_http_client(http_client: crate::client::Client) -> Self {
        Client {
            discovery: Discovery::new(http_client.clone()),
            m2m: M2M::new(http_client.clone()),
            magic_links: MagicLinks::new(http_client.clone()),
            oauth: OAuth::new(http_client.clone()),
            otps: OTPs::new(http_client.clone()),
            organizations: Organizations::new(http_client.clone()),
            passwords: Passwords::new(http_client.clone()),
            project: Project::new(http_client.clone()),
            rbac: RBAC::new(http_client.clone()),
            recovery_codes: RecoveryCodes::new(http_client.clone()),
            scim: SCIM::new(http_client.clone()),
            sso: SSO::new(http_client.clone()),
            sessions: Sessions::new(http_client.clone()),
            totps: TOTPs::new(http_client.clone()),
        }
    }
}
