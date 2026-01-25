/// Macro for generating domain models.
///
/// This macro generates a domain model and two optional helpers:
///
/// - One for creating a new instance of the entity (the "new entity" helper).
/// - One for hydrating an existing entity from a set of attributes (the "hydration" helper).
///
/// # Examples
///
/// ```
/// # use identify_macros::gen_model;
/// gen_model!  {
///     // This is the model itself.
///     pub struct Model {
///         /// A numeric ID with a custom type for new models.
///         #[new(type(u32))]
///         id: u64,
///         /// A string field with a custom getter.
///         #[get(as_ref(&str))]
///         #[new(skip)]
///         first_name: String,
///         #[new(skip)]
///         #[hydrate(skip)]
///         last_name: String,
///     }
///
///     // This is the new entity helper with additional field.
///     pub struct NewModelAttrs {
///         /// We want to parse first name and last name from a concatenated string.
///         full_name: String,
///     }
///
///     // This is the hydration helper with additional field.
///     pub struct ModelAttrs {
///         /// Concatenated [Model::first_name] and [Model::last_name].
///         full_name: String,
///     }
/// }
/// ```
///
/// The above expands to the following code:
///
/// ```
/// pub struct Model {
///     /// A numeric ID with a custom type for new models.
///     id: u64,
///     first_name: String,
///     last_name: String,
/// }
///
/// impl Model {
///     /// A numeric ID with a custom type for new models.
///     pub fn id(&self) -> &u64 {
///         &self.id
///     }
///
///     /// A string field with a custom getter.
///     pub fn first_name(&self) -> &str {
///         self.first_name.as_ref()
///     }
///
///     pub fn last_name(&self) -> &String {
///         &self.last_name
///     }
/// }
///
/// pub struct NewModelAttrs {
///     /// We want to parse first name and last name from a concatenated string.
///     pub full_name: String,
///     /// A numeric ID with a custom type for new models.
///     pub id: u32,
/// }
///
/// pub struct ModelAttrs {
///     /// Concatenated [Model::first_name] and [Model::last_name].
///     pub full_name: String,
///     /// A numeric ID with a custom type for new models.
///     pub id: u64,
///     /// A string field with a custom getter.
///     pub first_name: String,
/// }
/// ```
///
/// # Usage
///
/// ## Getter options
///
/// You can annotate fields on your model with `#[get(...)]` to change the generated getters.
///
/// Supported options:
///
/// - `#[get(skip)]` - skips the field.
/// - `#[get(into(<type>))]` - calls `Into::into()` on the field to cast it to the specified type.
/// - `#[get(ref_into(<type>))]` - calls `Into::into()` on a reference to the field to cast it to the specified type.
/// - `#[get(as_ref(<type>))]` - calls `AsRef::as_ref()` on the field to borrow the specified type from it.
/// - `#[get(copy)]` - returns a copy of the field.
///
///  ⚠️ All provided options are **mutually-exclusive**.
///
/// ## Generating helpers
///
/// You can have at most two additional helper structs for every model you generate:
///
/// - One for creating a new instance of the entity.
/// - And the second one for hydrating an existing entity from a set of attributes.
///
/// ## New entity helper options
///
/// You can annotate fields on your model with `#[new(...)]` to change the generated helper.
///
/// Supported options:
///
/// - `#[new(skip)]` - skips the field.
/// - `#[new(type(<type>))]` - uses a different type for this field in the helper struct.
///
/// ⚠️ All provided options are **mutually-exclusive**.
///
/// ## Hydration helper options
///
/// You can annotate fields on your model with `#[new(...)]` to change the generated helper.
///
/// Supported options:
///
/// - `#[hydrate(skip)]` - skips the field.
/// - `#[hydrate(type(<type>))]` - uses a different type for this field in the helper struct.
///
/// ⚠️ All provided options are **mutually-exclusive**.
///
/// ## Using custom attributes
///
/// This macro supports forwarding any custom attributes using a special attribute `#[fw(...)]`.
///
/// The wrapping is needed to simplify parsing of attributes provided by this macro.
///
/// Example:
///
/// ```
/// # use identify_macros::gen_model;
/// gen_model! {
///     pub struct Model {
///         #[fw(allow(dead_code))]
///         field: String,
///     }
/// }
/// ```
#[macro_export]
macro_rules! gen_model {
    ($($input:tt)*) => {
        $crate::gen_model_helper!($($input)*);
    }
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! gen_model_helper {
    // Main entrypoint.
    (
        $(#[$model_attrs:meta])*
        $model_vis:vis struct $model_name:ident {
            $(
                // Doc comments.
                $(#[doc = $($f_doc:tt)*])*
                $(#[doc($($f_doc2:tt)*)])*

                // Additional options for the getter that will be generated for this field.
                $(#[get(
                    $(skip$(($get_skip_marker:tt))?)?
                    $(into($into_type:ty))?
                    $(ref_into($ref_into_type:ty))?
                    $(as_ref($as_ref_type:ty))?
                    $(copy$(($get_copy_marker:tt))?)?
                )])?

                // Additional options for the new entity creation helper struct field generated from this field.
                $(#[new(
                    $(skip$(($new_skip_marker:tt))?)?
                    $(type($new_type:ty))?
                )])?

                // Additional options for the hydration helper struct field generated from this field.
                $(#[hydrate(
                    $(skip$(($hydrate_skip_marker:tt))?)?
                    $(type($hydrate_type:ty))?
                )])?

                // A forwarding wrapper for any additional attributes the field needs.
                $(#[fw($($f_forwarded_attr:tt)*)])*
                $f_vis:vis $f_name:ident: $f_type:ty,
            )+
        }

        // You can have at most two additional helper structs for every model you generate:
        //  - One for creating a new instance of the entity.
        //  - And the second one for hydrating an existing entity from a set of attributes.
        $(
            $(#[$helper_attrs:meta])*
            $helper_vis:vis struct $helper_name:ident$(;)?
            $(
                {
                    $(
                        $(#[$helper_f_attrs:meta])*
                        $helper_f_name:ident: $helper_f_type:ty,
                    )*
                }
            )?
        )*
    ) => {
        // Create the model itself.
        $(#[$model_attrs])*
        $model_vis struct $model_name {
            // Use all attrs except the ones meant for this macro.
            $(
                $(#[doc = $($f_doc)*])*
                $(#[doc($($f_doc2)*)])*
                $(#[$($f_forwarded_attr)*])*
                $f_vis $f_name: $f_type,
            )+
        }

        // Generate getters.
        gen_model_helper!(
            @gen-getters
            $model_vis,
            $model_name,
            $(
                $(#[doc = $($f_doc)*])*
                $(#[doc($($f_doc2)*)])*
                $(#[get(
                    $(skip$(($get_skip_marker))?)?
                    $(into($into_type))?
                    $(ref_into($ref_into_type))?
                    $(as_ref($as_ref_type))?
                    $(copy$(($get_copy_marker))?)?
                )])?
                $f_name: $f_type,
            )*
        );

        // Generate helpers (if any).
        gen_model_helper!(
            @gen-helpers
            $(
                $(#[$helper_attrs])*
                $helper_vis struct $helper_name
                $(
                    // Additional fields required by the corresponding helper struct
                    {
                        $(
                            $(#[$helper_f_attrs])*
                            $helper_f_name: $helper_f_type,
                        )*
                    }
                )?
            )*
            $(
                $(#[doc = $($f_doc)*])*
                $(#[doc($($f_doc2)*)])*
                $(#[new($(skip$(($new_skip_marker))?)? $(type($new_type))?)])?
                $(#[hydrate($(skip$(($hydrate_skip_marker))?)? $(type($hydrate_type))?)])?
                $f_name: $f_type,
            )*
        );
    };

    // Generate both helper structs.
    (
        @gen-helpers

        // New entity helper.
        $(#[$new_h_attrs:meta])*
        $new_h_vis:vis struct $new_h_name:ident
        $(
            {
                $(
                    $(#[$new_h_f_attrs:meta])*
                    $new_h_f_name:ident: $new_h_f_type:ty,
                )*
            }
        )?

        // Hydration helper.
        $(#[$hydrate_h_attrs:meta])*
        $hydrate_h_vis:vis struct $hydrate_h_name:ident
        $(
            {
                $(
                    $(#[$hydrate_h_f_attrs:meta])*
                    $hydrate_h_f_name:ident: $hydrate_h_f_type:ty,
                )*
            }
        )?

        // Model fields.
        $(
            $(#[doc = $($f_doc:tt)*])*
            $(#[doc($($f_doc2:tt)*)])*
            $(#[new($(skip$(($new_skip_marker:ident))?)? $(type($new_type:ty))?)])?
            $(#[hydrate($(skip$(($hydrate_skip_marker:ident))?)? $(type($hydrate_type:ty))?)])?
            $f_vis:vis $f_name:ident: $f_type:ty,
        )*
    ) => {
        // Generate new entity helper.
        gen_model_helper!(
            @gen-new-helper
            $(#[$new_h_attrs])*
            $new_h_vis struct $new_h_name
            [
                $(
                    $(
                        $(#[$new_h_f_attrs])*
                        pub $new_h_f_name: $new_h_f_type,
                    )*
                )?
            ]
            $(
                $(#[doc = $($f_doc)*])*
                $(#[doc($($f_doc2)*)])*
                $(#[new($(skip$(($new_skip_marker))?)? $(type($new_type))?)])?
                $f_name: $f_type,
            )*
        );

        // Generate hydration helper.
        gen_model_helper!(
            @gen-hydrate-helper
            $(#[$hydrate_h_attrs])*
            $hydrate_h_vis struct $hydrate_h_name
            [
                $(
                    $(
                        $(#[$hydrate_h_f_attrs])*
                        pub $hydrate_h_f_name: $hydrate_h_f_type,
                    )*
                )?
            ]
            $(
                $(#[doc = $($f_doc)*])*
                $(#[doc($($f_doc2)*)])*
                $(#[hydrate($(skip$(($hydrate_skip_marker))?)? $(type($hydrate_type))?)])?
                $f_name: $f_type,
            )*
        );
    };

    // Generate only the new entity helper struct.
    (
        @gen-helpers

        // New entity helper.
        $(#[$new_h_attrs:meta])*
        $new_h_vis:vis struct $new_h_name:ident
        $(
            {
                $(
                    $(#[$new_h_f_attrs:meta])*
                    $new_h_f_name:ident: $new_h_f_type:ty,
                )*
            }
        )?

        // Model fields.
        $(
            $(#[doc = $($f_doc:tt)*])*
            $(#[doc($($f_doc2:tt)*)])*
            $(#[new($(skip$(($new_skip_marker:ident))?)? $(type($new_type:ty))?)])?
            $(#[hydrate$($_:tt)*])?
            $f_vis:vis $f_name:ident: $f_type:ty,
        )*
    ) => {
        gen_model_helper!(
            @gen-new-helper
            $(#[$new_h_attrs])*
            $new_h_vis struct $new_h_name
            [
                $(
                    $(
                        $(#[$new_h_f_attrs])*
                        pub $new_h_f_name: $new_h_f_type,
                    )*
                )?
            ]
            $(
                $(#[doc = $($f_doc)*])*
                $(#[doc($($f_doc2)*)])*
                $(#[new($(skip$(($new_skip_marker))?)? $(type($new_type))?)])?
                $f_name: $f_type,
            )*
        );
    };

    // Fallback case when no helpers are required.
    (@gen-helpers $($_:tt)*) => {};

    // Generate an ordinary field for the new entity helper struct.
    (
        @gen-new-helper
        $(#[$attr:meta])*
        $vis:vis struct $name:ident
        [$($processed:tt)*]

        $(#[doc = $($f_doc:tt)*])*
        $(#[doc($($f_doc2:tt)*)])*
        $f_name:ident: $f_type:ty,

        $($rest:tt)*
    ) => {
        gen_model_helper!(
            @gen-new-helper
            $(#[$attr])*
            $vis struct $name
            [
                $($processed)*

                $(#[doc = $($f_doc)*])*
                $(#[doc($($f_doc2)*)])*
                pub $f_name: $f_type,
            ]
            $($rest)*
        );
    };

    // Skip a field for the new entity helper struct.
    (
        @gen-new-helper
        $(#[$attr:meta])*
        $vis:vis struct $name:ident
        [$($processed:tt)*]

        $(#[doc = $($f_doc:tt)*])*
        $(#[doc($($f_doc2:tt)*)])*
        #[new(skip$([$_:tt])?)]
        $f_name:ident: $f_type:ty,

        $($rest:tt)*
    ) => {
        gen_model_helper!(
            @gen-new-helper
            $(#[$attr])*
            $vis struct $name
            [$($processed)*]
            $($rest)*
        );
    };

    // Use a custom type for a field in the new entity helper struct.
    (
        @gen-new-helper
        $(#[$attr:meta])*
        $vis:vis struct $name:ident
        [$($processed:tt)*]

        $(#[doc = $($f_doc:tt)*])*
        $(#[doc($($f_doc2:tt)*)])*
        #[new(type($type:ty))]
        $f_name:ident: $f_type:ty,

        $($rest:tt)*
    ) => {
        gen_model_helper!(
            @gen-new-helper
            $(#[$attr])*
            $vis struct $name
            [
                $($processed)*

                $(#[doc = $($f_doc)*])*
                $(#[doc($($f_doc2)*)])*
                pub $f_name: $type,
            ]
            $($rest)*
        );
    };

    // Generate the new entity helper struct from all the processed fields.
    (
        @gen-new-helper
        $(#[$attr:meta])*
        $vis:vis struct $name:ident
        [$($processed:tt)*]
    ) => {
        $(#[$attr])*
        $vis struct $name {
            $($processed)*
        }
    };

    // Generate an ordinary field for the hydration helper struct.
    (
        @gen-hydrate-helper
        $(#[$attr:meta])*
        $vis:vis struct $name:ident
        [$($processed:tt)*]

        $(#[doc = $($f_doc:tt)*])*
        $(#[doc($($f_doc2:tt)*)])*
        $f_name:ident: $f_type:ty,

        $($rest:tt)*
    ) => {
        gen_model_helper!(
            @gen-hydrate-helper
            $(#[$attr])*
            $vis struct $name
            [
                $($processed)*

                $(#[doc = $($f_doc)*])*
                $(#[doc($($f_doc2)*)])*
                pub $f_name: $f_type,
            ]
            $($rest)*
        );
    };

    // Skip a field for the hydration helper struct.
    (
        @gen-hydrate-helper
        $(#[$attr:meta])*
        $vis:vis struct $name:ident
        [$($processed:tt)*]

        $(#[doc = $($f_doc:tt)*])*
        $(#[doc($($f_doc2:tt)*)])*
        #[hydrate(skip$([$_:tt])?)]
        $f_name:ident: $f_type:ty,

        $($rest:tt)*
    ) => {
        gen_model_helper!(
            @gen-hydrate-helper
            $(#[$attr])*
            $vis struct $name
            [$($processed)*]
            $($rest)*
        );
    };

    // Use a custom type for a field in the hydration helper struct.
    (
        @gen-hydrate-helper
        $(#[$attr:meta])*
        $vis:vis struct $name:ident
        [$($processed:tt)*]

        $(#[doc = $($f_doc:tt)*])*
        $(#[doc($($f_doc2:tt)*)])*
        #[hydrate(type($type:ty))]
        $f_name:ident: $f_type:ty,

        $($rest:tt)*
    ) => {
        gen_model_helper!(
            @gen-hydrate-helper
            $(#[$attr])*
            $vis struct $name
            [
                $($processed)*

                $(#[doc = $($f_doc)*])*
                $(#[doc($($f_doc2)*)])*
                pub $f_name: $type,
            ]
            $($rest)*
        );
    };

    // Generate the hydration helper struct from all the processed fields.
    (
        @gen-hydrate-helper
        $(#[$attr:meta])*
        $vis:vis struct $name:ident
        [$($processed:tt)*]
    ) => {
        $(#[$attr])*
        $vis struct $name {
            $($processed)*
        }
    };

    // Entrypoint for generating the getters.
    (
        @gen-getters
        $vis:vis,
        $name:ident,
        $($fields:tt)*
    ) => {
        impl $name {
            gen_model_helper!(@gen-getter $vis, $($fields)*);
        }
    };

    // Generate an ordinary getter.
    (
        @gen-getter
        $vis:vis,
        $(#[doc = $($f_doc:tt)*])*
        $(#[doc($($f_doc2:tt)*)])*
        $f_name:ident: $f_type:ty,
        $($rest:tt)*
    ) => {
        $(#[doc = $($f_doc)*])*
        $(#[doc($($f_doc2)*)])*
        $vis fn $f_name(&self) -> &$f_type {
            &self.$f_name
        }

        gen_model_helper!(@gen-getter $vis, $($rest)*);
    };

    // Skip a field when generating getters.
    (
        @gen-getter
        $vis:vis,
        $(#[doc = $($f_doc:tt)*])*
        $(#[doc($($f_doc2:tt)*)])*
        #[get(skip)]
        $f_name:ident: $f_type:ty,
        $($rest:tt)*
    ) => {
        gen_model_helper!(@gen-getter $vis, $($rest)*);
    };

    // Call `Into::into()` on the field in the getter.
    (
        @gen-getter
        $vis:vis,
        $(#[doc = $($f_doc:tt)*])*
        $(#[doc($($f_doc2:tt)*)])*
        #[get(into($into_ty:ty))]
        $f_name:ident: $f_type:ty,
        $($rest:tt)*
    ) => {
        $(#[doc = $($f_doc)*])*
        $(#[doc($($f_doc2)*)])*
        $vis fn $f_name(&self) -> $into_ty {
            self.$f_name.into()
        }

        gen_model_helper!(@gen-getter $vis, $($rest)*);
    };

    // Call `Into::into()` on a reference to a field in the getter.
    (
        @gen-getter
        $vis:vis,
        $(#[doc = $($f_doc:tt)*])*
        $(#[doc($($f_doc2:tt)*)])*
        #[get(ref_into($into_ty:ty))]
        $f_name:ident: $f_type:ty,
        $($rest:tt)*
    ) => {
        $(#[doc = $($f_doc)*])*
        $(#[doc($($f_doc2)*)])*
        $vis fn $f_name(&self) -> $into_ty {
            (&self.$f_name).into()
        }

        gen_model_helper!(@gen-getter $vis, $($rest)*);
    };

    // Call `AsRef::as_ref()` on a field in the getter.
    (
        @gen-getter
        $vis:vis,
        $(#[doc = $($f_doc:tt)*])*
        $(#[doc($($f_doc2:tt)*)])*
        #[get(copy)]
        $f_name:ident: $f_type:ty,
        $($rest:tt)*
    ) => {
        $(#[doc = $($f_doc)*])*
        $(#[doc($($f_doc2)*)])*
        $vis fn $f_name(&self) -> $f_type {
            self.$f_name
        }

        gen_model_helper!(@gen-getter $vis, $($rest)*);
    };

    // Return a copy of the field in the getter.
    (
        @gen-getter
        $vis:vis,
        $(#[doc = $($f_doc:tt)*])*
        $(#[doc($($f_doc2:tt)*)])*
        #[get(as_ref($as_ref_ty:ty))]
        $f_name:ident: $f_type:ty,
        $($rest:tt)*
    ) => {
        $(#[doc = $($f_doc)*])*
        $(#[doc($($f_doc2)*)])*
        $vis fn $f_name(&self) -> $as_ref_ty {
            self.$f_name.as_ref()
        }

        gen_model_helper!(@gen-getter $vis, $($rest)*);
    };


    // Fallback case when all getters have been generated already.
    (@gen-getter $vis:vis$(,)?) => {};
}
