use super::prelude::*;

/// Abstract extra active model methods implementation.
pub trait ActiveModelX<E>
where
    E: EntityX<A = Self>,
    Self: ActiveModelTrait<Entity = E> + ActiveModelBehavior + Default + Send + Sync,
{
    /// Set default values from macro default.
    /// Should be generated in the model macro.
    fn set_defaults(self) -> Self;

    fn get_id(&self) -> ActiveValue<String>;
    fn set_id(self, v: &str) -> Self;
    fn get_created_at(&self) -> ActiveValue<DateTimeUtc>;
    fn set_created_at(self, v: DateTimeUtc) -> Self;
    fn get_updated_at(&self) -> ActiveValue<Option<DateTimeUtc>>;
    fn set_updated_at(self, v: DateTimeUtc) -> Self;
    fn get_deleted_at(&self) -> ActiveValue<Option<DateTimeUtc>>;
    fn set_deleted_at(self, v: DateTimeUtc) -> Self;

    /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
    /// We need to have this method instead to get default values on create.
    /// This will be used together with the macro grand_line::am_create.
    fn set_defaults_on_create(mut self) -> Self {
        if !self.get_id().is_set() {
            self = self.set_id(&ulid());
        }
        if !self.get_created_at().is_set() && E::col_created_at().is_some() {
            self = self.set_created_at(now());
        }
        self = self.set_defaults();
        self
    }
    /// Shortcut for Self::default().set_defaults_on_create()
    fn defaults_on_create() -> Self {
        <Self as Default>::default().set_defaults_on_create()
    }

    /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
    /// We need to have this method instead to get default values on update.
    /// This will be used together with the macro grand_line::am_update.
    fn set_defaults_on_update(mut self) -> Self {
        if !self.get_updated_at().is_set() && E::col_updated_at().is_some() {
            self = self.set_updated_at(now());
        }
        self
    }
    /// Shortcut for Self::default().set_defaults_on_update()
    fn defaults_on_update() -> Self {
        <Self as Default>::default().set_defaults_on_update()
    }

    /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
    /// We need to have this method instead to get default values on delete.
    /// This will be used together with the macro grand_line::am_soft_delete.
    fn set_defaults_on_delete(mut self) -> Self {
        self = self.set_defaults_on_update();
        if let Set(Some(v)) = self.get_updated_at() {
            self = self.set_deleted_at(v);
        } else if E::col_updated_at().is_some() || E::col_deleted_at().is_some() {
            let now = now();
            self = self.set_updated_at(now).set_deleted_at(now);
        }
        self
    }
    /// Shortcut for Self::default().set_defaults_on_delete()
    fn defaults_on_delete() -> Self {
        <Self as Default>::default().set_defaults_on_delete()
    }
}
