use std::collections::HashSet;

use crate::expr::{NormExpr, RawExpr};

pub(super) fn normalize_raw_eq(head: RawExpr, args: Vec<RawExpr>) -> NormExpr {
    let mut deduplicated_children = HashSet::new();
    deduplicated_children.extend(args.into_iter().map(|a| a.normalize()));

    if deduplicated_children.len() <= 1 {
        RawExpr::new_boolean(true).into_normexpr_unsafe()
    } else {
        let mut args: Vec<RawExpr> = deduplicated_children
            .into_iter()
            .map(|a| a.into_raw())
            .collect();
        args.sort();
        RawExpr::new_node(head, args).into_normexpr_unsafe()
    }
}

pub(super) fn normalize_raw_not_eq(head: RawExpr, args: Vec<RawExpr>) -> NormExpr {
    let [lhs, rhs]: [RawExpr; 2] = args.try_into().unwrap();
    let lhs = lhs.normalize();
    let rhs = rhs.normalize();

    if lhs == rhs {
        RawExpr::new_boolean(false).into_normexpr_unsafe()
    } else if let (Some(lhs), Some(rhs)) = (lhs.get_number(), rhs.get_number()) {
        RawExpr::new_boolean(lhs != rhs).into_normexpr_unsafe()
    } else {
        // Could be undecidable yet (e.g. x != y)
        RawExpr::new_binary_node(head, lhs.into_raw(), rhs.into_raw()).into_normexpr_unsafe()
    }
}
