use syntax::{ConstantNodeValue, NodeType, SyntaxNode, SyntaxTree};

// pub fn propagate_constants(tree: &mut SyntaxTree) {}

pub fn fold_constants(tree: &mut SyntaxTree) {
    log::debug!("Fold constants");
    for func in tree.postorder_traverse() {
        for node in func {
            let n = &mut *node.borrow_mut();
            if n.is_binop() {
                let (l, r) = n.get_both_binary_children();
                let left = &*l.borrow();
                let right = &*r.borrow();
                let ntype = NodeType::Num;
                let rtype = left.return_type();
                assert_eq!(rtype, right.return_type());
                if let SyntaxNode::Constant { value: lval, .. } = left {
                    if let SyntaxNode::Constant { value: rval, .. } = right {
                        log::debug!("Found folding candidate: l:{} r:{}", lval, rval);
                        let folded = match n.node_type() {
                            NodeType::Add => *lval + *rval,
                            NodeType::Sub => *lval - *rval,
                            NodeType::Mul => *lval * *rval,
                            NodeType::Div => {
                                if i64::from(*rval) == 0 {
                                    continue;
                                }
                                *lval / *rval
                            }
                            NodeType::And => {
                                let l = i64::from(*lval) != 0;
                                let r = i64::from(*rval) != 0;
                                let new = l && r;
                                ConstantNodeValue::from(new as i64)
                            }
                            NodeType::Or => {
                                let l = i64::from(*lval) != 0;
                                let r = i64::from(*rval) != 0;
                                let new = l || r;
                                ConstantNodeValue::from(new as i64)
                            }
                            NodeType::Mod => {
                                let l = i64::from(*lval);
                                let r = i64::from(*rval);
                                let new = l % r;
                                ConstantNodeValue::from(new)
                            }
                            NodeType::RelGT => ConstantNodeValue::from((*lval > *rval) as i64),
                            NodeType::RelGTE => ConstantNodeValue::from((*lval >= *rval) as i64),
                            NodeType::RelLT => ConstantNodeValue::from((*lval < *rval) as i64),
                            NodeType::RelLTE => ConstantNodeValue::from((*lval <= *rval) as i64),
                            NodeType::RelEqual => ConstantNodeValue::from((*lval == *rval) as i64),
                            NodeType::RelNotEqual => {
                                ConstantNodeValue::from((*lval != *rval) as i64)
                            }

                            _ => unreachable!(),
                        };
                        log::debug!("Folded value: {}", folded);
                        let new_node = SyntaxNode::Constant {
                            node_type: ntype,
                            return_type: rtype,
                            value: folded,
                        };
                        *n = new_node;
                    }
                }
            } else if n.is_unop() {
                let parent_ret = n.return_type();
                let parent_type = n.node_type();
                let child = n.get_unary_child().unwrap();
                if let SyntaxNode::Constant {
                    value,
                    node_type: child_type,
                    ..
                } = &*child.borrow()
                {
                    let folded = match parent_type {
                        NodeType::Coercion => {
                            let val = i64::from(*value);
                            ConstantNodeValue::new_with_ret(val, parent_ret)
                        }
                        NodeType::SignPlus => *value,
                        NodeType::SignMinus => {
                            let val = i64::from(*value);
                            ConstantNodeValue::new_with_ret(-val, parent_ret)
                        }
                        _ => unreachable!(),
                    };
                    let new_node = SyntaxNode::Constant {
                        node_type: *child_type,
                        return_type: parent_ret,
                        value: folded,
                    };
                    *n = new_node;
                };
            }
        }
    }
}
