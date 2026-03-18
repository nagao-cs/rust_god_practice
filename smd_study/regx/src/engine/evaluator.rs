use super::Instruction;
use crate::helper::safe_add;
use std::{
    collections::VecDeque,
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum EvalError {
    PCOverFlow,
    SPOverFlow,
    InvalidPC,
    InvalidContext,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}

impl Error for EvalError {}

/// 深さ優先探索で再帰的にマッチングを行う関数
fn eval_depth(
    inst: &[Instruction],
    line: &[char],
    mut pc: usize,
    mut sp: usize,
) -> Result<bool, EvalError> {
    loop {
        let next = if let Some(i) = inst.get(pc) {
            i
        } else {
            return Err(EvalError::InvalidPC);
        };

        match next {
            Instruction::Char(c) => {
                if let Some(sp_c) = line.get(sp) {
                    if c == sp_c {
                        safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
                    } else {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
            Instruction::Match => {
                return Ok(true);
            }
            Instruction::Jump(addr) => {
                pc = *addr;
            }
            Instruction::Split(addr1, addr2) => {
                if eval_depth(inst, line, *addr1, sp)? ||
                    eval_depth(inst, line, *addr2, sp)? {
                        return Ok(true);
                } else {
                    return Ok(false);
                }
            }
        }
    }
}


// 幅優先探索の実装を追加
fn eval_width(inst: &[Instruction], line: &[char]) -> Result<bool, EvalError> {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0)); // (pc, sp)

    while let Some((mut pc, mut sp)) = queue.pop_front() {
        loop {
            let next = inst.get(pc).ok_or(EvalError::InvalidPC)?;
            match next {
                Instruction::Char(c) => {
                    if let Some(&sp_c) = line.get(sp) {
                        if *c == sp_c {
                            safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                            safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
                        } else {
                            break; // 失敗した場合は次のキューへ
                        }
                    } else {
                        break;
                    }
                }
                Instruction::Match => {
                    return Ok(true);
                }
                Instruction::Jump(addr) => {
                    pc = *addr;
                }
                Instruction::Split(addr1, addr2) => {
                    // 分岐先をキューに追加して幅優先で評価
                    queue.push_back((*addr1, sp));
                    queue.push_back((*addr2, sp));
                    break; 
                }
            }
        }
    }
    Ok(false)
}

/// 命令列の評価を行う関数
/// instが命令列となり その命令列を用いて入力文字列lineにマッチさせる
/// is_depthがtrueの場合に深さ優先探索を, falseの場合に幅優先探索を行う
/// 
/// 実行時にエラーが起きた場合はErrを返す
/// マッチ成功時はOk(true)を, 失敗時にはOk(false)を返す
/// 
pub fn eval(inst: &[Instruction], line: &[char], is_depth: bool) -> Result<bool, EvalError> {
    if is_depth {
        eval_depth(inst, line, 0, 0)
    } else {
        eval_width(inst, line)
    }
}