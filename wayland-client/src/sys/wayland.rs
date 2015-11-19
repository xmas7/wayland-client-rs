#![allow(dead_code,non_camel_case_types,unused_unsafe,unused_variables)]
#![allow(non_upper_case_globals,non_snake_case)]

pub mod interfaces {
    include!(concat!(env!("OUT_DIR"), "/wayland_interfaces.rs"));
}

pub mod client {
    include!(concat!(env!("OUT_DIR"), "/wayland_client_api.rs"));
}