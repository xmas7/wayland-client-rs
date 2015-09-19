#![feature(static_recursion, result_expect)]

#[macro_use]
extern crate dlib;

#[cfg(feature = "dlopen")]
#[macro_use]
extern crate lazy_static;

extern crate libc;

mod abi;
mod sys;

pub mod wayland;

use abi::client::wl_proxy;
use abi::common::wl_interface;

pub trait Proxy {
    fn ptr(&self) -> *mut wl_proxy;
    fn interface() -> *mut wl_interface;
    fn id(&self) -> ProxyId;
    unsafe fn from_ptr(ptr: *mut wl_proxy) -> Self;
}

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub struct ProxyId { id: usize }

fn wrap_proxy(ptr: *mut wl_proxy) -> ProxyId {
    ProxyId { id: ptr as usize}
}

#[derive(Debug)]
pub enum Event {
    Wayland(wayland::WaylandProtocolEvent)
}

fn dispatch_event(evt: Event) {
    println!("event: {:?}", evt);
}