/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use rustc::lint::{Context, LintPass, LintArray};
use rustc::middle::def_id::DefId;
use rustc_front::hir;
use rustc_front::hir::Public;
use syntax::ast;
use syntax::attr::AttrMetaMethods;

declare_lint!(PRIVATIZE, Deny,
              "Allows to enforce private fields for struct definitions");

/// Lint for keeping DOM fields private
///
/// This lint (disable with `-A privatize`/`#[allow(privatize)]`) ensures all types marked with `#[privatize]`
/// have no public fields
pub struct PrivatizePass;

impl LintPass for PrivatizePass {
    fn get_lints(&self) -> LintArray {
        lint_array!(PRIVATIZE)
    }

    fn check_struct_def(&mut self,
                        cx: &Context,
                        def: &hir::StructDef,
                        _i: ast::Ident,
                        _gen: &hir::Generics,
                        id: ast::NodeId) {
        if cx.tcx.has_attr(DefId::local(id), "privatize") {
            for field in &def.fields {
                match field.node {
                    hir::StructField_ { kind: hir::NamedField(ident, visibility), .. } if visibility == Public => {
                        cx.span_lint(PRIVATIZE, field.span,
                                     &format!("Field {} is public where only private fields are allowed",
                                              ident.name));
                    }
                    _ => {}
                }
            }
        }
    }
}
