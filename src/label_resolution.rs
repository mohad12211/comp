use std::collections::HashMap;

use crate::{
    ast::{Block, BlockItem, Function, Program, Stmt},
    Result,
};

pub struct LabelResolver {
    pub counter: usize,
    _label_map: HashMap<String, String>,
}

impl LabelResolver {
    pub fn new(counter: usize) -> Self {
        Self {
            counter,
            _label_map: HashMap::new(),
        }
    }
    pub fn resolve_program(&mut self, program: &mut Program) -> Result<()> {
        match program {
            Program::Function(function) => self.resolve_fun(function)?,
        }

        Ok(())
    }

    fn resolve_fun(&mut self, function: &mut Function) -> Result<()> {
        self.resolve_block(&mut function.body)?;
        Ok(())
    }

    fn resolve_block(&mut self, block: &mut Block) -> Result<()> {
        block
            .items
            .iter_mut()
            .map(|block_item| self.resolve_block_item(block_item))
            .collect::<Result<Vec<_>>>()?;
        Ok(())
    }

    fn resolve_block_item(&mut self, block_item: &mut BlockItem) -> Result<()> {
        match block_item {
            BlockItem::Statement(stmt) => self.resolve_statement(stmt)?,
            BlockItem::Decleration(_) => {}
        }
        Ok(())
    }

    fn resolve_statement(&mut self, stmt: &mut Stmt) -> Result<()> {
        match stmt {
            Stmt::Return(_) | Stmt::Expression(_) | Stmt::Null => {}
            Stmt::If {
                condition: _,
                then_branch,
                else_branch,
            } => {
                self.resolve_statement(then_branch)?;
                if let Some(else_branch) = else_branch {
                    self.resolve_statement(else_branch)?;
                }
            }
            Stmt::Compound(block) => self.resolve_block(block)?,
            // TODO: actually implement this
            Stmt::Goto(_) => {}
            Stmt::Label(_, _) => {}
            Stmt::Break { label } => todo!(),
            Stmt::Continue { label } => todo!(),
            Stmt::While {
                condition,
                body,
                label,
            } => todo!(),
            Stmt::DoWhile {
                body,
                condition,
                label,
            } => todo!(),
            Stmt::For {
                init,
                condition,
                post,
                body,
                label,
            } => todo!(),
        };
        Ok(())
    }

    fn _make_temp(&mut self, name: &str) -> String {
        let unique_name = format!("{name}.{counter}", counter = self.counter);
        self.counter += 1;
        unique_name
    }
}
