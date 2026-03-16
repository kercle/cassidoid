use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use crate::atom::Atom;

#[derive(Clone)]
pub struct Raw;

#[derive(Clone)]
pub struct Normalized;

#[derive(Clone, PartialEq)]
pub enum ExprKind<E> {
    Atom { entry: Atom },
    Node { head: Box<E>, args: Vec<E> },
}

#[derive(Clone)]
pub struct Expr<S> {
    pub(super) kind: ExprKind<Expr<S>>,
    fingerprint: u64,
    _state: PhantomData<S>,
}

pub type RawExpr = Expr<Raw>;
pub type NormExpr = Expr<Normalized>;

impl<S> Expr<S> {
    pub(super) fn new_unchecked(kind: ExprKind<Expr<S>>) -> Self {
        let fingerprint = kind.digest();
        Self {
            kind,
            fingerprint,
            _state: PhantomData,
        }
    }

    pub fn fingerprint(&self) -> u64 {
        self.fingerprint
    }

    pub fn kind(&self) -> &ExprKind<Self> {
        &self.kind
    }

    pub fn into_kind(self) -> ExprKind<Self> {
        self.kind
    }
}

// -------- ExprPool brainstorming -------------

type ExprId = u32;
type ArgsId = u32;

#[derive(Clone, PartialEq, Eq, Hash)]
enum ExprCell {
    Atom(Atom),
    Node { head_id: ExprId, args_id: ArgsId },
}

struct ExprPool {
    objs: Vec<ExprCell>,
    args: Vec<Vec<ExprId>>,

    obj_map: HashMap<ExprCell, ExprId>,
    args_map: HashMap<Vec<ExprId>, ArgsId>,
}

impl ExprPool {
    fn new() -> Self {
        ExprPool {
            objs: Vec::new(),
            args: Vec::new(),
            obj_map: HashMap::new(),
            args_map: HashMap::new(),
        }
    }

    fn get_obj(&self, id: ExprId) -> &ExprCell {
        &self.objs[id as usize]
    }

    fn get_args(&self, id: ArgsId) -> &[ExprId] {
        &self.args[id as usize]
    }

    fn insert_args(&mut self, args: Vec<ExprId>) -> ArgsId {
        if let Some(&id) = self.args_map.get(&args) {
            return id;
        }
        let id = self.args.len() as ArgsId;
        self.args_map.insert(args.clone(), id);
        self.args.push(args);
        id
    }

    fn insert(&mut self, obj: ExprCell) -> ExprId {
        if let Some(&id) = self.obj_map.get(&obj) {
            return id;
        }
        let id = self.objs.len() as ExprId;
        self.obj_map.insert(obj.clone(), id);
        self.objs.push(obj);
        id
    }

    fn atom(&mut self, a: Atom) -> ExprId {
        self.insert(ExprCell::Atom(a))
    }

    fn node(&mut self, head: ExprId, args: Vec<ExprId>) -> ExprId {
        let args_id = self.insert_args(args);
        self.insert(ExprCell::Node {
            head_id: head,
            args_id,
        })
    }

    pub fn insert_expr<S>(&mut self, expr: &Expr<S>) -> ExprId {
        match expr.kind() {
            ExprKind::Atom { entry } => self.atom(entry.clone()),
            ExprKind::Node { head, args } => {
                let head_id = self.insert_expr(head);
                let arg_ids: Vec<ExprId> = args.iter().map(|arg| self.insert_expr(arg)).collect();
                self.node(head_id, arg_ids)
            }
        }
    }

    pub fn insert_raw(&mut self, expr: &RawExpr) -> RawExprHandle {
        let id = self.insert_expr(expr);
        ExprHandle::new(id)
    }

    pub fn insert_norm(&mut self, expr: &NormExpr) -> NormExprHandle {
        let id = self.insert_expr(expr);
        ExprHandle::new(id)
    }
}

#[derive(Copy, Clone)]
struct ExprHandle<S> {
    id: ExprId,
    _state: PhantomData<S>,
}

pub type RawExprHandle = ExprHandle<Raw>;
pub type NormExprHandle = ExprHandle<Normalized>;

impl<S> ExprHandle<S> {
    fn new(id: ExprId) -> Self {
        ExprHandle {
            id,
            _state: PhantomData,
        }
    }

    fn id(&self) -> ExprId {
        self.id
    }

    fn materialize(&self, pool: &ExprPool) -> Expr<S> {
        match pool.get_obj(self.id()) {
            ExprCell::Atom(atom) => Expr::new_unchecked(ExprKind::Atom {
                entry: atom.clone(),
            }),
            ExprCell::Node { head_id, args_id } => Expr::new_unchecked(ExprKind::Node {
                head: Box::new(Self::new(*head_id).materialize(pool)),
                args: pool
                    .get_args(*args_id)
                    .iter()
                    .map(|a| Self::new(*a).materialize(&pool))
                    .collect(),
            }),
        }
    }
}

enum ExprView<'a, S> {
    Atom(&'a Atom),
    Node {
        head: ExprHandle<S>,
        args: &'a [ExprId],
    },
}

impl<S: Copy> ExprHandle<S> {
    fn view(self, pool: &ExprPool) -> ExprView<S> {
        match &pool.objs[self.id as usize] {
            ExprCell::Atom(a) => ExprView::Atom(a),
            ExprCell::Node {
                head_id: head,
                args_id: args,
            } => ExprView::Node {
                head: ExprHandle::new(*head),
                args: &pool.args[*args as usize],
            },
        }
    }

    fn children(self, pool: &ExprPool) -> impl Iterator<Item = ExprHandle<S>> {
        let args = match &pool.objs[self.id as usize] {
            ExprCell::Node { args_id: args, .. } => &pool.args[*args as usize],
            ExprCell::Atom(_) => &[] as &[ExprId],
        };
        args.iter().map(move |&id| ExprHandle::new(id))
    }

    fn eq(self, other: ExprHandle<S>) -> bool {
        self.id == other.id
    }
}
