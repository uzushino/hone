use std::rc::Rc;

use crate::query::UnsafeSqlFunctionArgument;
use crate::types::{HasValue, NeedParens, Raw};

impl<A, DB> UnsafeSqlFunctionArgument for Rc<HasValue<A, DB>> {
    fn to_arg_list(a: Rc<HasValue<A, DB>>) -> Vec<Rc<HasValue<(), bool>>> {
        let a = Rc::new(Raw(NeedParens::Never, a.to_string()));
        vec![a]
    }
}

impl<A> UnsafeSqlFunctionArgument for Vec<A>
where
    A: UnsafeSqlFunctionArgument + Clone,
{
    fn to_arg_list(a: Vec<A>) -> Vec<Rc<HasValue<(), bool>>> {
        let mut result = vec![];

        for (_, i) in a.iter().enumerate() {
            let v = UnsafeSqlFunctionArgument::to_arg_list((*i).clone());
            result.append(&mut v.to_vec())
        }

        result
    }
}

impl<A, B> UnsafeSqlFunctionArgument for (A, B)
where
    A: UnsafeSqlFunctionArgument,
    B: UnsafeSqlFunctionArgument,
{
    fn to_arg_list(v: (A, B)) -> Vec<Rc<HasValue<(), bool>>> {
        let mut result = vec![];

        let mut a = UnsafeSqlFunctionArgument::to_arg_list(v.0);
        let mut b = UnsafeSqlFunctionArgument::to_arg_list(v.1);

        result.append(&mut a);
        result.append(&mut b);

        result
    }
}

impl<A, B, C> UnsafeSqlFunctionArgument for (A, B, C)
where
    A: UnsafeSqlFunctionArgument,
    B: UnsafeSqlFunctionArgument,
    C: UnsafeSqlFunctionArgument,
{
    fn to_arg_list(v: (A, B, C)) -> Vec<Rc<HasValue<(), bool>>> {
        let mut result = vec![];

        let mut a = UnsafeSqlFunctionArgument::to_arg_list(v.0);
        let mut b = UnsafeSqlFunctionArgument::to_arg_list(v.1);
        let mut c = UnsafeSqlFunctionArgument::to_arg_list(v.2);

        result.append(&mut a);
        result.append(&mut b);
        result.append(&mut c);

        result
    }
}

impl<A, B, C, D> UnsafeSqlFunctionArgument for (A, B, C, D)
where
    A: UnsafeSqlFunctionArgument,
    B: UnsafeSqlFunctionArgument,
    C: UnsafeSqlFunctionArgument,
    D: UnsafeSqlFunctionArgument,
{
    fn to_arg_list(v: (A, B, C, D)) -> Vec<Rc<HasValue<(), bool>>> {
        let mut result = vec![];

        let mut a = UnsafeSqlFunctionArgument::to_arg_list(v.0);
        let mut b = UnsafeSqlFunctionArgument::to_arg_list(v.1);
        let mut c = UnsafeSqlFunctionArgument::to_arg_list(v.2);
        let mut d = UnsafeSqlFunctionArgument::to_arg_list(v.3);

        result.append(&mut a);
        result.append(&mut b);
        result.append(&mut c);
        result.append(&mut d);

        result
    }
}
