use crate::prelude::*;

static DEFAULT: LazyLock<AuthzConfig> = LazyLock::new(AuthzConfig::default);

pub trait AuthzConfigContext<'a>
where
    Self: CoreContext<'a>,
{
    fn authz_config(&self) -> &'a AuthzConfig {
        if let Some(cfg) = self.data_opt_impl::<AuthzConfig>() {
            cfg
        } else {
            &DEFAULT
        }
    }

    fn authz_err(&self) -> &'a GrandLineErr {
        &self.authz_config().unauthorized_err
    }

    fn authz_org_impl(&self) -> Res<&'a dyn AuthzOrgImpl> {
        let r = self
            .data_opt_impl::<Box<dyn AuthzOrgImpl>>()
            .ok_or(MyErr::OrgImplNotFound)?
            .as_ref();
        Ok(r)
    }
}

impl<'a> AuthzConfigContext<'a> for Context<'a> {
}
