//! Takes in a parse tree and writes the corresponding VM code.
use crate::parser::parse_tree::ParseTree;
use crate::parser::parse_tree::class::ClassNode;
use crate::parser::parse_tree::expression::{
    ExpressionElement, ExpressionNode, SubroutineCallNode, TermElement, TermNode,
};
use crate::parser::parse_tree::statement::WhileStatementNode;
use crate::parser::parse_tree::subroutine::{SubroutineBodyNode, SubroutineNode, SubroutineType};
use crate::parser::parse_tree::var::{VarKind, VarType};
use crate::parser::parse_tree::statement::{
    DoStatementNode, IfStatementNode, LetStatementNode, ReturnStatementNode, Statement,
};

mod symbol_table;
use symbol_table::SymbolTable;

/// Takes in a parse tree and writes the corresponding VM code.
pub struct VMWriter {}

impl VMWriter {
    pub fn new() -> VMWriter {
        VMWriter {}
    }

    /// Takes in a [`ParseTree`](crate::parser::parse_tree::ParseTree) and returns
    /// a string containing the VM code.
    pub fn write(&self, parse_tree: &ParseTree) -> String {
        let mut vm_code = Vec::new();
        let mut symbol_table = SymbolTable::new();

        self.write_class(&parse_tree.class_node, &mut vm_code, &mut symbol_table);
        let mut vm_code = vm_code.join("\n");
        vm_code.push('\n');
        vm_code
    }

    fn write_class(
        &self,
        class_node: &ClassNode,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        let mut field_index = 0;
        let mut static_index = 0;
        for var in class_node.variables.iter() {
            for name in var.names.iter() {
                let index = match &var.var_kind {
                    VarKind::Static => {
                        static_index += 1;
                        static_index - 1
                    }
                    VarKind::Field => {
                        field_index += 1;
                        field_index - 1
                    }
                    _ => 0, // won't happen
                };
                symbol_table.define(name, &var.var_type, &var.var_kind, index);
            }
        }
        for subroutine in class_node.subroutines.iter() {
            self.write_subroutine(
                subroutine,
                &class_node.name,
                field_index,
                vm_code,
                symbol_table,
            );
        }
    }

    fn write_subroutine(
        &self,
        subroutine_node: &SubroutineNode,
        class_name: &String,
        num_fields: u32,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        let mut param_index = 0;
        if let SubroutineType::Method = subroutine_node.subroutine_type {
            symbol_table.define(
                &"this".to_string(),
                &VarType::get(class_name),
                &VarKind::Arg,
                param_index,
            );
            param_index += 1;
        }
        for param in subroutine_node.parameter_list.parameters.iter() {
            symbol_table.define(&param.name, &param.var_type, &VarKind::Arg, param_index);
            param_index += 1;
        }
        let mut if_index = 0;
        let mut while_index = 0;
        self.write_subroutine_body(
            &subroutine_node.body,
            class_name,
            &subroutine_node.name,
            &subroutine_node.subroutine_type,
            num_fields,
            &mut if_index,
            &mut while_index,
            vm_code,
            symbol_table,
        );
        symbol_table.clear_subroutine_scope();
    }

    fn write_subroutine_body(
        &self,
        subroutine_body: &SubroutineBodyNode,
        class_name: &String,
        subroutine_name: &String,
        subroutine_type: &SubroutineType,
        num_fields: u32,
        if_index: &mut u32,
        while_index: &mut u32,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {

        if subroutine_name == "nextMask" {
            println!("{:?}", subroutine_body);
        }

        let mut local_index = 0;
        for var in subroutine_body.variables.iter() {
            for name in var.var_names.iter() {
                symbol_table.define(name, &var.var_type, &var.var_kind, local_index);
                local_index += 1;
            }
        }
        vm_code.push(format!(
            "function {}.{} {}",
            class_name, subroutine_name, local_index
        ));
        match subroutine_type {
            SubroutineType::Constructor => {
                // if subroutine is a constructor, allocate memory for object
                self.push("constant", num_fields, vm_code);
                self.call("Memory.alloc", 1, vm_code);
                // set "this" segment to the base address of the new object
                self.pop("pointer", 0, vm_code);
            }
            SubroutineType::Method => {
                // before a method call, the this object is pushed onto the stack
                // set "this" segment to this address
                self.push("argument", 0, vm_code);
                self.pop("pointer", 0, vm_code);
            }
            _ => (),
        }
        for statement in subroutine_body.statements.iter() {
            self.write_statement(
                statement,
                class_name,
                if_index,
                while_index,
                vm_code,
                symbol_table,
            );
        }
    }

    //------------------------------
    // STATEMENTS
    //------------------------------

    fn write_statement(
        &self,
        statement: &Statement,
        class_name: &String,
        if_index: &mut u32,
        while_index: &mut u32,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        match statement {
            Statement::Let(let_statement) => {
                self.write_let_statement(let_statement, class_name, vm_code, symbol_table);
            }
            Statement::Do(do_statement) => {
                self.write_do_statement(do_statement, class_name, vm_code, symbol_table);
            }
            Statement::If(if_statement) => {
                self.write_if_statement(
                    if_statement,
                    class_name,
                    if_index,
                    while_index,
                    vm_code,
                    symbol_table,
                );
            }
            Statement::While(while_statement) => {
                self.write_while_statement(
                    while_statement,
                    class_name,
                    if_index,
                    while_index,
                    vm_code,
                    symbol_table,
                );
            }
            Statement::Return(return_statement) => {
                self.write_return_statement(return_statement, class_name, vm_code, symbol_table);
            }
        }
    }

    fn write_let_statement(
        &self,
        let_statement: &LetStatementNode,
        class_name: &String,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        if let Some(lhs_express) = &let_statement.lhs_expression {
            // array indexing
            self.write_expression(lhs_express, class_name, vm_code, symbol_table);
            // lhs expression evaluates to array index, which will be on top of the stack
            self.push(
                &symbol_table.get_var_kind(&let_statement.var_name),
                symbol_table.get_var_index(&let_statement.var_name),
                vm_code,
            );
            vm_code.push("add".to_string());
            // now evaluate rhs expression
            self.write_expression(
                &let_statement.rhs_expression,
                class_name,
                vm_code,
                symbol_table,
            );
            // load rhs expression value into "temp 0" segment
            self.pop("temp", 0, vm_code);
            // the top of the stack will now be the pointer to the lvalue array index
            // set the "that" segment to this address
            self.pop("pointer", 1, vm_code);
            // load value from "temp 0" into the lvalue array at the specified index
            self.push("temp", 0, vm_code);
            self.pop("that", 0, vm_code);
        } else {
            self.write_expression(
                &let_statement.rhs_expression,
                class_name,
                vm_code,
                symbol_table,
            );
            // I think we can only have normal variables here
            // the lvalue that the rhs expression will be assigned to
            self.pop(
                &symbol_table.get_var_kind(&let_statement.var_name),
                symbol_table.get_var_index(&let_statement.var_name),
                vm_code,
            );
        }
    }

    fn write_do_statement(
        &self,
        do_statement: &DoStatementNode,
        class_name: &String,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        self.write_subroutine_call(
            &do_statement.subroutine_call,
            class_name,
            vm_code,
            symbol_table,
        );
        // pop return value from stack and load it into "temp 0"
        self.pop("temp", 0, vm_code);
    }

    fn write_if_statement(
        &self,
        if_statement: &IfStatementNode,
        class_name: &String,
        if_index: &mut u32,
        while_index: &mut u32,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        // evaluate condition and put resulting value on the stack
        self.write_expression(&if_statement.condition, class_name, vm_code, symbol_table);
        let temp_idx = *if_index;
        vm_code.push(format!("if-goto IF_TRUE{}", temp_idx));
        vm_code.push(format!("goto IF_FALSE{}", temp_idx));
        vm_code.push(format!("label IF_TRUE{}", temp_idx));
        // code from if block here

        *if_index += 1;
        for statement in if_statement.if_block.iter() {
            self.write_statement(
                statement,
                class_name,
                if_index,
                while_index,
                vm_code,
                symbol_table,
            );
        }

        // when the else block exists
        if let Some(else_block) = &if_statement.else_block {
            vm_code.push(format!("goto IF_END{}", temp_idx));
            vm_code.push(format!("label IF_FALSE{}", temp_idx));
            // code from else block here
            for statement in else_block.iter() {
                self.write_statement(
                    statement,
                    class_name,
                    if_index,
                    while_index,
                    vm_code,
                    symbol_table,
                );
            }
            vm_code.push(format!("label IF_END{}", temp_idx));
        } else {
            vm_code.push(format!("label IF_FALSE{}", temp_idx));
        }
    }

    fn write_while_statement(
        &self,
        while_statement: &WhileStatementNode,
        class_name: &String,
        if_index: &mut u32,
        while_index: &mut u32,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        let temp_idx = *while_index;
        *while_index += 1;
        vm_code.push(format!("label WHILE_EXP{}", temp_idx));
        self.write_expression(
            &while_statement.condition,
            class_name,
            vm_code,
            symbol_table,
        );
        vm_code.push("not".to_string());
        vm_code.push(format!("if-goto WHILE_END{}", temp_idx));
        // loop body here
        for statement in while_statement.block.iter() {
            self.write_statement(
                statement,
                class_name,
                if_index,
                while_index,
                vm_code,
                symbol_table,
            );
        }
        vm_code.push(format!("goto WHILE_EXP{}", temp_idx));
        vm_code.push(format!("label WHILE_END{}", temp_idx));
    }

    fn write_return_statement(
        &self,
        return_statement: &ReturnStatementNode,
        class_name: &String,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        if let Some(expression) = &return_statement.expression {
            self.write_expression(expression, class_name, vm_code, symbol_table);
        } else {
            // no return value
            self.push("constant", 0, vm_code);
        }
        vm_code.push("return".to_string());
    }

    //------------------------------
    // EXPRESSIONS
    //------------------------------

    fn write_expression(
        &self,
        expression_node: &ExpressionNode,
        class_name: &String,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        if expression_node.elements.len() >= 1 {
            if let ExpressionElement::Term(term_node) = &expression_node.elements[0] {
                // we assume that if the expression node contains only a single element,
                // this element is a term node (per language specification)
                self.write_term_node(term_node, class_name, vm_code, symbol_table);
            }
            for i in (1..expression_node.elements.len() - 1).step_by(2) {
                if let ExpressionElement::Term(term_node) = &expression_node.elements[i + 1] {
                    self.write_term_node(term_node, class_name, vm_code, symbol_table);
                }
                let binary_op = &expression_node.elements[i];
                self.write_binary_operator(binary_op, vm_code);
            }
        }
    }

    fn write_term_node(
        &self,
        term_node: &TermNode,
        class_name: &String,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        if term_node.elements.len() > 0 {
            // a[expression]
            if term_node.elements.len() >= 4 {
                if let TermElement::Identifier(_) = &term_node.elements[0] {
                    match &term_node.elements[1] {
                        TermElement::Symbol(symbol) if symbol == "[" => {
                            self.evaluate_array_index(term_node, class_name, vm_code, symbol_table);
                            return;
                        }
                        _ => (),
                    }
                }
            }

            // (-|~)(term_node)
            if term_node.elements.len() >= 2 {
                match &term_node.elements[0] {
                    TermElement::Symbol(symbol) if symbol == "-" || symbol == "~" => {
                        self.evaluate_unary_operator(term_node, class_name, vm_code, symbol_table);
                        return;
                    }
                    _ => (),
                }
            }

            for term_elem in term_node.elements.iter() {
                self.write_term_element(term_elem, class_name, vm_code, symbol_table);
            }
        }
    }

    fn write_term_element(
        &self,
        term_element: &TermElement,
        class_name: &String,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        match term_element {
            TermElement::Identifier(identifier) => {
                self.push(
                    &symbol_table.get_var_kind(identifier),
                    symbol_table.get_var_index(identifier),
                    vm_code,
                );
            }
            TermElement::Symbol(_) => {
                // we can ignore symbols at this stage because
                // the parenthesis are already implicitly there because of the parse tree structure
                // and square brackets (array indexing) are already handled somewhere else
            }
            TermElement::IntegerConstant(val) => {
                self.push("constant", *val, vm_code);
            }
            TermElement::KeywordConstant(keyword) => match &keyword[..] {
                "null" => self.push("constant", 0, vm_code),
                "false" => self.push("constant", 0, vm_code),
                "true" => {
                    self.push("constant", 0, vm_code);
                    vm_code.push("not".to_string());
                }
                "this" => {
                    // push the address of the this object onto the stack
                    self.push("pointer", 0, vm_code);
                }
                _ => (),
            },
            TermElement::StringConstant(s) => {
                self.write_string_constant(s, vm_code);
            }
            TermElement::Expression(expression) => {
                self.write_expression(expression, class_name, vm_code, symbol_table);
            }
            TermElement::Term(term_node) => {
                for term_elem in term_node.elements.iter() {
                    self.write_term_element(term_elem, class_name, vm_code, symbol_table);
                }
            }
            TermElement::SubroutineCall(subroutine_call) => {
                self.write_subroutine_call(subroutine_call, class_name, vm_code, symbol_table);
            }
        }
    }

    fn write_binary_operator(&self, operator: &ExpressionElement, vm_code: &mut Vec<String>) {
        if let ExpressionElement::Operator(operator) = operator {
            match &operator[..] {
                "+" => vm_code.push("add".to_string()),
                "-" => vm_code.push("sub".to_string()),
                "=" => vm_code.push("eq".to_string()),
                "<" => vm_code.push("lt".to_string()),
                ">" => vm_code.push("gt".to_string()),
                "&" => vm_code.push("and".to_string()),
                "|" => vm_code.push("or".to_string()),
                "*" => vm_code.push("call Math.multiply 2".to_string()),
                "/" => vm_code.push("call Math.divide 2".to_string()),
                _ => {
                    eprintln!("Error: Unknown binary operator {}:", &operator[..]);
                    std::process::exit(1);
                }
            }
        }
    }

    //------------------------------
    // HELPER FUNCTIONS
    //------------------------------
    fn push(&self, var_kind: &str, index: u32, vm_code: &mut Vec<String>) {
        vm_code.push(format!("push {} {}", var_kind, index));
    }

    fn pop(&self, var_kind: &str, index: u32, vm_code: &mut Vec<String>) {
        vm_code.push(format!("pop {} {}", var_kind, index));
    }

    fn call(&self, subroutine_name: &str, num_args: u32, vm_code: &mut Vec<String>) {
        vm_code.push(format!("call {} {}", subroutine_name, num_args));
    }

    fn write_string_constant(&self, s: &str, vm_code: &mut Vec<String>) {
        self.push("constant", s.len() as u32, vm_code);
        self.call("String.new", 1, vm_code);
        for c in s.chars() {
            if !c.is_ascii() {
                eprintln!(
                    "VMWrite error: String constants can only consist of ASCII characters: {}",
                    s
                );
                std::process::exit(1);
            }
            self.push("constant", c as u32, vm_code);
            self.call("String.appendChar", 2, vm_code);
        }
    }

    fn evaluate_array_index(
        &self,
        term_node: &TermNode,
        class_name: &String,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        if let TermElement::Identifier(var_name) = &term_node.elements[0] {
            if let TermElement::Expression(expression) = &term_node.elements[2] {
                // this expression evaluates to the index
                self.write_expression(expression, class_name, vm_code, symbol_table);
                // base address of array
                self.push(
                    &symbol_table.get_var_kind(var_name),
                    symbol_table.get_var_index(var_name),
                    vm_code,
                );
                // add index to base address
                vm_code.push("add".to_string());
                // set the "that" segment to this address
                self.pop("pointer", 1, vm_code);
                // push the contents from "that" to the stack
                self.push("that", 0, vm_code);
            }
        };
    }

    fn evaluate_unary_operator(
        &self,
        term_node: &TermNode,
        class_name: &String,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        match &term_node.elements[0] {
            // if a unary operator (- or ~) is followed by a term,
            // we have to evaluate the term first, and then apply the
            // function implied by the unary operator
            TermElement::Symbol(symbol) if symbol == "-" || symbol == "~" => {
                if term_node.elements.len() > 1 {
                    self.write_term_element(
                        &term_node.elements[1],
                        class_name,
                        vm_code,
                        symbol_table,
                    );
                }
                if symbol == "-" {
                    vm_code.push("neg".to_string());
                } else {
                    vm_code.push("not".to_string());
                }
            }
            _ => (),
        }
    }

    fn write_subroutine_call(
        &self,
        subroutine_call: &SubroutineCallNode,
        class_name: &String,
        vm_code: &mut Vec<String>,
        symbol_table: &mut SymbolTable,
    ) {
        let (caller, this_count) = if let Some(caller) = &subroutine_call.caller {
            if symbol_table.contains(caller) {
                // I have to clone here because if I borrow here, I cannot create
                // a mutable reference of symbol_table below when processing the arguments
                let var_type = symbol_table.get_var_type(caller).clone();
                if let VarType::ClassName(class_name) = var_type {
                    // in this case the subroutine is a method and we have to push
                    // the this object to the stack
                    self.push(
                        &symbol_table.get_var_kind(caller),
                        symbol_table.get_var_index(caller),
                        vm_code,
                    );
                    (class_name, 1)
                } else {
                    // dummy, cannot happen
                    eprintln!(
                        "VMWrite Error: Callee must have var type ClassName: {}",
                        var_type
                    );
                    std::process::exit(1);
                }
            } else {
                // p.279 of the book: any identifier not found in the symbol table may be assumed
                // to be a subroutine name or a class name
                (caller.to_string(), 0)
            }
        } else {
            // use this object as caller
            self.push("pointer", 0, vm_code);
            (class_name.to_string(), 1)
        };

        let num_args = subroutine_call.expression_list.len() + this_count;
        for argument in subroutine_call.expression_list.iter() {
            self.write_expression(argument, class_name, vm_code, symbol_table);
        }
        self.call(
            &format!("{}.{}", caller, &subroutine_call.subroutine_name),
            num_args as u32,
            vm_code,
        );
    }
}
