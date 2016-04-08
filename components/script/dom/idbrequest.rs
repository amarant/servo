/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::IDBRequestBinding;
use dom::bindings::codegen::Bindings::IDBRequestBinding::IDBRequestMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::error::Fallible;
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use std::rc::Rc;
use js::jsapi::JSContext;
use js::jsval::JSVal;
use dom::domexception::DOMException;
use dom::bindings::codegen::Bindings::EventHandlerBinding;

#[dom_struct]
pub struct IDBRequest {
    reflector_: Reflector,
}

impl IDBRequest {
    pub fn new_inherited() -> IDBRequest {
        IDBRequest {
            reflector_: Reflector::new(),
        }
    }

    pub fn new(global: GlobalRef) -> Root<IDBRequest> {
        reflect_dom_object(box IDBRequest::new_inherited(),
          global, IDBRequestBinding::Wrap)
    }
}

impl IDBRequestMethods for IDBRequest {
    fn GetResult(&self, cx: *mut JSContext) -> Fallible<JSVal> {
        unimplemented!();
    }

    fn GetError(&self) -> Fallible<Option<Root<DOMException>>> {
        unimplemented!();
    }

    fn GetOnsuccess(&self) -> Option<Rc<EventHandlerBinding::EventHandlerNonNull>> {
        unimplemented!();
    }

    fn SetOnsuccess(&self, value: Option<Rc<EventHandlerBinding::EventHandlerNonNull>>) -> () {
        unimplemented!();
    }

    fn GetOnerror(&self) -> Option<Rc<EventHandlerBinding::EventHandlerNonNull>> {
        unimplemented!();
    }

    fn SetOnerror(&self, value: Option<Rc<EventHandlerBinding::EventHandlerNonNull>>) -> () {
        unimplemented!();
    }
}