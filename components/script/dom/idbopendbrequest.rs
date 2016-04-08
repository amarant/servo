/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::IDBOpenDBRequestBinding;
use dom::bindings::codegen::Bindings::IDBOpenDBRequestBinding::IDBOpenDBRequestMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use std::rc::Rc;
use dom::bindings::codegen::Bindings::EventHandlerBinding;

#[dom_struct]
pub struct IDBOpenDBRequest {
    reflector_: Reflector,
}

impl IDBOpenDBRequest {
    pub fn new_inherited() -> IDBOpenDBRequest {
        IDBOpenDBRequest {
            reflector_: Reflector::new(),
        }
    }

    pub fn new(global: GlobalRef) -> Root<IDBOpenDBRequest> {
        reflect_dom_object(box IDBOpenDBRequest::new_inherited(),
          global, IDBOpenDBRequestBinding::Wrap)
    }
}

impl IDBOpenDBRequestMethods for IDBOpenDBRequest {
    fn GetOnblocked(&self) -> Option<Rc<EventHandlerBinding::EventHandlerNonNull>> {
        unimplemented!();
    }

    fn SetOnblocked(&self, value: Option<Rc<EventHandlerBinding::EventHandlerNonNull>>) -> () {
        unimplemented!();
    }

    fn GetOnupgradeneeded(&self) -> Option<Rc<EventHandlerBinding::EventHandlerNonNull>> {
        unimplemented!();
    }

    fn SetOnupgradeneeded(&self, value: Option<Rc<EventHandlerBinding::EventHandlerNonNull>>) -> () {
        unimplemented!();
    }
}
