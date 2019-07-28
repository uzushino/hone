use crate::expression::never_;
use crate::types::{
    ToLiteral, 
    HasValue, 
};

pub trait UnsafeSqlFunctionArgument {
    fn to_arg_list(arg: Self) -> Vec<Box<HasValue<bool, Output=bool>>>;
}

impl<A, DB: ToLiteral> UnsafeSqlFunctionArgument for Box<HasValue<A, Output=DB>> {
    fn to_arg_list(a: Box<HasValue<A, Output=DB>>) -> Vec<Box<HasValue<bool, Output=bool>>> {
        vec![never_(a)]
    }
}

impl<A> UnsafeSqlFunctionArgument for Vec<A> where A: UnsafeSqlFunctionArgument + std::fmt::Display {
    fn to_arg_list(a: Vec<A>) -> Vec<Box<HasValue<bool, Output=bool>>> {
        let mut result = vec![];

        for (_, i) in a.iter().enumerate() {
            let t: Box<HasValue<bool, Output=bool>> = never_(i);
            let mut v = UnsafeSqlFunctionArgument::to_arg_list(t);
            result.append(&mut v)
        }

        result
    }
}

impl<A, B> UnsafeSqlFunctionArgument for (A, B)
where
    A: UnsafeSqlFunctionArgument,
    B: UnsafeSqlFunctionArgument,
{
    fn to_arg_list(v: (A, B)) -> Vec<Box<HasValue<bool, Output=bool>>> {
        let mut a = UnsafeSqlFunctionArgument::to_arg_list(v.0);
        let mut b = UnsafeSqlFunctionArgument::to_arg_list(v.1);
        let mut result = vec![];

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
    fn to_arg_list(v: (A, B, C)) -> Vec<Box<HasValue<bool, Output=bool>>> {
        let mut a = UnsafeSqlFunctionArgument::to_arg_list(v.0);
        let mut b = UnsafeSqlFunctionArgument::to_arg_list(v.1);
        let mut c = UnsafeSqlFunctionArgument::to_arg_list(v.2);
        let mut result = vec![];

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
    fn to_arg_list(v: (A, B, C, D)) -> Vec<Box<HasValue<bool, Output=bool>>> {
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
