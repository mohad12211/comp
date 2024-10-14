use std::collections::HashMap;

use crate::{
    ast::{BlockItem, Decleration, Expr, Function, Program, Stmt},
    Error, Result,
};

pub struct Resolver {
    pub counter: usize,
    variable_map: HashMap<String, String>,
}

impl Resolver {
    pub fn new(counter: usize) -> Self {
        Self {
            counter,
            variable_map: HashMap::new(),
        }
    }
    pub fn resolve_program(&mut self, program: &mut Program) -> Result<()> {
        match program {
            Program::Function(function) => self.resolve_fun(function)?,
        }

        Ok(())
    }

    fn resolve_fun(&mut self, function: &mut Function) -> Result<()> {
        function
            .body
            .iter_mut()
            .map(|block_item| self.resolve_block_item(block_item))
            .collect::<Result<Vec<_>>>()?;
        Ok(())
    }

    fn resolve_block_item(&mut self, block_item: &mut BlockItem) -> Result<()> {
        match block_item {
            BlockItem::Statement(stmt) => self.resolve_statement(stmt)?,
            BlockItem::Decleration(decleration) => {
                self.resolve_decleration(decleration)?;
            }
        }
        Ok(())
    }

    fn resolve_decleration(&mut self, decleration: &mut Decleration) -> Result<()> {
        match decleration {
            Decleration::Decleration { name, init } => {
                if self.variable_map.contains_key(name) {
                    return Err(Error::Resolver(
                        "Duplicate Variable Decleration".to_string(),
                    ));
                }
                let unique_name = self.make_temp(name);
                self.variable_map.insert(name.clone(), unique_name.clone());
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
            Expr::Constant(_) => {}
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
                *name = self
                    .variable_map
                    .get(name)
                    .ok_or_else(|| Error::Resolver("Undeclared variable".to_string()))?
                    .clone();
            }
            Expr::Assignment { left, right } => {
                if !matches!(left.as_ref(), Expr::Var(_)) {
                    return Err(Error::Resolver("Invalid lavlue".to_string()));
                }
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
        };
        Ok(())
    }

    fn resolve_statement(&mut self, stmt: &mut Stmt) -> Result<()> {
        match stmt {
            Stmt::Return(expr) => self.resolve_expr(expr)?,
            Stmt::Expression(expr) => self.resolve_expr(expr)?,
            Stmt::Null => {}
        };
        Ok(())
    }

    fn make_temp(&mut self, name: &str) -> String {
        let unique_name = format!("{name}.{counter}", counter = self.counter);
        self.counter += 1;
        unique_name
    }
}
