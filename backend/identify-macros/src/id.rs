/// Macro for generating stable UUID identifiers from a set of inputs.
///
/// This macro generates an ID model that is used to generate a UUID V5 from the contained fields.
///
/// It uses the [model generation macros](crate::gen_model) internally for generating the ID model and the
/// new entity helper, so possible field attributes and configurations are the same as for that macro.
///
/// # Examples
///
/// ```
/// # use uuid::Uuid;
/// # use identify_macros::gen_id;
/// const UUID_NAMESPACE: Uuid = Uuid::from_bytes(*b"doc-example-uuid");
///
/// gen_id! {
///     UUID_NAMESPACE,
///     /// A stable and deterministic ID that uniquely identifies an entity within the system.
///    #[derive(Debug, Clone)]
///     pub struct ModelId {
///         /// Email.
///         email: String,
///         /// Username.
///         username: String,
///     }
///
///     #[derive(Debug, Clone)]
///     pub struct ModelIdAttrs;
/// }
/// ```
///
/// The above expands to the following code:
///
/// ```
/// # const UUID_NAMESPACE: uuid::Uuid = uuid::Uuid::from_bytes(*b"doc-example-uuid");
/// /// A stable and deterministic ID that uniquely identifies an entity within the system.
/// #[derive(Debug, Clone)]
/// pub struct ModelId {
///     /// Email.
///     email: String,
///     /// Username.
///     username: String,
/// }
///
/// impl ModelId {
///     /// Email.
///     pub fn email(&self) -> &String {
///         &self.email
///     }
///
///     /// Username.
///     pub fn username(&self) -> &String {
///         &self.username
///     }
/// }
///
/// #[derive(Debug, Clone)]
/// pub struct ModelIdAttrs {
///     /// Email.
///     pub email: String,
///     /// Username.
///     pub username: String,
/// }
///
/// impl ModelId {
///     /// Generates a UUID V5 from the fields this ID model has.
///     pub fn to_uuid(&self) -> ::uuid::Uuid {
///         let mut name = Vec::new();
///
///         name.extend_from_slice("ModelId".as_bytes());
///         name.extend_from_slice(b" ID");
///         name.extend_from_slice(self.email.as_bytes());
///         name.extend_from_slice(self.username.as_bytes());
///
///         ::uuid::Uuid::new_v5(&UUID_NAMESPACE, &name)
///     }
/// }
///
/// impl From<&ModelId> for ::uuid::Uuid {
///     fn from(value: &ModelId) -> Self {
///         value.to_uuid()
///     }
/// }
/// ```
///
/// # Usage
///
/// ## Using custom byte slice conversion function
///
/// By default, the macro tries to call the `as_bytes` method on every field to gets their byte slice representation.
///
/// You can override this if needed by using the following syntax:
///
/// ```
/// # use identify_macros::gen_id;
/// # const UUID_NAMESPACE: uuid::Uuid = uuid::Uuid::from_bytes(*b"doc-example-uuid");
/// // An explicit version of what is done by default.
/// fn custom_to_bytes(value: &str) -> &[u8] {
///     value.as_ref()
/// }
///
/// gen_id! {
///     UUID_NAMESPACE,
///     pub struct ModelId {
///         value: String [custom_to_bytes],
///     }
/// }
/// ```
///
/// The UUID generation function will then look like this:
///
/// ```
/// # const UUID_NAMESPACE: uuid::Uuid = uuid::Uuid::from_bytes(*b"doc-example-uuid");
/// # fn custom_to_bytes(value: &str) -> &[u8] {
/// #    value.as_ref()
/// # }
/// # pub struct ModelId {
/// #     value: String,
/// # }
/// # impl ModelId {
///  pub fn to_uuid(&self) -> ::uuid::Uuid {
///     let mut name = Vec::new();
///
///     name.extend_from_slice("ModelId".as_bytes());
///     name.extend_from_slice(b" ID");
///     name.extend_from_slice(custom_to_bytes(&self.value));
///
///     ::uuid::Uuid::new_v5(&UUID_NAMESPACE, &name)
///  }
/// #    }
/// ```
///
/// # Notes
///
/// The generated UUIDs **depend on the order of fields in the ID model**. Rearranging the fields will
/// result in different UUIDs being generated.
#[macro_export]
macro_rules! gen_id {
    ($($input:tt)*) => {
        $crate::gen_id_helper!($($input)*);
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! gen_id_helper {
    (
        // UUID namespace used for the UUID V5 generation.
        $uuid_namespace:ident,

        // ID model.
        $(#[$id_attrs:meta])*
        $id_vis:vis struct $name:ident {
            $(
                $(#[$($f_attrs:tt)*])*
                $f_vis:vis $f_name:ident: $f_type:ty $([$($conv_fn:tt)*])?,
            )+
        }

        // Optional helper definitions as expected by the `gen_model` macro.
        $($helpers:tt)*
    ) => {
        // Generate the ID model using the model generation macro.
        $crate::gen_model! {
            $(#[$id_attrs])*
            $id_vis struct $name {
               $(
                    $(#[$($f_attrs)*])*
                    $f_vis $f_name: $f_type,
                )+
            }

            // Optional helpers.
            $($helpers)*
        }

        // Implement the UUID generation method.
        impl $name {
            /// Generates a UUID V5 from the fields this ID model has.
            pub fn to_uuid(&self) -> ::uuid::Uuid {
                let mut name = Vec::new();

                name.extend_from_slice(::core::stringify!($name).as_bytes());
                name.extend_from_slice(b" ID");
                // Use all fields this ID model has.
                $(
                    // Account for fields that use custom functions to get bytes representation.
                    name.extend_from_slice(gen_id_helper!(@bytes self, $f_name $(, $($conv_fn)*)?));
                )+

                ::uuid::Uuid::new_v5(&$uuid_namespace, &name)
            }
        }

        impl From<&$name> for ::uuid::Uuid {
            fn from(value: &$name) -> Self {
                value.to_uuid()
            }
        }
    };

    // A standard field.
    (@bytes $self:ident, $field:ident) => {
        $self.$field.as_bytes()
    };

    // Field with a custom byte slice conversion function.
    (@bytes $self:ident, $field:ident, $($conv_fn:tt)*) => {
        $($conv_fn)*(&$self.$field)
    };
}
