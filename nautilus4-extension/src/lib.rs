#![deny(bare_trait_objects)]

extern crate gio_sys as gio_ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk4_sys as gtk_ffi;
extern crate libc;
extern crate nautilus4_extension_sys as nautilus4_ffi;

pub use crate::column_provider::{Column, ColumnProvider};
pub use crate::info_provider::{FileInfo, InfoProvider};
pub use crate::menu_provider::{Menu, MenuItem, MenuProvider};
pub use crate::nautilus_module::NautilusModule;
pub use crate::property_page_provider::{PropertyPage, PropertyPageProvider};

pub mod column_provider;
pub mod info_provider;
pub mod menu_provider;
mod nautilus_module;
pub mod property_page_provider;
mod translate;

#[macro_export]
macro_rules! nautilus_module {
    ($register_fn:ident) => {
        use std::sync::{LazyLock, Mutex as NautilusExtensionMutex};
        static MODULE_TYPE_LIST: LazyLock<NautilusExtensionMutex<Vec<GType>>> =
            LazyLock::new(|| NautilusExtensionMutex::new(Vec::new()));

        #[no_mangle]
        pub unsafe extern "C" fn nautilus_module_initialize(module: *mut GTypeModule) {
            let module_type: GType = $register_fn(module);
            MODULE_TYPE_LIST.lock().unwrap().push(module_type);
        }

        #[no_mangle]
        pub unsafe extern "C" fn nautilus_module_list_types(types: *mut *const GType, num_types: *mut c_int) {
            let list_ptr = MODULE_TYPE_LIST.lock().unwrap().as_ptr();
            let list_len = MODULE_TYPE_LIST.lock().unwrap().len() as c_int;
            unsafe {
                *types = list_ptr;
                *num_types = list_len;
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn nautilus_module_shutdown() {
        }
    }
}

#[macro_export]
macro_rules! nautilus_menu_item_activate_cb {
    ($extern_fn:ident, $safe_fn:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $extern_fn(_nautilusmenuitem: *mut GObject, user_data: gpointer) {
            use std::mem;
            use $crate::info_provider::FileInfo;

            let files: Box<Vec<FileInfo>> = Box::from_raw(mem::transmute(user_data));
            $safe_fn(*files);
        }
    };
}

#[macro_export]
macro_rules! nautilus_menu_background_activate_cb {
    ($extern_fn:ident, $safe_fn:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $extern_fn(_nautilusmenuitem: *mut GObject, user_data: gpointer) {
            use std::mem;
            use $crate::info_provider::FileInfo;

            let file: Box<FileInfo> = Box::from_raw(mem::transmute(user_data));
            $safe_fn(*file);
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
