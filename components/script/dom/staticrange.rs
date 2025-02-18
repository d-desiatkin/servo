/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;

use crate::dom::abstractrange::AbstractRange;
use crate::dom::bindings::codegen::Bindings::StaticRangeBinding::{
    StaticRangeInit, StaticRangeMethods,
};
use crate::dom::bindings::codegen::Bindings::WindowBinding::WindowMethods;
use crate::dom::bindings::error::{Error, Fallible};
use crate::dom::bindings::inheritance::NodeTypeId;
use crate::dom::bindings::reflector::reflect_dom_object_with_proto;
use crate::dom::bindings::root::DomRoot;
use crate::dom::document::Document;
use crate::dom::node::Node;
use crate::dom::window::Window;
use crate::script_runtime::CanGc;

#[dom_struct]
pub(crate) struct StaticRange {
    abstract_range: AbstractRange,
}

impl StaticRange {
    fn new_inherited(
        start_container: &Node,
        start_offset: u32,
        end_container: &Node,
        end_offset: u32,
    ) -> StaticRange {
        StaticRange {
            abstract_range: AbstractRange::new_inherited(
                start_container,
                start_offset,
                end_container,
                end_offset,
            ),
        }
    }
    pub(crate) fn new_with_doc(
        document: &Document,
        proto: Option<HandleObject>,
        init: &StaticRangeInit,
        can_gc: CanGc,
    ) -> DomRoot<StaticRange> {
        StaticRange::new_with_proto(document, proto, init, can_gc)
    }

    pub(crate) fn new_with_proto(
        document: &Document,
        proto: Option<HandleObject>,
        init: &StaticRangeInit,
        can_gc: CanGc,
    ) -> DomRoot<StaticRange> {
        let staticrange = reflect_dom_object_with_proto(
            Box::new(StaticRange::new_inherited(
                &init.startContainer,
                init.startOffset,
                &init.endContainer,
                init.endOffset,
            )),
            document.window(),
            proto,
            can_gc,
        );
        staticrange
    }
}

impl StaticRangeMethods<crate::DomTypeHolder> for StaticRange {
    /// <https://dom.spec.whatwg.org/#dom-staticrange-staticrange>
    #[allow(non_snake_case)]
    fn Constructor(
        window: &Window,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        init: &StaticRangeInit,
    ) -> Fallible<DomRoot<StaticRange>> {
        match init.startContainer.type_id() {
            NodeTypeId::DocumentType | NodeTypeId::Attr => {
                return Err(Error::InvalidNodeType);
            },
            _ => (),
        }
        match init.endContainer.type_id() {
            NodeTypeId::DocumentType | NodeTypeId::Attr => {
                return Err(Error::InvalidNodeType);
            },
            _ => (),
        }
        let document = window.Document();
        Ok(StaticRange::new_with_doc(&document, proto, init, can_gc))
    }
}
