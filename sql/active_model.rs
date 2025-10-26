use crate::prelude::*;

/// Abstract extra active model methods implementation.
pub trait ActiveModelX<E>
where
    E: EntityX<A = Self>,
    Self: ActiveModelTrait<Entity = E> + ActiveModelBehavior + Default + Send + Sync + Sized,
{
    /// To clarify model name in case of error in the macro am_value.
    fn _model_name(&self) -> &'static str {
        E::_model_name()
    }

    /// Set default values from macro default.
    fn _set_default_values(self) -> Self;

    fn _get_id(&self) -> ActiveValue<String>;
    fn _set_id(self, v: &str) -> Self;
    fn _get_created_at(&self) -> ActiveValue<DateTimeUtc>;
    fn _set_created_at(self, v: DateTimeUtc) -> Self;
    fn _get_updated_at(&self) -> ActiveValue<Option<DateTimeUtc>>;
    fn _set_updated_at(self, v: DateTimeUtc) -> Self;
    fn _get_deleted_at(&self) -> ActiveValue<Option<DateTimeUtc>>;
    fn _set_deleted_at(self, v: DateTimeUtc) -> Self;

    /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
    /// We need to have this method instead to get default values on create.
    /// This will be used together with the macro grand_line::am_create.
    fn _create(mut self) -> Self {
        if !self._get_id().is_set() {
            self = self._set_id(&ulid());
        }
        if !self._get_created_at().is_set() {
            self = self._set_created_at(now());
        }
        self = self._set_default_values();
        self
    }

    /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
    /// We need to have this method instead to get default values on update.
    /// This will be used together with the macro grand_line::am_update.
    fn _update(mut self) -> Self {
        if !self._get_updated_at().is_set() {
            self = self._set_updated_at(now());
        }
        self
    }

    /// sea_orm ActiveModel hooks will not be called with Entity:: or bulk methods.
    /// We need to have this method instead to get default values on delete.
    /// This will be used together with the macro grand_line::am_soft_delete.
    fn _delete(mut self) -> Self {
        self = self._update();
        if let Set(Some(v)) = self._get_updated_at() {
            self = self._set_deleted_at(v);
        } else {
            let now = now();
            self = self._set_updated_at(now)._set_deleted_at(now);
        }
        self
    }
}
