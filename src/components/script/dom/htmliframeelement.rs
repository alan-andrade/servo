/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::BindingDeclarations::HTMLIFrameElementBinding;
use dom::bindings::codegen::InheritTypes::{ElementCast, HTMLIFrameElementDerived, HTMLElementCast};
use dom::bindings::error::ErrorResult;
use dom::bindings::js::{JSRef, Temporary, OptionalRootable};
use dom::document::Document;
use dom::element::{HTMLIFrameElementTypeId, Element};
use dom::element::AttributeHandlers;
use dom::eventtarget::{EventTarget, NodeTargetTypeId};
use dom::htmlelement::HTMLElement;
use dom::node::{Node, ElementNodeTypeId, window_from_node};
use dom::virtualmethods::VirtualMethods;
use dom::window::Window;
use script_task::IterablePage;
use servo_msg::constellation_msg::{PipelineId, SubpageId};
use servo_msg::constellation_msg::{IFrameSandboxed, IFrameUnsandboxed};
use servo_msg::constellation_msg::{ConstellationChan, LoadIframeUrlMsg};
use servo_util::namespace::Null;
use servo_util::str::DOMString;
use servo_util::url::try_parse_url;

use std::ascii::StrAsciiExt;
use url::Url;

enum SandboxAllowance {
    AllowNothing = 0x00,
    AllowSameOrigin = 0x01,
    AllowTopNavigation = 0x02,
    AllowForms = 0x04,
    AllowScripts = 0x08,
    AllowPointerLock = 0x10,
    AllowPopups = 0x20
}

#[deriving(Encodable)]
pub struct HTMLIFrameElement {
    pub htmlelement: HTMLElement,
    pub size: Option<IFrameSize>,
    pub sandbox: Option<u8>
}

impl HTMLIFrameElementDerived for EventTarget {
    fn is_htmliframeelement(&self) -> bool {
       self.type_id == NodeTargetTypeId(ElementNodeTypeId(HTMLIFrameElementTypeId))
    }
}

#[deriving(Encodable)]
pub struct IFrameSize {
    pub pipeline_id: PipelineId,
    pub subpage_id: SubpageId,
}

pub trait HTMLIFrameElementHelpers {
    fn is_sandboxed(&self) -> bool;
    fn get_url(&self) -> Option<Url>;
}

impl<'a> HTMLIFrameElementHelpers for JSRef<'a, HTMLIFrameElement> {
    fn is_sandboxed(&self) -> bool {
        self.sandbox.is_some()
    }

    fn get_url(&self) -> Option<Url> {
        let element: &JSRef<Element> = ElementCast::from_ref(self);
        element.get_attribute(Null, "src").root().and_then(|src| {
            let window = window_from_node(self).root();
            try_parse_url(src.deref().value_ref(),
                          Some(window.deref().page().get_url())).ok()
        })
    }
}

impl HTMLIFrameElement {
    pub fn new_inherited(localName: DOMString, document: &JSRef<Document>) -> HTMLIFrameElement {
        HTMLIFrameElement {
            htmlelement: HTMLElement::new_inherited(HTMLIFrameElementTypeId, localName, document),
            size: None,
            sandbox: None,
        }
    }

    pub fn new(localName: DOMString, document: &JSRef<Document>) -> Temporary<HTMLIFrameElement> {
        let element = HTMLIFrameElement::new_inherited(localName, document);
        Node::reflect_node(box element, document, HTMLIFrameElementBinding::Wrap)
    }
}

pub trait HTMLIFrameElementMethods {
    fn Src(&self) -> DOMString;
    fn SetSrc(&mut self, _src: DOMString) -> ErrorResult;
    fn Srcdoc(&self) -> DOMString;
    fn SetSrcdoc(&mut self, _srcdoc: DOMString) -> ErrorResult;
    fn Name(&self) -> DOMString;
    fn SetName(&mut self, _name: DOMString) -> ErrorResult;
    fn Sandbox(&self) -> DOMString;
    fn SetSandbox(&mut self, sandbox: DOMString);
    fn AllowFullscreen(&self) -> bool;
    fn SetAllowFullscreen(&mut self, _allow: bool) -> ErrorResult;
    fn Width(&self) -> DOMString;
    fn SetWidth(&mut self, _width: DOMString) -> ErrorResult;
    fn Height(&self) -> DOMString;
    fn SetHeight(&mut self, _height: DOMString) -> ErrorResult;
    fn GetContentDocument(&self) -> Option<Temporary<Document>>;
    fn GetContentWindow(&self) -> Option<Temporary<Window>>;
    fn Align(&self) -> DOMString;
    fn SetAlign(&mut self, _align: DOMString) -> ErrorResult;
    fn Scrolling(&self) -> DOMString;
    fn SetScrolling(&mut self, _scrolling: DOMString) -> ErrorResult;
    fn FrameBorder(&self) -> DOMString;
    fn SetFrameBorder(&mut self, _frameborder: DOMString) -> ErrorResult;
    fn LongDesc(&self) -> DOMString;
    fn SetLongDesc(&mut self, _longdesc: DOMString) -> ErrorResult;
    fn MarginHeight(&self) -> DOMString;
    fn SetMarginHeight(&mut self, _marginheight: DOMString) -> ErrorResult;
    fn MarginWidth(&self) -> DOMString;
    fn SetMarginWidth(&mut self, _marginwidth: DOMString) -> ErrorResult;
    fn GetSVGDocument(&self) -> Option<Temporary<Document>>;
}

impl<'a> HTMLIFrameElementMethods for JSRef<'a, HTMLIFrameElement> {
    fn Src(&self) -> DOMString {
        "".to_owned()
    }

    fn SetSrc(&mut self, _src: DOMString) -> ErrorResult {
        Ok(())
    }

    fn Srcdoc(&self) -> DOMString {
        "".to_owned()
    }

    fn SetSrcdoc(&mut self, _srcdoc: DOMString) -> ErrorResult {
        Ok(())
    }

    fn Name(&self) -> DOMString {
        "".to_owned()
    }

    fn SetName(&mut self, _name: DOMString) -> ErrorResult {
        Ok(())
    }

    fn Sandbox(&self) -> DOMString {
        let element: &JSRef<Element> = ElementCast::from_ref(self);
        element.get_string_attribute("sandbox")
    }

    fn SetSandbox(&mut self, sandbox: DOMString) {
        let element: &mut JSRef<Element> = ElementCast::from_mut_ref(self);
        element.set_string_attribute("sandbox", sandbox);
    }

    fn AllowFullscreen(&self) -> bool {
        false
    }

    fn SetAllowFullscreen(&mut self, _allow: bool) -> ErrorResult {
        Ok(())
    }

    fn Width(&self) -> DOMString {
        "".to_owned()
    }

    fn SetWidth(&mut self, _width: DOMString) -> ErrorResult {
        Ok(())
    }

    fn Height(&self) -> DOMString {
        "".to_owned()
    }

    fn SetHeight(&mut self, _height: DOMString) -> ErrorResult {
        Ok(())
    }

    fn GetContentDocument(&self) -> Option<Temporary<Document>> {
        None
    }

    fn GetContentWindow(&self) -> Option<Temporary<Window>> {
        self.size.and_then(|size| {
            let window = window_from_node(self).root();
            let children = &*window.deref().page.children.deref().borrow();
            let child = children.iter().find(|child| {
                child.subpage_id.unwrap() == size.subpage_id
            });
            child.and_then(|page| {
                page.frame.deref().borrow().as_ref().map(|frame| {
                    Temporary::new(frame.window.clone())
                })
            })
        })
    }

    fn Align(&self) -> DOMString {
        "".to_owned()
    }

    fn SetAlign(&mut self, _align: DOMString) -> ErrorResult {
        Ok(())
    }

    fn Scrolling(&self) -> DOMString {
        "".to_owned()
    }

    fn SetScrolling(&mut self, _scrolling: DOMString) -> ErrorResult {
        Ok(())
    }

    fn FrameBorder(&self) -> DOMString {
        "".to_owned()
    }

    fn SetFrameBorder(&mut self, _frameborder: DOMString) -> ErrorResult {
        Ok(())
    }

    fn LongDesc(&self) -> DOMString {
        "".to_owned()
    }

    fn SetLongDesc(&mut self, _longdesc: DOMString) -> ErrorResult {
        Ok(())
    }

    fn MarginHeight(&self) -> DOMString {
        "".to_owned()
    }

    fn SetMarginHeight(&mut self, _marginheight: DOMString) -> ErrorResult {
        Ok(())
    }

    fn MarginWidth(&self) -> DOMString {
        "".to_owned()
    }

    fn SetMarginWidth(&mut self, _marginwidth: DOMString) -> ErrorResult {
        Ok(())
    }

    fn GetSVGDocument(&self) -> Option<Temporary<Document>> {
        None
    }
}

impl<'a> VirtualMethods for JSRef<'a, HTMLIFrameElement> {
    fn super_type<'a>(&'a mut self) -> Option<&'a mut VirtualMethods:> {
        let htmlelement: &mut JSRef<HTMLElement> = HTMLElementCast::from_mut_ref(self);
        Some(htmlelement as &mut VirtualMethods:)
    }

    fn after_set_attr(&mut self, name: DOMString, value: DOMString) {
        match self.super_type() {
            Some(ref mut s) => s.after_set_attr(name.clone(), value.clone()),
            _ => (),
        }

        if "sandbox" == name {
            let mut modes = AllowNothing as u8;
            for word in value.split(' ') {
                modes |= match word.to_ascii_lower().as_slice() {
                    "allow-same-origin" => AllowSameOrigin,
                    "allow-forms" => AllowForms,
                    "allow-pointer-lock" => AllowPointerLock,
                    "allow-popups" => AllowPopups,
                    "allow-scripts" => AllowScripts,
                    "allow-top-navigation" => AllowTopNavigation,
                    _ => AllowNothing
                } as u8;
            }
            self.deref_mut().sandbox = Some(modes);
        }
    }

    fn before_remove_attr(&mut self, name: DOMString, value: DOMString) {
        match self.super_type() {
            Some(ref mut s) => s.before_remove_attr(name.clone(), value),
            _ => (),
        }

        if "sandbox" == name {
            self.deref_mut().sandbox = None;
        }
    }

    fn bind_to_tree(&mut self) {
        match self.super_type() {
            Some(ref mut s) => s.bind_to_tree(),
            _ => (),
        }

        match self.get_url() {
            Some(url) => {
                let sandboxed = if self.is_sandboxed() {
                    IFrameSandboxed
                } else {
                    IFrameUnsandboxed
                };

                // Subpage Id
                let window = window_from_node(self).root();
                let page = window.deref().page();
                let subpage_id = page.get_next_subpage_id();

                self.deref_mut().size = Some(IFrameSize {
                    pipeline_id: page.id,
                    subpage_id: subpage_id,
                });

                let ConstellationChan(ref chan) = *page.constellation_chan.deref();
                chan.send(LoadIframeUrlMsg(url, page.id, subpage_id, sandboxed));
            }
            _ => ()
        }
    }
}
