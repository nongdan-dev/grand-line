use crate::prelude::*;

static DEFAULT: LazyLock<AuthzConfig> = LazyLock::new(AuthzConfig::default);

pub trait AuthzConfigContext<'a> {
    fn authz_config(&self) -> &'a AuthzConfig;
}

impl<'a> AuthzConfigContext<'a> for Context<'a> {
    fn authz_config(&self) -> &'a AuthzConfig {
        if let Some(cfg) = self.data_opt::<AuthzConfig>() {
            cfg
        } else {
            &DEFAULT
        }
    }
}

pub trait AuthzOrgImplContext<'a> {
    fn authz_org_impl(&self) -> Res<&'a Box<dyn AuthzOrgImpl>>;
}

impl<'a> AuthzOrgImplContext<'a> for Context<'a> {
    fn authz_org_impl(&self) -> Res<&'a Box<dyn AuthzOrgImpl>> {
        self.data_opt::<Box<dyn AuthzOrgImpl>>()
            .ok_or_else(|| MyErr::OrgImplNotFound.into())
    }
}
