use crate::value::{Value, is_integer, BuiltinFn};
use crate::eval_env::EvalEnv;
use std::process;
use std::panic;
pub fn apply(params: Vec<Value>, env: &EvalEnv) -> Value {
    if params.len() < 2{
        panic!("SyntaxError: Missing argument.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument");
    }
    else {
        match params[0].clone() {
            Value::ProcedureValue(f) => {
                // let args: Vec<Value> = params[1..].iter().cloned().map(|value| env.eval(value)).collect();
                let args: Vec<Value> = params[1].to_vector();
                return f(args, env);
            },
            Value::LambdaValue(params_in_lambda, body, env) => {
                let env_derived = env.derive(*params_in_lambda, params[1].to_vector());
                let mut result: Value = Value::NilValue;
                for bodyv in *body {
                    result = env_derived.eval(bodyv);
                }
                return result;
            },
            _ => panic!("Cannot evaluate the expression as a procedure."),
        }
    }
}
pub fn print(params: Vec<Value>, _env: &EvalEnv) -> Value {
    params.iter().for_each(|param| println!("{}", param.to_string()));
    Value::NilValue  
}
pub fn display(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <display>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <display>.");
    }
    else {
        match params[0].clone() {
            Value::StringValue(s) => {
                print!("{}", s);
            },
            v => {
                print!("{}", v.to_string());
            }
        }
        Value::NilValue
    }
}
pub fn displayln(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <display>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <display>.");
    }
    else {
        match params[0].clone() {
            Value::StringValue(s) => {
                println!("{}", s);
            },
            v => {
                println!("{}", v.to_string());
            }
        }
        Value::NilValue
    }
}
pub fn error(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <display>.");
    }
    else if params.len() == 1 {
        panic!("{}", params[0].to_string());
    }
    else {
        panic!("Error thrown.");
    }
}
pub fn eval(params: Vec<Value>, env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <eval>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <eval>.");
    }
    else {
        env.eval(params[0].clone())
    }
}
/// 非安全退出. 
/// 并不保证能够顺利退出. 
/// 当exit调用格式不对时会panic而非exit.
pub fn exit(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.is_empty() {
        process::exit(0);
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <exit>");
    }
    else {
        match params[0].clone() {
            Value::NumericValue(n) => process::exit(n as i32),
            _ => panic!("SyntaxError: Non integer exit code is forbidden"),
        }
    }
}
/// 强制安全退出. 
/// 当出现exit_force调用格式不对时会以127退出码退出.
pub fn exit_force(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.is_empty() {
        process::exit(0);
    }
    else if params.len() > 1 {
        eprint!("SyntaxError: Too many argument in procedure <exit>");
        process::exit(127);
    }
    else {
        match params[0].clone() {
            Value::NumericValue(n) => process::exit(n as i32),
            _ => { 
                eprint!("SyntaxError: Non integer exit code is forbidden");
                process::exit(127);
            }
        }
    }
}
pub fn newline(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.is_empty() {
        println!();
        Value::NilValue
    }
    else {
        panic!("SyntaxError: Cannot append argument to procedure <newline>.");
    }
}

pub fn atom_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <atom?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <atom?>.");
    }
    else {
        match params[0] {
            Value::BooleanValue(_) => return Value::BooleanValue(true),
            Value::NumericValue(_) => return Value::BooleanValue(true),
            Value::StringValue(_) => return Value::BooleanValue(true),
            Value::SymbolValue(_) => return Value::BooleanValue(true),
            Value::NilValue => return Value::BooleanValue(true),
            _ => return Value::BooleanValue(false),
        }
    }
}
pub fn boolean_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <boolean?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <boolean?>.");
    }
    else {
        match params[0] {
            Value::BooleanValue(_) => return Value::BooleanValue(true),
            _ => return Value::BooleanValue(false),
        }
    }
}
pub fn integer_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <integer?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <integer?>.");
    }
    else {
        match params[0].clone() {
            Value::NumericValue(n) => {
                if is_integer(&n) {
                    return Value::BooleanValue(true);
                }
                else {
                    return Value::BooleanValue(false);
                }
            },
            _ => return Value::BooleanValue(false),
        }
    }
}
pub fn list_or_not(params: Vec<Value>, env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <list?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <list?>.");
    }
    else {
        match params[0].clone() {
            Value::NilValue => return Value::BooleanValue(true),
            Value::PairValue(_, cdr) => return list_or_not(vec![*cdr], env),
            _ => return Value::BooleanValue(false),
        }
    }
}
pub fn number_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <number?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <number?>.");
    }
    else {
        match params[0] {
            Value::NumericValue(_) => return Value::BooleanValue(true),
            _ => return Value::BooleanValue(false),
        }
    }
}
pub fn null_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <null?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <null?>.");
    }
    else {
        match params[0] {
            Value::NilValue => return Value::BooleanValue(true),
            _ => return Value::BooleanValue(false),
        }
    }
}
pub fn pair_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <pair?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <pair?>.");
    }
    else {
        match params[0] {
            Value::PairValue(_, _) => return Value::BooleanValue(true),
            _ => return Value::BooleanValue(false),
        }
    }
}
pub fn procedure_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <procedure?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <procedure?>.");
    }
    else {
        match params[0] {
            Value::ProcedureValue(_) => return Value::BooleanValue(true),
            Value::LambdaValue(_, _, _) => return Value::BooleanValue(true),
            _ => return Value::BooleanValue(false),
        }
    }
}
pub fn string_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <string?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <string?>.");
    }
    else {
        match params[0] {
            Value::StringValue(_) => return Value::BooleanValue(true),
            _ => return Value::BooleanValue(false),
        }
    }
}
pub fn symbol_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <symbol?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <symbol?>.");
    }
    else {
        match params[0] {
            Value::SymbolValue(_) => return Value::BooleanValue(true),
            _ => return Value::BooleanValue(false),
        }
    }
}
/// 自己拓展的功能
/// 检查某个符号是否已经在当前环境绑定
pub fn defined_local_or_not(params: Vec<Value>, env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <defined_local?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <defined_local?>.");
    }
    else {
        if env.symbol_map.borrow().contains_key(&params[0].to_string()) {
            return Value::BooleanValue(true);
        }
        else {
            return Value::BooleanValue(false);
        }
    }
}
/// 自己拓展的功能
/// 检查某个符号是否已经在所有可见环境内绑定
pub fn defined_all_or_not(params: Vec<Value>, env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <defined_all?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <defined_all?>.");
    }
    else {
        let bind = env.find_binding(&params[0].to_string());
        if bind.is_some() {
            return Value::BooleanValue(true);
        }
        else {
            return Value::BooleanValue(false);
        }
    }
}
/// ( append list1 ... ) 内置过程
/// 将 list 内的元素按顺序拼接为一个新的列表. 
/// 返回值:拼接后的列表
/// 实参个数为零时返回空表。
pub fn append(params: Vec<Value>, env: &EvalEnv) -> Value {
    let mut ret: Vec<Value> = Vec::new();
    for param in params {
        match param {
            Value::NilValue => (),
            Value::PairValue(_, _) => {
                // 注意这里可能逻辑实现有错误, 如果发生错误请立刻改正为忠实翻译
                let result = panic::catch_unwind(|| {
                    param.to_vector()
                });
                if result.is_ok() {
                    ret.append(result.unwrap().as_mut());
                }
                else {
                    panic!("Cannot append a procedure value.");
                }
            },
            _ => panic!("Cannot append a procedure value."),
        }
    }
    list(ret, env)
}
/// ( push list value ) 自定义过程
/// 将 value 加入到 list 末尾
/// value 只可以是原子类型.
/// value 是空表的时候将不进行任何操作
/// value 是过程类型与lambda类型时将报错
pub fn push(params: Vec<Value>, env: &EvalEnv) -> Value {
    let mut ret: Vec<Value> = Vec::new();
    for param in params {
        match param {
            Value::NilValue => (),
            Value::PairValue(_, _) => {
                // 注意这里可能逻辑实现有错误, 如果发生错误请立刻改正为忠实翻译
                let result = panic::catch_unwind(|| {
                    param.to_vector()
                });
                if result.is_ok() {
                    ret.append(result.unwrap().as_mut());
                }
                else {
                    panic!("Cannot append a procedure value.");
                }
            },
            Value::BooleanValue(_) => ret.push(param),
            Value::NumericValue(_) => ret.push(param),
            Value::StringValue(_) => ret.push(param),
            Value::SymbolValue(_) => ret.push(param),
            _ => panic!("Cannot append a procedure value."),
        }
    }
    list(ret, env)
}
pub fn car(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <car>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <car>.");
    }
    else {
        match params[0].clone() {
            Value::PairValue(car, _) => return *car,
            _ => panic!("Cannot get car of a non-pair/list type value."),
        }
    }
}
pub fn cdr(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <car>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <car>.");
    }
    else {
        match params[0].clone() {
            Value::PairValue(_, cdr) => return *cdr,
            _ => panic!("Cannot get car of a non-pair/list type value."),
        }
    }
}
pub fn cons(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 2 {
        panic!("SyntaxError: Missing argument in procedure <cons>.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument in procedure <cons>.");
    }
    else {
        Value::PairValue(Box::new(params[0].clone()), Box::new(params[1].clone()))
    }
}
pub fn length(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <length>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <length>.");
    } 
    else {
        match params[0] {
            Value::PairValue(_, _) => {
                let vec: Vec<Value> = params[0].to_vector();
                if vec.len() == 1  {
                    match vec[0] {
                        Value::NilValue => return Value::NumericValue(0f64),
                        _ => {},
                    }
                }
                return Value::NumericValue(vec.len() as f64);
            },
            _ => {
                panic!("TypeError. Cannot get length of a non-list value.");
            },
        }
    }
}
pub fn list(params: Vec<Value>, env: &EvalEnv) -> Value {
    if params.is_empty() {
        Value::NilValue
    }
    else {
        Value::PairValue(Box::new(params[0].clone()), Box::new(list(params[1..].to_vec(), env)))
    }
}
pub fn map(params: Vec<Value>, env: &EvalEnv) -> Value {
    if params.len() < 2 {
        panic!("SyntaxError: Missing argument in procedure <map>.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument in procedure <map>.");
    }
    else {
        let args = panic::catch_unwind(|| {
            params[1].to_vector()
        });
        if args.is_ok() {
            let mut results: Vec<Value> = Vec::new();
            match params[0].clone() {
                Value::ProcedureValue(f) => {
                    args.unwrap().iter().clone().for_each(|arg| results.push(f(vec![arg.clone()], env)));
                    return list(results, env);
                }
                Value::LambdaValue(params, body, env_in_lambda) => {
                    args.unwrap().iter().clone().for_each(|arg| results.push(
                        {
                            let args_in_lambda = vec![arg.clone()];
                            let env_derived = env_in_lambda.derive(*params.clone(), args_in_lambda);
                            let mut result: Value = Value::NilValue;
                            for bodyv in *body.clone() {
                                result = env_derived.eval(bodyv);
                            }
                            result
                        }
                    ));
                    return list(results, env);
                },
                _ => panic!("Error type."),
            }
        }
        else {
            panic!("Cannot map a non-list value.");
        }
    }
}
pub fn map_expand(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn filter(params: Vec<Value>, env: &EvalEnv) -> Value {
    if params.len() < 2 {
        panic!("SyntaxError: Missing argument in procedure <filter>.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument in procedure <filter>.");
    }
    else {
        let args = panic::catch_unwind(|| {
            params[1].to_vector()
        });
        if args.is_ok() {
            let mut results: Vec<Value> = Vec::new();
            match params[0].clone() {
                Value::ProcedureValue(f) => {
                    for arg in args.unwrap() {
                        let result: Value = f(vec![arg.clone()], env);
                        match result {
                            Value::BooleanValue(false) => {},
                            _ => results.push(arg.clone()),
                        }
                    }
                    return list(results, env);
                }
                /*Value::LambdaValue(_, _) => {
                    todo!();
                },*/
                Value::LambdaValue(params, body, env_in_lambda) => {
                    for arg in args.unwrap() {
                        let args_in_lambda = vec![arg];
                        let env_derived = env_in_lambda.derive(*params.clone(), args_in_lambda);
                        let mut result: Value = Value::NilValue;
                        for bodyv in *body.clone() {
                            result = env_derived.eval(bodyv);
                        }
                        match result {
                            Value::BooleanValue(false) => continue,
                            _ => results.push(result),
                        }
                    }
                    return list(results, env);
                },
                _ => panic!("SyntaxError: need a procedure and a list."),
            }
        }
        else {
            panic!("SyntaxError: need a procedure and a list.");
        }
    }
}
pub fn reduce(params: Vec<Value>, env: &EvalEnv) -> Value {
    if params.len() < 2 {
        panic!("SyntaxError: Missing argument in procedure <reduce>.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument in procedure <reduce>.");
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::ProcedureValue(f), Value::PairValue(car, cdr)) => {
                match *cdr {
                    Value::NilValue => return *car,
                    _ => {
                        let args: Vec<Value> = vec![*car, reduce(vec![params[0].clone(), *cdr], env)];
                        return f(args, env);
                    },
                }
            },
            (Value::LambdaValue(params_in_lambda, body, env_in_lambda), Value::PairValue(car, cdr)) => {
                match *cdr {
                    Value::NilValue => return *car,
                    _ => {
                        let args: Vec<Value> = vec![*car, reduce(vec![params[0].clone(), *cdr], env)];
                        let env_derived = env_in_lambda.derive(*params_in_lambda, args);
                        let mut result: Value = Value::NilValue;
                        for bodyv in *body.clone() {
                            result = env_derived.eval(bodyv);
                        }
                        return result;
                    }
                }
            },
            _ => panic!("SyntaxError: need a procedure and a list."),
        }
    }
}

pub fn add(params: Vec<Value>, _env: &EvalEnv) -> Value {
    let mut result: f64 = 0f64;
    for param in params {
        match param {
            Value::NumericValue(n) => result += n,
            _ => panic!("Cannot add a non-numeric value."),
        }
    }
    Value::NumericValue(result)
}
pub fn subtract(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <->.");
    }
    else if params.len() == 1 {
        match params[0].clone() {
            Value::NumericValue(n) => return Value::NumericValue(-n),
            _ => panic!("Cannot subtract a non-numeric value."),
        }
    }
    else if params.len() == 2 {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n1), Value::NumericValue(n2)) => return Value::NumericValue(n1 - n2),
            _ => panic!("Cannot do subtraction with a non-numeric value."),
        }
    }
    else {
        panic!("SyntaxError: Too many argument in procedure <->.");
    }
}
pub fn multiply(params: Vec<Value>, _env: &EvalEnv) -> Value {
    let mut ret: f64 = 1f64;
    for param in params {
        match param {
            Value::NumericValue(n) => ret *= n,
            _ => panic!("Cannot multiply a non-numeric value."),
        }
    }
    Value::NumericValue(ret)
}
pub fn divide(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure </>.");
    }
    else if params.len() == 1 {
        match params[0].clone() {
            Value::NumericValue(n) if n != 0f64 => return Value::NumericValue(1f64 / n),
            Value::NumericValue(n) if n == 0f64 => panic!("Division by zero."),
            _ => panic!("Cannot divide a non-numeric value."),
        }
    }
    else if params.len() == 2 {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n1), Value::NumericValue(n2)) => return Value::NumericValue(n1 - n2),
            _ => panic!("Cannot do division with a non-numeric value."),
        }
    }
    else {
        panic!("SyntaxError: Too many argument in procedure </>.");
    }
}
pub fn abs(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <abs>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <abs>.");
    }
    else {
        match params[0] {
            Value::NumericValue(n) => return Value::NumericValue(n.abs()),
            _ => panic!("Cannot do abs to a non-numeric value."),
        }
    }
}
pub fn expt(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 2 {
        panic!("SyntaxError: Missing argument in procedure <expt>.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument in procedure <expt>.");
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(base), Value::NumericValue(expo)) if base != 0f64 && expo != 0f64 => return Value::NumericValue(base.powf(expo)),
            (Value::NumericValue(base), Value::NumericValue(expo)) if base == 0f64 && expo == 0f64 => panic!("Cannot calculate 0^0."),
            _ => panic!("Cannot do exptential with non-numeric values"),
            
        }
    }
}
pub fn quotient(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 2 {
        panic!("SyntaxError: Missing argument in procedure <quotient>.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument in procedure <quotient>.");
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) if n1 != 0f64 => return Value::NumericValue((n0 / n1) as i64 as f64),
            (Value::NumericValue(_), Value::NumericValue(n1)) if n1 == 0f64 => panic!("Division by zero."),
            _ => panic!("Cannot do quotient with non-numeric values"),
            
        }
    }
}
pub fn modulo(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 2 {
        panic!("SyntaxError: Missing argument in procedure <modulo>.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument in procedure <modulo>.");
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) if n1 != 0f64 => {
                if is_integer(&n0) && is_integer(&n1) {
                    let ans: f64 = n0 % n1;
                    if ans == 0f64 || n1 * ans > 0f64 {
                        return Value::NumericValue(ans);
                    }
                    else {
                        return Value::NumericValue(ans + n1);
                    }
                }
                else {
                    panic!("Cannot do modulo with non-integer values.");
                }
            }
            (Value::NumericValue(_), Value::NumericValue(n1)) if n1 == 0f64 => panic!("Division by zero."),
            _ => panic!("Cannot do modulo with non-numeric values"),
        }
    }
}
pub fn remainder(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 2 {
        panic!("SyntaxError: Missing argument in procedure <remainder>.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument in procedure <remainder>.");
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) if n1 != 0f64 => {
                if is_integer(&n0) && is_integer(&n1) {
                    return Value::NumericValue(n0 % n1);
                }
                else {
                    panic!("Cannot do remainder with non-integer values.");
                }
            }
            (Value::NumericValue(_), Value::NumericValue(n1)) if n1 == 0f64 => panic!("Division by zero."),
            _ => panic!("Cannot do remainder with non-numeric values"),
        }
    }
}
pub fn eq_q(params: Vec<Value>, env: &EvalEnv) -> Value {
    if params.len() < 2 {
        panic!("SyntaxError: Missing argument in procedure <eq?>.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument in procedure <eq?>.");
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) => return Value::BooleanValue(n0 == n1),
            (Value::BooleanValue(b0), Value::BooleanValue(b1)) => return Value::BooleanValue(b0 == b1),
            (Value::NilValue, Value::NilValue) => return Value::BooleanValue(true),
            (Value::SymbolValue(s0), Value::SymbolValue(s1)) => return Value::BooleanValue(s0 == s1),
            (Value::StringValue(s0), Value::StringValue(s1)) => return Value::BooleanValue(s0 == s1),
            (Value::PairValue(car0, cdr0), Value::PairValue(car1, cdr1)) => {
                match eq_q(vec![*car0, *car1].to_vec(), env) {
                    v @ Value::BooleanValue(false) => return v,
                    Value::BooleanValue(true) => return eq_q(vec![*cdr0, *cdr1].to_vec(), env),
                    _ => panic!("You should never see this message."),
                }
            },
            (Value::ProcedureValue(f0), Value::ProcedureValue(f1)) => 
                return Value::BooleanValue(std::ptr::eq(&*f0, &*f1)),
            // 我直接规定, 任何两个lambda表达式都是不一样的! 如何?!
            (Value::LambdaValue(params_in_lambda_0, body0, env_in_lambda_0), Value::LambdaValue(params_in_lambda_1, body1, env_in_lambda1)) => return Value::BooleanValue(false),
            _ => return Value::BooleanValue(false),
        }
    }
}
pub fn equal_q(params: Vec<Value>, env: &EvalEnv) -> Value { todo!(); }
pub fn not(params: Vec<Value>, env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <not>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <not>.");
    }
    else {
        let result = std::panic::catch_unwind(||
            env.eval(params[0].clone())
        );
        if result.is_ok() {
            match result.unwrap() {
                Value::BooleanValue(false) => return Value::BooleanValue(true),
                Value::BooleanValue(true) => return Value::BooleanValue(false),
                _ => panic!("Error in procedure <not>."),
            }
        }
        else {
            match params[0] {
                Value::NilValue => return Value::BooleanValue(false),
                Value::PairValue(_, _) => return Value::BooleanValue(false),
                Value::SymbolValue(_) => return Value::BooleanValue(false),
                _ => panic!("Error in procedure <not>"),
            }
        }
    }
}
pub fn equal_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 2 {
        panic!("SyntaxError: Missing argument in procedure < = >.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument in procedure < = >.");
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) => return Value::BooleanValue(n0 == n1),
            _ => panic!("Cannot compare two non-numeric values"),
        } 
    }
}
pub fn less_than_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 2 {
        panic!("SyntaxError: Missing argument in procedure < < >.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument in procedure < < >.");
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) => return Value::BooleanValue(n0 < n1),
            _ => panic!("Cannot compare two non-numeric values"),
        } 
    }
}
pub fn more_than_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 2 {
        panic!("SyntaxError: Missing argument in procedure < > >.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument in procedure < > >.");
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) => return Value::BooleanValue(n0 > n1),
            _ => panic!("Cannot compare two non-numeric values"),
        } 
    }
}
pub fn less_than_or_equal_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 2 {
        panic!("SyntaxError: Missing argument in procedure < <= >.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument in procedure < <= >.");
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) => return Value::BooleanValue(n0 <= n1),
            _ => panic!("Cannot compare two non-numeric values"),
        } 
    }
}
pub fn more_than_or_equal_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 2 {
        panic!("SyntaxError: Missing argument in procedure < >= >.");
    }
    else if params.len() > 2 {
        panic!("SyntaxError: Too many argument in procedure < >= >.");
    }
    else {
        match (params[0].clone(), params[1].clone()) {
            (Value::NumericValue(n0), Value::NumericValue(n1)) => return Value::BooleanValue(n0 >= n1),
            _ => panic!("Cannot compare two non-numeric values"),
        } 
    }
}
pub fn even_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <even?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <even?>.");
    }
    else {
        match params[0] {
            Value::NumericValue(n) => {
                if is_integer(&n) {
                    return Value::BooleanValue(n as i32 % 2 == 0);
                }
                else {
                    panic!("Cannot judge even/odd with a non-integer number.");
                }
            },
            _ => panic!("Cannot compare a non-numeric values."),
        }
    }
}
pub fn odd_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <odd?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <odd?>.");
    }
    else {
        match params[0] {
            Value::NumericValue(n) => {
                if is_integer(&n) {
                    return Value::BooleanValue(n as i32 % 2 == 1);
                }
                else {
                    panic!("Cannot judge even/odd with a non-integer number.");
                }
            },
            _ => panic!("Cannot compare a non-numeric values."),
        }
    }
}
pub fn zero_or_not(params: Vec<Value>, _env: &EvalEnv) -> Value {
    if params.len() < 1 {
        panic!("SyntaxError: Missing argument in procedure <zero?>.");
    }
    else if params.len() > 1 {
        panic!("SyntaxError: Too many argument in procedure <zero?>.");
    }
    else {
        match params[0] {
            Value::NumericValue(n) => return Value::BooleanValue(n == 0f64),
            _ => panic!("Cannot compare a non-numeric values"),
        }
    }
}
pub fn sort(params: Vec<Value>, env: &EvalEnv) -> Value {
    todo!();
}
