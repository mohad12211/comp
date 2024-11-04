use std::collections::HashMap;

use crate::{
    ast::{Block, BlockItem, Decleration, Expr, Function, Program, Stmt},
    Error, Result,
};

pub struct VarResolver {
    pub counter: usize,
    variable_map: HashMap<String, (String, bool)>,
}

impl VarResolver {
    pub fn new(counter: usize) -> Self {
        Self {
            counter,
            variable_map: HashMap::new(),
        }
    }

    pub fn resolve_program(&mut self, program: &mut Program) -> Result<()> {
        match program {
            Program::Function(function) => self.resolve_fun(function),
        }
    }

    fn resolve_fun(&mut self, function: &mut Function) -> Result<()> {
        self.resolve_block(&mut function.body)
    }

    fn resolve_block(&mut self, block: &mut Block) -> Result<()> {
        for item in &mut block.items {
            self.resolve_block_item(item)?;
        }
        Ok(())
    }

    fn resolve_block_item(&mut self, block_item: &mut BlockItem) -> Result<()> {
        match block_item {
            BlockItem::Statement(stmt) => self.resolve_statement(stmt),
            BlockItem::Decleration(decleration) => self.resolve_decleration(decleration),
        }
    }

    fn resolve_decleration(&mut self, decleration: &mut Decleration) -> Result<()> {
        match decleration {
            Decleration::Decleration { name, init } => {
                if let Some((_, true)) = self.variable_map.get(name) {
                    return Err(Error::Resolver(
                        "Duplicate Variable Decleration".to_string(),
                    ));
                }
                let unique_name = self.make_temp(name);
                self.variable_map
                    .insert(name.clone(), (unique_name.clone(), true));
                if let Some(init) = init {
                    self.resolve_expr(init)?;
                }
                *name = unique_name;
            }
        }
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &mut Expr) -> Result<()> {
        match expr {
            Expr::Unary { operator: _, right } => self.resolve_expr(right)?,
            Expr::Binary {
                operator: _,
                left,
                right,
            } => {
                self.resolve_expr(right)?;
                self.resolve_expr(left)?;
            }
            Expr::Var(name) => {
                if let Some((unique_name, _)) = self.variable_map.get(name) {
                    *name = unique_name.clone();
                } else {
                    return Err(Error::Resolver("Undeclared variable".to_string()));
                }
            }
            Expr::Assignment {
                left,
                right,
                operator: _,
            } => {
                if !matches!(left.as_ref(), Expr::Var(_)) {
                    return Err(Error::Resolver("Invalid lavlue".to_string()));
                }
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Conditional {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition)?;
                self.resolve_expr(then_branch)?;
                self.resolve_expr(else_branch)?;
            }
            Expr::Constant(_) => {}
        };
        Ok(())
    }

    fn resolve_statement(&mut self, stmt: &mut Stmt) -> Result<()> {
        match stmt {
            Stmt::Return(expr) => self.resolve_expr(expr)?,
            Stmt::Expression(expr) => self.resolve_expr(expr)?,
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition)?;
                self.resolve_statement(then_branch)?;
                if let Some(else_branch) = else_branch {
                    self.resolve_statement(else_branch)?;
                }
            }
            Stmt::Label(_, stmt) => self.resolve_statement(stmt)?,
            Stmt::Compound(block) => {
                let new_map = self.create_new_scope();
                let old_map = std::mem::replace(&mut self.variable_map, new_map);
                self.resolve_block(block)?;
                self.variable_map = old_map;
            }
            Stmt::Goto(_) | Stmt::Null => {}
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

    fn create_new_scope(&self) -> HashMap<String, (String, bool)> {
        self.variable_map
            .iter()
            .map(|(name, (unique_name, _))| (name.clone(), (unique_name.clone(), false)))
            .collect()
    }

    fn make_temp(&mut self, name: &str) -> String {
        let unique_name = format!("{name}.{counter}", counter = self.counter);
        self.counter += 1;
        unique_name
    }
}
