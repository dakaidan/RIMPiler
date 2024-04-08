use std::collections::HashMap;
use std::fmt::Display;
use ordered_float::NotNan;
use crate::abstract_machine::stack::{Stack, ResultStack, ControlStack, BackStack, Builder, C, P, Lab, R, Num, Type, Var, BinOp};
use crate::abstract_machine::store::Store;
use crate::AST::Program;
use crate::interpreter::memory_store::{MemoryStoreElement, MemoryStoreTrait, Value};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Rules {
    // expression rules
    Num,
    Mun,
    Var,
    Rav,
    Exp,
    UnExp,
    Pxe,
    UnPxe,
    BinOp,
    BinPo,
    UnOp,
    UnPo,
    // statement rules
    Skip,
    Asgn,
    Ngsa,
    Assign,             // :=
    AsgnR,
    NgsaR,
    Ngissa,             // =:
    Seq,
    Qes,
    Sequence,           // ;
    // conditional rules
    Cond,
    Dnoc,
    IfT,
    FiT,
    IfF,
    FiF,
    EndIf,
    IfRexp,
    // loop rules
    Loop,
    Pool,
    LoopT,
    PoolT,
    LoopF,
    PoolF,
    EndWF,
    WEndF,
    EndWT,
    WEndT,
    EndW,
    WRexp,
}

impl Rules {
    pub fn new(name: &str) -> Option<Self> {
        match name {
            "Num" => Some(Rules::Num),
            "Mun" => Some(Rules::Mun),
            "Var" => Some(Rules::Var),
            "Rav" => Some(Rules::Rav),
            "Exp" => Some(Rules::Exp),
            "UnExp" => Some(Rules::UnExp),
            "Pxe" => Some(Rules::Pxe),
            "UnPxe" => Some(Rules::UnPxe),
            "BinOp" => Some(Rules::BinOp),
            "BinPo" => Some(Rules::BinPo),
            "UnOp" => Some(Rules::UnOp),
            "UnPo" => Some(Rules::UnPo),
            "Skip" => Some(Rules::Skip),
            "Asgn" => Some(Rules::Asgn),
            "Ngsa" => Some(Rules::Ngsa),
            "Assign" => Some(Rules::Assign),
            "AsgnR" => Some(Rules::AsgnR),
            "NgsaR" => Some(Rules::NgsaR),
            "Ngissa" => Some(Rules::Ngissa),
            "Seq" => Some(Rules::Seq),
            "Qes" => Some(Rules::Qes),
            "Sequence" => Some(Rules::Sequence),
            "Cond" => Some(Rules::Cond),
            "Dnoc" => Some(Rules::Dnoc),
            "IfT" => Some(Rules::IfT),
            "FiT" => Some(Rules::FiT),
            "IfF" => Some(Rules::IfF),
            "FiF" => Some(Rules::FiF),
            "EndIf" => Some(Rules::EndIf),
            "IfRexp" => Some(Rules::IfRexp),
            "Loop" => Some(Rules::Loop),
            "Pool" => Some(Rules::Pool),
            "LoopT" => Some(Rules::LoopT),
            "PoolT" => Some(Rules::PoolT),
            "LoopF" => Some(Rules::LoopF),
            "PoolF" => Some(Rules::PoolF),
            "EndWF" => Some(Rules::EndWF),
            "WEndF" => Some(Rules::WEndF),
            "EndWT" => Some(Rules::EndWT),
            "WEndT" => Some(Rules::WEndT),
            "EndW" => Some(Rules::EndW),
            "WRexp" => Some(Rules::WRexp),
            _ => None,
        }
    }

    pub fn all_rules() -> Vec<Rules> {
        vec![
            Rules::Num,
            Rules::Mun,
            Rules::Var,
            Rules::Rav,
            Rules::Exp,
            Rules::UnExp,
            Rules::Pxe,
            Rules::UnPxe,
            Rules::BinOp,
            Rules::BinPo,
            Rules::UnOp,
            Rules::UnPo,
            Rules::Skip,
            Rules::Asgn,
            Rules::Ngsa,
            Rules::Assign,
            Rules::AsgnR,
            Rules::NgsaR,
            Rules::Ngissa,
            Rules::Seq,
            Rules::Qes,
            Rules::Sequence,
            Rules::Cond,
            Rules::Dnoc,
            Rules::IfT,
            Rules::FiT,
            Rules::IfF,
            Rules::FiF,
            Rules::EndIf,
            Rules::IfRexp,
            Rules::Loop,
            Rules::Pool,
            Rules::LoopT,
            Rules::PoolT,
            Rules::LoopF,
            Rules::PoolF,
            Rules::EndWF,
            Rules::WEndF,
            Rules::EndWT,
            Rules::WEndT,
            Rules::EndW,
            Rules::WRexp,
        ]
    }
}

impl Display for Rules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rules::Num => write!(f, "Num"),
            Rules::Mun => write!(f, "Mun"),
            Rules::Var => write!(f, "Var"),
            Rules::Rav => write!(f, "Rav"),
            Rules::Exp => write!(f, "Exp"),
            Rules::UnExp => write!(f, "UnExp"),
            Rules::Pxe => write!(f, "Pxe"),
            Rules::UnPxe => write!(f, "UnPxe"),
            Rules::BinOp => write!(f, "BinOp"),
            Rules::BinPo => write!(f, "BinPo"),
            Rules::UnOp => write!(f, "UnOp"),
            Rules::UnPo => write!(f, "UnPo"),
            Rules::Skip => write!(f, "Skip"),
            Rules::Asgn => write!(f, "Asgn"),
            Rules::Ngsa => write!(f, "Ngsa"),
            Rules::Assign => write!(f, "Assign"),
            Rules::AsgnR => write!(f, "AsgnR"),
            Rules::NgsaR => write!(f, "NgsaR"),
            Rules::Ngissa => write!(f, "Ngissa"),
            Rules::Seq => write!(f, "Seq"),
            Rules::Qes => write!(f, "Qes"),
            Rules::Sequence => write!(f, "Sequence"),
            Rules::Cond => write!(f, "Cond"),
            Rules::Dnoc => write!(f, "Dnoc"),
            Rules::IfT => write!(f, "IfT"),
            Rules::FiT => write!(f, "FiT"),
            Rules::IfF => write!(f, "IfF"),
            Rules::FiF => write!(f, "FiF"),
            Rules::EndIf => write!(f, "EndIf"),
            Rules::IfRexp => write!(f, "IfRexp"),
            Rules::Loop => write!(f, "Loop"),
            Rules::Pool => write!(f, "Pool"),
            Rules::LoopT => write!(f, "LoopT"),
            Rules::PoolT => write!(f , "PoolT"),
            Rules::LoopF => write!(f, "LoopF"),
            Rules::PoolF => write!(f, "PoolF"),
            Rules::EndWF => write!(f, "EndWF"),
            Rules::WEndF => write!(f, "WEndF"),
            Rules::EndWT => write!(f, "EndWT"),
            Rules::WEndT => write!(f, "WEndT"),
            Rules::EndW => write!(f, "EndW"),
            Rules::WRexp => write!(f, "WRexp"),
        }
    }
}

pub struct Engine {
    pub control_stack: ControlStack,
    pub back_stack: BackStack,
    pub result_stack: ResultStack,
    pub store: Store,

    pub while_condition: HashMap<usize, P>,
}

impl Engine {
    pub fn new(ast: Program) -> Engine {
        let cs = Builder::new().from_ast(ast);
        Engine {
            control_stack: cs.clone(),
            back_stack: Stack::new(),
            result_stack: Stack::new(),
            store: Store::new(),
            while_condition: Self::while_map(cs),
        }
    }

    pub fn get_control_stack(&self) -> &ControlStack {
        &self.control_stack
    }

    pub fn get_back_stack(&self) -> &BackStack {
        &self.back_stack
    }

    pub fn get_result_stack(&self) -> &ResultStack {
        &self.result_stack
    }

    pub fn get_store(&self) -> &Store {
        &self.store
    }

    pub fn get_next_rule(&self) -> Rules {
        let rule = self.check_rule();
        rule
    }

    pub fn step(&mut self) {
        let rule = self.check_rule();
        match rule {
            // Expressions
            Rules::Num => {
                // (n · c, r, m, b) −→ (c, n · r, m, n' · b)
                let n = self.control_stack.pop().unwrap();
                let number = n.unwrap_p().unwrap_num();
                self.result_stack.push(R::Value(number.clone()));
                self.back_stack.push(C::P(P::Mun(number.clone())));
            },
            Rules::Mun => {
                // (n' · b, n · r, m, c) −→ (b, r, m, n · c)
                let n = self.control_stack.pop().unwrap();
                let number = n.unwrap_p().unwrap_mun();
                // we assume that the state is correct and r is number'
                let r = self.result_stack.pop().unwrap();
                self.back_stack.push(C::P(P::Num(number.clone())));
            },
            Rules::Var => {
                // (!l · c, r, m, b) −→ (c, m1 (l) · r, m, !l' · b)
                let v = self.control_stack.pop().unwrap();
                let var = v.unwrap_p().unwrap_var();
                let (r#type, name) = var.unwrap();

                let value = match r#type {
                    Type::Int => {
                        let value = self.store.get(&name);
                        match value {
                            Some(MemoryStoreElement::Integer(i)) => R::Value(Num::Int(i.get())),
                            None => {
                                // assign initial value (0)
                                self.store.assign(&name, Value::Integer(0));
                                R::Value(Num::Int(0))
                            }
                            _ => panic!("Variable Type mismatch")
                        }
                    }
                    Type::Float => {
                        let value = self.store.get(&name);
                        match value {
                            Some(MemoryStoreElement::Float(f)) => {
                                let v: NotNan<f32> = NotNan::new(f.get()).unwrap();
                                R::Value(Num::Float(v))
                            },
                            None => {
                                // assign initial value (0)
                                self.store.assign(&name, Value::Float(0.0));
                                R::Value(Num::Float(NotNan::new(0.0).unwrap()))
                            }
                            _ => panic!("Variable Type mismatch")
                        }
                    }
                };

                self.back_stack.push(C::P(P::Rav(var.clone())));
                self.result_stack.push(value);
            },
            Rules::Rav => {
                // (!l' · b, n · r, m, c) −→ (b, r, m, !l · c)
                let v = self.control_stack.pop().unwrap();
                let rav = v.unwrap_p().unwrap_rav();
                let n = self.result_stack.pop().unwrap();

                self.back_stack.push(C::P(P::Var(rav.clone())));
            },
            Rules::Exp => {
                // ((E1 oper E2 ) · c, r, m, b) −→ (E1 · E2 · oper · c, r, m, exp · E1' · E2' · b)
                // where oper = op or op'
                let v = self.control_stack.pop().unwrap();
                let (E1, E2, op) = v.unwrap_p().unwrap_binop();

                self.control_stack.push(C::Lab(Lab::BinOp(op.clone())));
                self.control_stack.push(C::P(E2.clone()));
                self.control_stack.push(C::P(E1.clone()));

                self.back_stack.push(C::P(P::Rexp(Box::new(E2.clone()))));
                self.back_stack.push(C::P(P::Rexp(Box::new(E1.clone()))));
                self.back_stack.push(C::Lab(Lab::Exp));
            },
            Rules::Pxe => {
                // (exp · E1' · E2' · b, r, m, E1 · E2 · op · c) −→ (b, r, m, (E1 op E2 ) · c)
                let exp = self.control_stack.pop().unwrap();
                let E1_ = self.control_stack.pop().unwrap().unwrap_p().unwrap_rexp();
                let E2_ = self.control_stack.pop().unwrap().unwrap_p().unwrap_rexp();

                let E1 = self.back_stack.pop().unwrap();
                let E1 = E1.unwrap_p();
                let E2 = self.back_stack.pop().unwrap();
                let E2 = E2.unwrap_p();

                let op = self.back_stack.pop().unwrap();
                let op = op.unwrap_lab().unwrap_binop();

                self.back_stack.push(C::P(P::BinOp(Box::new(E1.clone()), Box::new(E2.clone()), op.clone())));
            },
            Rules::BinOp => {
                // (op · c, n2 · n1 · r, m, E2' · E1' · exp · E1' · E2' · b) −→ (c, n · r, m, (E1 op' E2 ) · b)
                // where n = n1 op n2
                let op = self.control_stack.pop().unwrap();
                let op = op.unwrap_lab().unwrap_binop();
                let n2 = self.result_stack.pop().unwrap();
                let n2 = n2.unwrap_value();
                let n1 = self.result_stack.pop().unwrap();
                let n1 = n1.unwrap_value();

                let E2_ = self.back_stack.pop().unwrap();
                let E1_ = self.back_stack.pop().unwrap();
                let E2_ = E2_.unwrap_p().unwrap_rexp();
                let E1_ = E1_.unwrap_p().unwrap_rexp();
                let exp = self.back_stack.pop().unwrap().unwrap_lab();
                let E1_ = self.back_stack.pop().unwrap();
                let E1_ = E1_.unwrap_p().unwrap_rexp();
                let E2_ = self.back_stack.pop().unwrap();
                let E2_ = E2_.unwrap_p().unwrap_rexp();

                let n = op.apply(n1.clone(), n2.clone());
                self.result_stack.push(R::Value(n));
                self.back_stack.push(C::P(P::Rexp(Box::new(P::BinOp(Box::new(E1_.clone()), Box::new(E2_.clone()), op.clone())))));
            },
            Rules::BinPo => {
                // (op' · b, n2 · n1 · n · r, m, E2' · E1' · exp · E1' · E2' · c) −→ (b, r, m, (E1 opE2 ) · c)
                // where n = n1 op n2
                let op = self.control_stack.pop().unwrap();
                let op = op.unwrap_lab().unwrap_binpo();

                let n2 = self.result_stack.pop().unwrap();
                let n2 = n2.unwrap_value();
                let n1 = self.result_stack.pop().unwrap();
                let n1 = n1.unwrap_value();
                let n = self.result_stack.pop().unwrap();
                let n = n.unwrap_value();

                let E2_ = self.back_stack.pop().unwrap().unwrap_p().unwrap_rexp();
                let E1_ = self.back_stack.pop().unwrap().unwrap_p().unwrap_rexp();
                let exp = self.back_stack.pop().unwrap().unwrap_lab();
                let E1_ = self.back_stack.pop().unwrap();
                let E1_ = E1_.unwrap_p().unwrap_rexp();
                let E2_ = self.back_stack.pop().unwrap();
                let E2_ = E2_.unwrap_p().unwrap_rexp();

                self.back_stack.push(C::P(P::BinOp(Box::new(E1_.clone()), Box::new(E2_.clone()), op.clone())));
            },
            Rules::UnExp => {
                // ((oper E ) · c, r, m, b) −→ (E · oper · c, r, m, unexp · E' · b)
                // where oper = op or op'
                let v = self.control_stack.pop().unwrap();
                let (E, op) = v.unwrap_p().unwrap_unop();

                self.control_stack.push(C::Lab(Lab::UnOp(op.clone())));
                self.control_stack.push(C::P(E.clone()));

                self.back_stack.push(C::P(P::Rexp(Box::new(E.clone()))));
                self.back_stack.push(C::Lab(Lab::UnExp));
            },
            Rules::UnPxe => {
                // (exp · E' · b, r, m, E · op · c) −→ (b, r, m, (op E ) · c)
                let exp = self.control_stack.pop().unwrap();
                let E_ = self.control_stack.pop().unwrap().unwrap_p().unwrap_rexp();

                let E = self.back_stack.pop().unwrap();
                let E = E.unwrap_p();
                let op = self.back_stack.pop().unwrap();
                let op = op.unwrap_lab().unwrap_unop();

                self.back_stack.push(C::P(P::UnOp(Box::new(E.clone()), op.clone())));
            },
            Rules::UnOp => {
                // (op · c, n_ · r, m, E' · unexp · E' · b) −→ (c, n · r, m, (op' E) · b)
                // where n = op n_
                let op = self.control_stack.pop().unwrap();
                let op = op.unwrap_lab().unwrap_unop();

                let n_ = self.result_stack.pop().unwrap();
                let n_ = n_.unwrap_value();

                let E_ = self.back_stack.pop().unwrap().unwrap_p().unwrap_rexp();
                let unexp = self.back_stack.pop().unwrap().unwrap_lab();
                let E_ = self.back_stack.pop().unwrap();
                let E_ = E_.unwrap_p().unwrap_rexp();

                let n = op.apply(n_.clone());

                self.result_stack.push(R::Value(n));
                self.back_stack.push(C::P(P::Rexp(Box::new(P::UnOp(Box::new(E_.clone()), op.clone())))));
            },
            Rules::UnPo => {
                // (op' · b, n_ · n · r, m, E' · unexp · E' · c) −→ (b, r, m, (op E) · c)
                // where n = op n_
                let op = self.control_stack.pop().unwrap();
                let op = op.unwrap_lab().unwrap_unpo();

                let n_ = self.result_stack.pop().unwrap();
                let n_ = n_.unwrap_value();
                let n = self.result_stack.pop().unwrap();
                let n = n.unwrap_value();

                let E_ = self.back_stack.pop().unwrap().unwrap_p().unwrap_rexp();
                let unexp = self.back_stack.pop().unwrap().unwrap_lab();
                let E_ = self.back_stack.pop().unwrap();
                let E_ = E_.unwrap_p().unwrap_rexp();

                self.back_stack.push(C::P(P::UnOp(Box::new(E_.clone()), op.clone())));
            },
            // Statements
            Rules::Skip => {
                // (skip · c, r, m, b) −→ (c, r, m, skip · b)
                self.control_stack.pop().unwrap();
                self.back_stack.push(C::P(P::Skip));
            },
            Rules::Asgn => {
                // ((l := E) · c, r, m, b) −→ (E · !l · := · c, l · r, m, asgn · E · b)
                let v = self.control_stack.pop().unwrap();
                let (l, E) = v.unwrap_p().unwrap_asgn();

                self.control_stack.push(C::Lab(Lab::Assign));
                self.control_stack.push(C::P(P::Var(l.clone())));
                self.control_stack.push(C::P(E.clone()));

                self.result_stack.push(R::Var(l.clone()));

                self.back_stack.push(C::P(E.clone()));
                self.back_stack.push(C::Lab(Lab::Asgn));
            },
            Rules::Ngsa => {
                // (asgn · E · b, l · r, m, E· !l · := · c) −→ (b, r, m, (l := E) · c)
                let asgn = self.control_stack.pop().unwrap();
                let E = self.control_stack.pop().unwrap();

                let l = self.result_stack.pop().unwrap();
                let l = l.unwrap_var();

                let E = self.back_stack.pop().unwrap();
                let E = E.unwrap_p();
                let l_ = self.back_stack.pop().unwrap();
                let Assn = self.back_stack.pop().unwrap();

                self.back_stack.push(C::P(P::Asgn(l.clone(), Box::new(E.clone()))));
            },
            Rules::Assign => {
                // (:= ·c, n2 · n1 · l · r, m, !l' · E' · asgn · E · b) −→
                // (c, r, m[l |→ (n1 , +(n, m2(l)))], (l =: E) · b)
                // where n = n1 − n2
                let asgn = self.control_stack.pop().unwrap();

                let n2 = self.result_stack.pop().unwrap();
                let n2 = n2.unwrap_value();
                let n1 = self.result_stack.pop().unwrap();
                let n1 = n1.unwrap_value();
                let l = self.result_stack.pop().unwrap();
                let l = l.unwrap_var();
                let (r#type, name) = l.unwrap();

                let l_ = self.back_stack.pop().unwrap();
                let E_ = self.back_stack.pop().unwrap();
                let asgn_ = self.back_stack.pop().unwrap();
                let E = self.back_stack.pop().unwrap();
                let E = E.unwrap_p();

                let value = match r#type {
                    Type::Int => {
                        Value::Integer(n1.clone().into_int())
                    }
                    Type::Float => {
                        Value::Float(n1.clone().into_float())
                    }
                };

                self.store.assign(&name, value);

                self.back_stack.push(C::P(P::Ngsa(l.clone(), Box::new(E.clone()))));
            },
            Rules::AsgnR => {
                // ((l =: E) · c, r, m[l |→ (n1 , +(n, v))], b) −→
                // (E· !l · =: · c, l · r, m' , asgnr · n · E · b)
                // where m' = m[l |→ (n1 − n, v)]
                let asgnr = self.control_stack.pop().unwrap();
                let (l, E) = asgnr.unwrap_p().unwrap_ngsa();

                self.control_stack.push(C::Lab(Lab::Ngissa));
                self.control_stack.push(C::P(P::Var(l.clone())));
                self.control_stack.push(C::P(E.clone()));

                self.result_stack.push(R::Var(l.clone()));

                let s = self.store.clone();

                let n = s.get(&l.unwrap().1).unwrap();
                let value = match n {
                    MemoryStoreElement::Integer(i) => {
                        Value::Integer(i.get())
                    }
                    MemoryStoreElement::Float(f) => {
                        Value::Float(f.get())
                    }
                };
                self.store.un_assign(&l.unwrap().1, value);

                let n = match n {
                    MemoryStoreElement::Integer(i) => Num::Int(i.get()),
                    MemoryStoreElement::Float(f) => Num::Float(NotNan::new(f.get()).unwrap())
                };

                self.back_stack.push(C::P(E.clone()));
                self.back_stack.push(C::P(P::Num(n)));
                self.back_stack.push(C::Lab(Lab::Ngsa));
            },
            Rules::NgsaR => {
                // (asgnr · n · E · b, l · r, m, E· !l · =: · c) −→
                // (b, r, m[l |→ (m1 (l) + n, +(n, m2 (l))], (l =: E) · c)
                let asgnr = self.control_stack.pop().unwrap();
                let n = self.control_stack.pop().unwrap();
                let n = n.unwrap_p().unwrap_num();
                let E = self.control_stack.pop().unwrap();

                let l = self.result_stack.pop().unwrap();
                let l = l.unwrap_var();

                let E = self.back_stack.pop().unwrap();
                let E = E.unwrap_p();
                let l_ = self.back_stack.pop().unwrap();
                let asgn = self.back_stack.pop().unwrap();

                let (r#type, name) = l.unwrap();

                let value = match r#type {
                    Type::Int => {
                        Value::Integer(n.clone().into_int())
                    }
                    Type::Float => {
                        Value::Float(n.clone().into_float())
                    }
                };

                self.store.assign(&name, value);

                self.back_stack.push(C::P(P::Ngsa(l.clone(), Box::new(E.clone()))));
            },
            Rules::Ngissa => {
                // (=: ·c, n2 · n1 · l · r, m' , !l' · E' · asgnr · n · E · b) −→ (c, r, m' , (l := E) · b) if n = n1 − n2
                let ngissa = self.control_stack.pop().unwrap();

                let n2 = self.result_stack.pop().unwrap();
                let n2 = n2.unwrap_value();
                let n1 = self.result_stack.pop().unwrap();
                let n1 = n1.unwrap_value();
                let l = self.result_stack.pop().unwrap();
                let l = l.unwrap_var();

                let l_ = self.back_stack.pop().unwrap();
                let E_ = self.back_stack.pop().unwrap();
                let asgnr = self.back_stack.pop().unwrap();
                let n = self.back_stack.pop().unwrap();
                let n = n.unwrap_p().unwrap_num();
                let E = self.back_stack.pop().unwrap();
                let E = E.unwrap_p();

                self.back_stack.push(C::P(P::Asgn(l.clone(), Box::new(E.clone()))));
            },
            Rules::Seq => {
                // ((C1 ; C2 ) · c, r, m, b) −→ (C1 · C2 · ; ·c, r, m, seq · b)
                let seq = self.control_stack.pop().unwrap();
                let (C1, C2) = seq.unwrap_p().unwrap_seq();

                self.control_stack.push(C::Lab(Lab::Sequence));
                self.control_stack.push(C::P(C2.clone()));
                self.control_stack.push(C::P(C1.clone()));

                self.back_stack.push(C::Lab(Lab::Seq));
            },
            Rules::Qes => {
                // (seq · b, r, m, C1 · C2 ·; ·c) −→ (b, r, m, (C1 ; C2 ) · c)
                let seq = self.control_stack.pop().unwrap();

                let C1 = self.back_stack.pop().unwrap();
                let C1 = C1.unwrap_p();
                let C2 = self.back_stack.pop().unwrap();
                let C2 = C2.unwrap_p();
                let seq_ = self.back_stack.pop().unwrap();

                self.back_stack.push(C::P(P::Seq(Box::new(C1.clone()), Box::new(C2.clone()))));
            },
            Rules::Sequence => {
                // (; ·c, r, m, rev(C2) · rev(C1) · seq · b) −→ (c, r, m, (rev(C2 ); rev(C1 )) · b)
                let seq = self.control_stack.pop().unwrap();

                let rev_C2 = self.back_stack.pop().unwrap();
                let rev_C2 = rev_C2.unwrap_p();
                let rev_C1 = self.back_stack.pop().unwrap();
                let rev_C1 = rev_C1.unwrap_p();
                let seq_ = self.back_stack.pop().unwrap();

                self.back_stack.push(C::P(P::Seq(Box::new(rev_C2.clone()), Box::new(rev_C1.clone()))));
            },
            // Conditionals
            Rules::Cond => {
                // ((if E then C1 else C2 ) · c, r, m, b) −→ (E · if · cond · c, C1 · C2 · r, m, cond' · b)
                let if_else = self.control_stack.pop().unwrap();
                let (E, C1, C2) = if_else.unwrap_p().unwrap_if();

                self.control_stack.push(C::Lab(Lab::Cond));
                self.control_stack.push(C::Lab(Lab::If));
                self.control_stack.push(C::P(E.clone()));

                self.result_stack.push(R::P(C2.clone()));
                self.result_stack.push(R::P(C1.clone()));

                self.back_stack.push(C::Lab(Lab::Dnoc));
            },
            Rules::Dnoc => {
                // (cond' · b, C1 · C2 · r, m, E · if · cond · c) −→ (b, r, m, (if E then C1 else C2 ) · c)
                let dnoc = self.control_stack.pop().unwrap();

                let C1 = self.result_stack.pop().unwrap();
                let C2 = self.result_stack.pop().unwrap();

                let E_ = self.back_stack.pop().unwrap();
                let if_ = self.back_stack.pop().unwrap();
                let cond = self.back_stack.pop().unwrap();

                let E = E_.unwrap_p();
                let C1 = C1.unwrap_p();
                let C2 = C2.unwrap_p();

                self.back_stack.push(C::P(P::If(Box::new(E.clone()), Box::new(C1.clone()), Box::new(C2.clone()))));
            },
            Rules::IfT => {
                // (if · cond · c, true · C1 · C2 · r, m, E' · cond' · b) −→ (C1 · cond · c, C1 · C2 · r, m, E · if' · cond' · b)
                let if_ = self.control_stack.pop().unwrap();
                let cond = self.control_stack.pop().unwrap();

                let true_ = self.result_stack.pop().unwrap();
                let C1 = self.result_stack.pop().unwrap();
                let C2 = self.result_stack.pop().unwrap();

                let E_ = self.back_stack.pop().unwrap();
                let cond_ = self.back_stack.pop().unwrap();

                self.control_stack.push(C::Lab(Lab::Cond));
                let C1 = C1.unwrap_p();
                self.control_stack.push(C::P(C1.clone()));

                let C2 = C2.unwrap_p();
                self.result_stack.push(R::P(C2.clone()));
                self.result_stack.push(R::P(C1.clone()));

                let E = E_.unwrap_p().unwrap_rexp();
                self.back_stack.push(C::Lab(Lab::Dnoc));
                self.back_stack.push(C::Lab(Lab::Fi));
                self.back_stack.push(C::P(E.clone()));
            },
            Rules::FiT => {
                // (if' · cond' · b, true · C1 · C2 · r, m, E' · C1 · cond · c) −→ (E' · cond' · b, true · C1 · C2 · r, m, if · cond · c)
                let fi = self.control_stack.pop().unwrap();
                let cond_ = self.control_stack.pop().unwrap();

                let true_ = self.result_stack.pop().unwrap();
                let C1 = self.result_stack.pop().unwrap();
                let C2 = self.result_stack.pop().unwrap();

                let E_ = self.back_stack.pop().unwrap();
                let C1_ = self.back_stack.pop().unwrap();
                let cond = self.back_stack.pop().unwrap();

                self.control_stack.push(cond_);
                self.control_stack.push(E_);

                let C1 = C1.unwrap_p();
                let C2 = C2.unwrap_p();
                let true_ = true_.unwrap_value();
                self.result_stack.push(R::P(C2.clone()));
                self.result_stack.push(R::P(C1.clone()));
                self.result_stack.push(R::Value(true_.clone()));

                self.back_stack.push(C::Lab(Lab::Cond));
                self.back_stack.push(C::Lab(Lab::If));
            },
            Rules::IfF => {
                // (if · cond · c, false · C1 · C2 · r, m, E' · cond' · b) −→ (C2 · cond · c, C1 · C2 · r, m, E · if' · cond' · b)
                let if_ = self.control_stack.pop().unwrap();
                let cond = self.control_stack.pop().unwrap();

                let false_ = self.result_stack.pop().unwrap();
                let C1 = self.result_stack.pop().unwrap();
                let C2 = self.result_stack.pop().unwrap();

                let E_ = self.back_stack.pop().unwrap();
                let cond_ = self.back_stack.pop().unwrap();

                self.control_stack.push(cond);
                let C2 = C2.unwrap_p();
                self.control_stack.push(C::P(C2.clone()));

                let C1 = C1.unwrap_p();
                self.result_stack.push(R::P(C2.clone()));
                self.result_stack.push(R::P(C1.clone()));

                let E = E_.unwrap_p().unwrap_rexp();
                self.back_stack.push(C::Lab(Lab::Dnoc));
                self.back_stack.push(C::Lab(Lab::Fi));
                self.back_stack.push(C::P(E.clone()));
            },
            Rules::FiF => {
                // (if' · cond' · b, false · C1 · C2 · r, m, E' · C2 · cond · c) −→ (E' · cond · b, false · C1 · C2 · r, m, if · cond · c)
                let fi = self.control_stack.pop().unwrap();
                let cond_ = self.control_stack.pop().unwrap();

                let false_ = self.result_stack.pop().unwrap();
                let C1 = self.result_stack.pop().unwrap();
                let C2 = self.result_stack.pop().unwrap();

                let E_ = self.back_stack.pop().unwrap();
                let C2_ = self.back_stack.pop().unwrap();
                let cond = self.back_stack.pop().unwrap();

                self.control_stack.push(cond);
                self.control_stack.push(E_);

                let C1 = C1.unwrap_p();
                let C2 = C2.unwrap_p();
                self.result_stack.push(R::P(C2.clone()));
                self.result_stack.push(R::P(C1.clone()));
                self.result_stack.push(R::Value(false_.unwrap_value().clone()));

                self.back_stack.push(C::Lab(Lab::Cond));
                self.back_stack.push(C::Lab(Lab::If));
            },
            Rules::EndIf => {
                // (cond · c, C1 · C2 · r, m, rev(C) · E · if' · cond' · b) −→ (c, r, m, (if E then rev(C1) else rev(C2 )) · b)
                // where C is either C1 or C2
                let cond = self.control_stack.pop().unwrap();

                let C1 = self.result_stack.pop().unwrap();
                let C2 = self.result_stack.pop().unwrap();

                let rev_c = self.back_stack.pop().unwrap();
                let E = self.back_stack.pop().unwrap();
                let if_ = self.back_stack.pop().unwrap();
                let cond_ = self.back_stack.pop().unwrap();

                let rev_c1 = self.rev(C1.unwrap_p());
                let rev_c2 = self.rev(C2.unwrap_p());
                let E = E.unwrap_p();

                self.back_stack.push(C::P(P::If(Box::new(E.clone()), Box::new(rev_c1), Box::new(rev_c2))));
            },
            Rules::IfRexp => {
                // (E' cond' · b, v · C1 · C2 · r, m, c) −→ (cond' · b, C1 · C2 · r, m, rev(C) · E · c)
                // where C = C1 if v is true, C2 otherwise
                let v = self.control_stack.pop().unwrap();
                let E_ = v.unwrap_p().unwrap_rexp();

                self.back_stack.push(C::P(E_.clone()));

                let v = self.result_stack.pop().unwrap();
                let C1 = self.result_stack.pop().unwrap();
                let C2 = self.result_stack.pop().unwrap();

                if v.is_truthy() {
                    self.back_stack.push(C::P(self.rev(C1.unwrap_p())));
                } else {
                    self.back_stack.push(C::P(self.rev(C2.unwrap_p())));
                }

                self.result_stack.push(C1);
                self.result_stack.push(C2);
            }
            // Loops
            Rules::Loop => {
                // ((whilei E do C) · c, r, m, b) −→ (E · whilei · loopi · c, E · C · r, m, loopi' · b)
                let loop_ = self.control_stack.pop().unwrap();

                let (E, C, i) = loop_.unwrap_p().unwrap_while();

                self.control_stack.push(C::Lab(Lab::Loop(i)));
                self.control_stack.push(C::Lab(Lab::While(i)));
                self.control_stack.push(C::P(E.clone()));

                self.result_stack.push(R::P(C.clone()));
                self.result_stack.push(R::P(E.clone()));

                self.back_stack.push(C::Lab(Lab::Pool(i)));
            },
            Rules::Pool => {
                // (loopi' · b, E · C · r, m, E · whilei · loopi · c) −→ (b, r, m, (whilei E do C) · c)
                let pool = self.control_stack.pop().unwrap();

                let E = self.result_stack.pop().unwrap();
                let C = self.result_stack.pop().unwrap();

                let E_ = self.back_stack.pop().unwrap();
                let while_ = self.back_stack.pop().unwrap();
                let loop_ = self.back_stack.pop().unwrap();

                let i = pool.unwrap_lab().unwrap_pool();
                let E = E.unwrap_p();
                let C = C.unwrap_p();

                self.back_stack.push(C::P(P::While(Box::new(E.clone()), Box::new(C.clone()), i)));
            },
            Rules::LoopT => {
                // (whilei · loopi · c, true · E · C · r, m, E' · loopi' · b) −→
                // (C ·(whilei E do C) · c, E · C · r, m, true · whilei' · loopi' · b)
                let while_ = self.control_stack.pop().unwrap();
                let loop_ = self.control_stack.pop().unwrap();

                let true_ = self.result_stack.pop().unwrap();
                let E = self.result_stack.pop().unwrap();
                let C = self.result_stack.pop().unwrap();

                let E_ = self.back_stack.pop().unwrap();
                let pool = self.back_stack.pop().unwrap();

                let i = loop_.unwrap_lab().unwrap_loop();
                let E = E.unwrap_p();
                let C = C.unwrap_p();

                self.control_stack.push(C::P(P::While(Box::new(E.clone()), Box::new(C.clone()), i)));
                self.control_stack.push(C::P(C.clone()));

                self.result_stack.push(R::P(C.clone()));
                self.result_stack.push(R::P(E.clone()));

                self.back_stack.push(C::Lab(Lab::Pool(i)));
                self.back_stack.push(C::Lab(Lab::Elihw(i)));
                let true_ = true_.unwrap_value();
                self.back_stack.push(C::P(P::Num(true_.clone())));

                // update the variable associated with the loop
                if self.check_while(i, E) {
                    self.increment_while_counter(i);
                } else {
                    self.decrement_while_counter(i);
                }
            },
            Rules::PoolT => {
                // (whilei' · loopi' · b, true · E · C · r, m, true' · C ·(whilei E do C) · c) −→
                // (E · loopi' · b, true · E · C · r, m, whilei · loopi · c)
                let while_lab = self.control_stack.pop().unwrap();
                let loop_ = self.control_stack.pop().unwrap();

                let true_ = self.result_stack.pop().unwrap();
                let E = self.result_stack.pop().unwrap();
                let C = self.result_stack.pop().unwrap();

                let eurt = self.back_stack.pop().unwrap();
                let C_ = self.back_stack.pop().unwrap();
                let while_ = self.back_stack.pop().unwrap();

                let i = while_lab.unwrap_lab().unwrap_elihw();
                let E = E.unwrap_p();
                let C = C.unwrap_p();

                self.control_stack.push(C::Lab(Lab::Pool(i)));
                self.control_stack.push(C::P(P::Rexp(Box::new(E.clone()))));

                self.result_stack.push(R::P(C.clone()));
                self.result_stack.push(R::P(E.clone()));

                let true_ = true_.unwrap_value();
                self.result_stack.push(R::Value(true_.clone()));

                self.back_stack.push(C::Lab(Lab::Loop(i)));
                self.back_stack.push(C::Lab(Lab::While(i)));
            },
            Rules::LoopF => {
                // (whilei · loopi · c, false · E · C · r, m, E' · loopi' · b) −→
                // (loopi · c, E · C · r, m, false · whilei' · loopi' · b)
                let while_ = self.control_stack.pop().unwrap();
                let loop_ = self.control_stack.pop().unwrap();

                let false_ = self.result_stack.pop().unwrap();
                let E = self.result_stack.pop().unwrap();
                let C = self.result_stack.pop().unwrap();

                let E_ = self.back_stack.pop().unwrap();
                let loop_ = self.back_stack.pop().unwrap();

                let i = loop_.unwrap_lab().unwrap_pool();

                self.control_stack.push(C::Lab(Lab::Loop(i)));

                let E = E.unwrap_p();
                let C = C.unwrap_p();
                self.result_stack.push(R::P(C.clone()));
                self.result_stack.push(R::P(E.clone()));

                self.back_stack.push(C::Lab(Lab::Pool(i)));
                self.back_stack.push(C::Lab(Lab::Elihw(i)));
                let false_ = false_.unwrap_value();
                self.back_stack.push(C::P(P::Num(false_.clone())));
            },
            Rules::PoolF => {
                // (whilei' · loopi' · b, false · E · C · r, m, false' · loopi · c) −→
                // (E' · loopi' · b, false · E · C · r, m, whilei · loopi · c)
                let while_lab = self.control_stack.pop().unwrap();
                let loop_ = self.control_stack.pop().unwrap();

                let false_ = self.result_stack.pop().unwrap();
                let E = self.result_stack.pop().unwrap();
                let C = self.result_stack.pop().unwrap();

                let false__ = self.back_stack.pop().unwrap();
                let loop_ = self.back_stack.pop().unwrap();

                let i = while_lab.unwrap_lab().unwrap_elihw();

                let E = E.unwrap_p();
                self.control_stack.push(C::Lab(Lab::Pool(i)));
                self.control_stack.push(C::P(P::Rexp(Box::new(E.clone()))));

                let C = C.unwrap_p();
                let false_ = false_.unwrap_value();
                self.result_stack.push(R::P(C.clone()));
                self.result_stack.push(R::P(E.clone()));
                self.result_stack.push(R::Value(false_.clone()));

                self.back_stack.push(C::Lab(Lab::Loop(i)));
                self.back_stack.push(C::Lab(Lab::While(i)));
            },
            Rules::EndWF => {
                // (loopi · c, E · C · r, m, false · whilei' · loopi' · b) −→
                // (loopi · c, 0 · C1 · E · C · r, m, endwi' · b)
                // where C1 = rev(whilei E do C)

                let E = self.result_stack.pop().unwrap();
                let C = self.result_stack.pop().unwrap();

                let false_ = self.back_stack.pop().unwrap();
                let Elihw = self.back_stack.pop().unwrap();
                let pool = self.back_stack.pop().unwrap();

                let i = pool.unwrap_lab().unwrap_pool();

                let C = C.unwrap_p();
                let E = E.unwrap_p();

                let while_ = P::While(Box::new(E.clone()), Box::new(C.clone()), i);
                let rev = self.rev(&while_);

                self.result_stack.push(R::P(C.clone()));
                self.result_stack.push(R::P(E.clone()));
                self.result_stack.push(R::P(rev));
                self.result_stack.push(R::Value(Num::Int(0)));

                self.back_stack.push(C::Lab(Lab::EndW(i)));
            },
            Rules::WEndF => {
                // (endwi' · b, 0 · C1 · E · C · r, m, loopi · c) −→
                // (false · whilei' · loopi' · b, E · C · r, m, loopi · c)
                let endwi = self.control_stack.pop().unwrap();

                let z = self.result_stack.pop().unwrap();
                let C1 = self.result_stack.pop().unwrap();
                let E = self.result_stack.pop().unwrap();
                let C = self.result_stack.pop().unwrap();

                let loopi = self.back_stack.pop().unwrap();

                let i = endwi.unwrap_lab().unwrap_endw();

                self.control_stack.push(C::Lab(Lab::Pool(i)));
                self.control_stack.push(C::Lab(Lab::Elihw(i)));
                self.control_stack.push(C::P(P::Num(Num::Int(0))));

                let C = C.unwrap_p();
                let E = E.unwrap_p();

                self.result_stack.push(R::P(C.clone()));
                self.result_stack.push(R::P(E.clone()));

                self.back_stack.push(C::Lab(Lab::Loop(i)));
            },
            Rules::EndWT => {
                // (loopi · c, n · C1 · E · C · r, m, endwi' · rev(C) · true · whilei' · loopi' · b) −→
                // (loopi · c, n + 1 · C1 · E · C · r, m, endwi' · b)
                let n = self.result_stack.pop().unwrap();
                
                let top = self.back_stack.peek_n(5);

                let endwi = self.back_stack.pop().unwrap();
                let rev_C = self.back_stack.pop().unwrap();
                let true_ = self.back_stack.pop().unwrap();
                let Elihw = self.back_stack.pop().unwrap();
                let pool = self.back_stack.pop().unwrap();

                let n = n.unwrap_value();
                let n = match n {
                    Num::Int(i) => i,
                    _ => panic!("Expected integer")
                };

                let n = Num::Int(n + 1);

                self.result_stack.push(R::Value(n));

                self.back_stack.push(endwi);
            },
            Rules::WEndT => {
                // (endwi' · b, n + 1 · C1 · E · C · r, m, loopi · c) −→
                // (endwi' · rev(C) · true · whilei' · loopi' · b, n · C1 · E · C · r, m, loopi · c)
                let endwi = self.control_stack.pop().unwrap();

                let n = self.result_stack.pop().unwrap();
                let C1 = self.result_stack.pop().unwrap();
                let E = self.result_stack.pop().unwrap();
                let C = self.result_stack.pop().unwrap();

                let loopi = self.back_stack.pop().unwrap();

                let i = endwi.unwrap_lab().unwrap_endw();

                self.control_stack.push(C::Lab(Lab::Pool(i)));
                self.control_stack.push(C::Lab(Lab::Elihw(i)));
                self.control_stack.push(C::P(P::Num(Num::Int(1))));
                let rev_C = self.rev(&C.unwrap_p());
                self.control_stack.push(C::P(rev_C));
                self.control_stack.push(C::Lab(Lab::EndW(i)));

                let n = n.unwrap_value();
                let n = match n {
                    Num::Int(i) => i,
                    _ => panic!("Expected integer")
                };

                let n = Num::Int(n - 1);

                let C = C.unwrap_p();
                let E = E.unwrap_p();
                let C1 = C1.unwrap_p();
                self.result_stack.push(R::P(C.clone()));
                self.result_stack.push(R::P(E.clone()));
                self.result_stack.push(R::P(C1.clone()));
                self.result_stack.push(R::Value(n));

                self.back_stack.push(C::Lab(Lab::Loop(i)));
            },
            Rules::EndW => {
                // (loopi · c, n · C1 · E · C · r, m, b) −→ (c, r, m, C1 · b)
                let loopi = self.control_stack.pop().unwrap();

                let n = self.result_stack.pop().unwrap();
                let C1 = self.result_stack.pop().unwrap();
                let E = self.result_stack.pop().unwrap();
                let C = self.result_stack.pop().unwrap();

                let endw = self.back_stack.pop().unwrap();

                let C1 = C1.unwrap_p();

                self.back_stack.push(C::P(C1.clone()));

                // Here there is a chance that there is junk left on the result stack
                // I dont know why this is, but we need to clear it.
                // Itll be of the format E · C
                while match self.result_stack.peek_n(2).as_slice() {
                    [E_, C_] => {
                        &E == E_ && &C == C_
                    },
                    _ => false
                } {
                    self.result_stack.pop();
                    self.result_stack.pop();
                }
            },
            Rules::WRexp => {
                // (E' · loopi' · b, v · E · C · r, m,     whilei · loopi · c) −→
                // (     loopi · b,      E · C · r, m, E · whilei · loopi · c)
                let E = self.control_stack.pop().unwrap();

                let v = self.result_stack.pop().unwrap();

                let E = E.unwrap_p().unwrap_rexp();

                self.back_stack.push(C::P(E.clone()));
            },
        }
    }

    pub fn reverse(&mut self) {
        std::mem::swap(&mut self.control_stack, &mut self.back_stack);
    }

    pub fn is_done(&self) -> bool {
        self.control_stack.is_empty()
    }

    fn check_rule(&self) -> Rules {
        // we only ever need to look at the top 3 elements of the control stack
        let top = self.control_stack.peek_n(3);

        let top = top.as_slice();

        // if theres not three elements, pad with skips
        let top = match top {
            [a, b, c] => vec![a, b, c],
            [a, b] => vec![a, b, &C::P(P::Skip)],
            [a] => vec![a, &C::P(P::Skip), &C::P(P::Skip)],
            _ => vec![&C::P(P::Skip), &C::P(P::Skip), &C::P(P::Skip)],
        };

        // rules for expressions and commands, we only need to look at the control stack for these.
        // as there is no ambiguity
        let rule = match top.as_slice() {
            // expression rules
            [C::P(p), _, _] if match p { P::Num(_) => true, _ => false } => Some(Rules::Num),
            [C::P(p), _, _] if match p { P::Mun(_) => true, _ => false } => Some(Rules::Mun),
            [C::P(p), _, _] if match p { P::Var(_) => true, _ => false } => Some(Rules::Var),
            [C::P(p), _, _] if match p { P::Rav(_) => true, _ => false } => Some(Rules::Rav),
            [C::P(p), _, _] if match p { P::BinOp(_, _, _) => true, _ => false } => Some(Rules::Exp),
            [C::Lab(l), E1, E2] if match l {
                Lab::Exp => E1.is_reverse_expression() && E2.is_reverse_expression(),
                _ => false
            } => Some(Rules::Pxe),
            [C::Lab(l), _, _] if match l { Lab::BinOp(_) => true, _ => false } => Some(Rules::BinOp),
            [C::Lab(l), _, _] if match l { Lab::BinPo(_) => true, _ => false } => Some(Rules::BinPo),
            [C::P(p), _, _] if match p { P::UnOp(_, _) => true, _ => false } => Some(Rules::UnExp),
            [C::Lab(l), E, _] if match l { Lab::UnExp => E.is_reverse_expression(), _ => false } => Some(Rules::UnPxe),
            [C::Lab(l), _, _] if match l { Lab::UnOp(_) => true, _ => false } => Some(Rules::UnOp),
            [C::Lab(l), _, _] if match l { Lab::UnPo(_) => true, _ => false } => Some(Rules::UnPo),
            // command rules
            [C::P(p), _, _] if match p { P::Skip => true, _ => false } => Some(Rules::Skip),
            [C::P(p), _, _] if match p { P::Asgn(_, _) => true, _ => false } => Some(Rules::Asgn),
            [C::Lab(l), _, _] if match l { Lab::Asgn => true, _ => false } => Some(Rules::Ngsa),
            [C::Lab(l), _, _] if match l { Lab::Assign => true, _ => false } => Some(Rules::Assign),
            [C::P(p), _, _] if match p { P::Ngsa(_, _) => true, _ => false } => Some(Rules::AsgnR),
            [C::Lab(l), _, _] if match l { Lab::Ngsa => true, _ => false } => Some(Rules::NgsaR),
            [C::Lab(l), _, _] if match l { Lab::Ngissa => true, _ => false } => Some(Rules::Ngissa),
            [C::P(p), _, _] if match p { P::Seq(_, _) => true, _ => false } => Some(Rules::Seq),
            [C::Lab(l), _, _] if match l { Lab::Seq => true, _ => false } => Some(Rules::Qes),
            [C::Lab(l), _, _] if match l { Lab::Sequence => true, _ => false } => Some(Rules::Sequence),
            _ => None
        };

        if let Some(rule) = rule {
            return rule;
        }

        // rules for conditionals and loops may be ambiguous and may require looking at the result stack
        let result = self.result_stack.peek();

        let rule = match (top.as_slice(), result) {
            // conditional rules
            ([C::P(p), _, _], _) if match p { P::If(_, _, _) => true, _ => false }  => Some(Rules::Cond),
            ([C::Lab(l), _, _], _) if match l { Lab::Dnoc => true, _ => false } => Some(Rules::Dnoc),
            ([C::Lab(l), C::Lab(c), _], Some(r)) if match (l,c) {
                (Lab::If, Lab::Cond) => r.is_truthy(),
                _ => false
            } => Some(Rules::IfT),
            ([C::Lab(l), C::Lab(c), _], Some(r)) if match (l, c) {
                (Lab::If, Lab::Cond) => r.is_falsy(),
                _ => false
            } => Some(Rules::IfF),
            ([C::Lab(l), C::Lab(c), _], Some(r)) if match (l, c) {
                (Lab::Fi, Lab::Dnoc) => r.is_truthy(),
                _ => false
            } => Some(Rules::FiT),
            ([C::Lab(l), C::Lab(c), _], Some(r)) if match (l, c) {
                (Lab::Fi, Lab::Dnoc) => r.is_falsy(),
                _ => false
            } => Some(Rules::FiF),
            ([C::Lab(l), _, _], _) if match l {
                Lab::Cond => true,
                _ => false
            } => Some(Rules::EndIf),
            ([C::P(p), C::Lab(l), _], _) if match (p, l) {
                (P::Rexp(_), Lab::Dnoc) => true,
                _ => false } => Some(Rules::IfRexp),
            // loop rules
            ([C::P(p), _, _], _) if match p { P::While(_, _, _) => true, _ => false } => Some(Rules::Loop),
            ([C::Lab(l_w), C::Lab(l_l), _], Some(r)) if match (l_w, l_l) {
                (Lab::While(_), Lab::Loop(_)) => r.is_truthy(),
                _ => false
            } => Some(Rules::LoopT),
            ([C::Lab(l_w), C::Lab(l_l), _], Some(r)) if match (l_w, l_l) {
                (Lab::Elihw(_), Lab::Pool(_)) => r.is_truthy(),
                _ => false
            } => Some(Rules::PoolT),
            ([C::Lab(l_w), C::Lab(l_l), _], Some(r)) if match (l_w, l_l) {
                (Lab::While(_), Lab::Loop(_)) => r.is_falsy(),
                _ => false
            } => Some(Rules::LoopF),
            ([C::Lab(l_w), C::Lab(l_l), _], Some(r)) if match (l_w, l_l) {
                (Lab::Elihw(_), Lab::Pool(_)) => r.is_falsy(),
                _ => false
            } => Some(Rules::PoolF),
            ([C::Lab(l), _, _], Some(R::Value(Num::Int(0)))) if match l {
                Lab::EndW(_) => true,
                _ => false
            } => Some(Rules::WEndF),
            ([C::Lab(l), _, _], Some(n)) if match l {
                Lab::EndW(_) => {
                    if let R::Value(Num::Int(n)) = n {
                        true
                    } else {
                        false
                    }
                },
                _ => false
            } => Some(Rules::WEndT),
            ([C::Lab(l), _, _], _) if match l {
                Lab::Pool(_) => true,
                _ => false
            } => Some(Rules::Pool),
            ([C::P(p), C::Lab(Lab::Pool(_)), _], _) if match p { P::Rexp(_) => true, _ => false } => Some(Rules::WRexp),
            ([C::Lab(l), _, _], r) => {
                if let Some(R::Value(_)) = r {
                    let back = self.result_stack.peek_n(4);
                    if let [_, R::P(P::While(E_, C_, _)), R::P(E), R::P(C)] = back.as_slice() {
                        // rev(while E_ do C_) should = while E do C
                        let top = self.back_stack.peek_n(5);
                        if let [C::Lab(Lab::EndW(_)), C::P(rev_C), C::P(P::Num(t)), C::Lab(Lab::Elihw(_)), C::Lab(Lab::Pool(_))] = top.as_slice() {
                            // t should be truthy,
                            // rev_c should be the reverse of C
                            Some(Rules::EndWT)
                        } else {
                            Some(Rules::EndW)
                        }
                    } else {
                        Some(Rules::EndW)
                    }

                } else {
                    return Rules::EndWF;
                }
            }
            _ => None
        };

        if let Some(rule) = rule {
            return rule;
        }

        panic!("No rule found for control stack: {:?}", self.control_stack);
    }

    fn rev(&self, p: &P) -> P {
        match p {
            P::Asgn(l, E) => P::Ngsa(l.clone(), E.clone()),
            P::Ngsa(l, E) => P::Asgn(l.clone(), E.clone()),
            P::Skip => P::Skip,
            P::Seq(C1, C2) => P::Seq(Box::new(self.rev(C2)), Box::new(self.rev(C1))),
            P::If(E, C1, C2) => P::If(E.clone(), Box::new(self.rev(C1)), Box::new(self.rev(C2))),
            P::While(E, C, i) => {
                if self.check_while(*i, E.as_ref()) {
                    P::While(
                        Box::new(Self::create_while_counter(*i)),
                        Box::new(self.rev(C)),
                        *i
                    )
                } else {
                    let E = self.while_condition.get(i).unwrap();
                    P::While(Box::new(E.clone()), Box::new(self.rev(C)), *i)
                }
            },
            E => E.clone()
        }
    }

    fn while_map(stack: ControlStack) -> HashMap<usize, P> {
        let mut map = HashMap::new();
        let mut stack = stack.clone();
        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            match top {
                C::P(p) => match p {
                    P::While(E, C, i) => {
                        let inner_E = E.as_ref();
                        map.insert(i, inner_E.clone());

                        let mut s = ControlStack::new();
                        s.push(C::P(*C));
                        let inner_map = Self::while_map(s);
                        map.extend(inner_map);
                    },
                    P::If(_, C1, C2) => {
                        let mut s = ControlStack::new();
                        s.push(C::P(*C1));
                        let inner_map = Self::while_map(s);
                        map.extend(inner_map);

                        let mut s = ControlStack::new();
                        s.push(C::P(*C2));
                        let inner_map = Self::while_map(s);
                        map.extend(inner_map);
                    },
                    P::Seq(C1, C2) => {
                        let mut s = ControlStack::new();
                        s.push(C::P(*C1));
                        let inner_map = Self::while_map(s);
                        map.extend(inner_map);

                        let mut s = ControlStack::new();
                        s.push(C::P(*C2));
                        let inner_map = Self::while_map(s);
                        map.extend(inner_map);
                    },
                    _ => {}
                }
                _ => {}
            }
        }
        map
    }

    fn check_while(&self, i: usize, E: &P) -> bool {
        let e = self.while_condition.get(&i);
        if let Some(E_) = e {
            return E == E_;
        } else {
            return false;
        }
    }

    fn create_while_counter(i: usize) -> P {
        let name = format!("while_counter_{}", i);
        P::BinOp(Box::new(P::Var(Var::Int(name.clone()))), Box::new(P::Num(Num::Int(0))), BinOp::Gt)
    }

    fn increment_while_counter(&mut self, i: usize) {
        let name = format!("while_counter_{}", i);
        let current = self.store.get(&name);
        if let Some(MemoryStoreElement::Integer(n)) = current {
            self.store.assign(&name, Value::Integer(n.get() + 1));
        } else {
            self.store.assign(&name, Value::Integer(1));
        }
    }

    fn decrement_while_counter(&mut self, i: usize) {
        let name = format!("while_counter_{}", i);
        let current = self.store.get(&name);
        if let Some(MemoryStoreElement::Integer(n)) = current {
            self.store.un_assign(&name, Value::Integer(n.get() - 1));
        } else {
            self.store.un_assign(&name, Value::Integer(0));
        }
    }
}