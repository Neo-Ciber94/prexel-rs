use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

use crate::function::{BinaryFunction, Function, UnaryFunction};
use crate::num::checked::CheckedNum;
use crate::num::unchecked::UncheckedNum;
use crate::ops::math::RandFunction;
use crate::ops::math::*;
use crate::utils::ignore_case_string::IgnoreCaseString;

/// Trait to provides the variables, constants and functions used for evaluate an expression.
pub trait Context<'a, N> {
    /// Gets the configuration of the context.
    fn config(&self) -> &Config;

    /// Adds a function to the context.
    fn add_function<F: Function<N> + 'a>(&mut self, func: F);

    /// Adds an unary function to the context.
    fn add_unary_function<F: UnaryFunction<N> + 'a>(&mut self, func: F);

    /// Adds a binary function to the context.
    fn add_binary_function<F: BinaryFunction<N> + 'a>(&mut self, func: F);

    /// Adds a constant value to the context.
    fn add_constant(&mut self, name: &str, value: N);

    /// Adds or set the value of a variable in the context.
    fn set_variable(&mut self, name: &str, value: N) -> Option<N>;

    /// Gets the value of a variable in the context.
    fn get_variable(&self, name: &str) -> Option<&N>;

    /// Gets the value of a constant in the context.
    fn get_constant(&self, name: &str) -> Option<&N>;

    /// Gets a function with the given name.
    fn get_function(&self, name: &str) -> Option<&Rc<dyn Function<N> + 'a>>;

    /// Gets an unary function with the given name.
    fn get_unary_function(&self, name: &str) -> Option<&Rc<dyn UnaryFunction<N> + 'a>>;

    /// Gets a binary function with the given name.
    fn get_binary_function(&self, name: &str) -> Option<&Rc<dyn BinaryFunction<N> + 'a>>;

    /// Checks if exists a variable with the given name.
    #[inline]
    fn is_variable(&self, name: &str) -> bool {
        self.get_variable(name).is_some()
    }

    /// Checks if exists a constant with the given name.
    #[inline]
    fn is_constant(&self, name: &str) -> bool {
        self.get_constant(name).is_some()
    }

    /// Checks if exists a function with the given name.
    #[inline]
    fn is_function(&self, name: &str) -> bool {
        self.get_function(name).is_some()
    }

    /// Checks if exists a unary function with the given name.
    #[inline]
    fn is_unary_function(&self, name: &str) -> bool {
        self.get_unary_function(name).is_some()
    }

    /// Checks if exists a binary function with the given name.
    #[inline]
    fn is_binary_function(&self, name: &str) -> bool {
        self.get_binary_function(name).is_some()
    }
}

/// Provides a default implementation of a math `Context`.
#[derive(Clone)]
pub struct DefaultContext<'a, N> {
    /// The variables.
    variables: HashMap<IgnoreCaseString, N>,
    /// The constants.
    constants: HashMap<IgnoreCaseString, N>,
    /// The functions.
    functions: HashMap<IgnoreCaseString, Rc<dyn Function<N> + 'a>>,
    /// The unary functions.
    unary_functions: HashMap<IgnoreCaseString, Rc<dyn UnaryFunction<N> + 'a>>,
    /// The binary functions.
    binary_functions: HashMap<IgnoreCaseString, Rc<dyn BinaryFunction<N> + 'a>>,
    /// Additional information about this context
    config: Config,
}

impl<'a, N> DefaultContext<'a, N> {
    /// Constructs a new `Context` with no variables, constants or functions.
    #[inline]
    pub fn empty() -> Self {
        DefaultContext {
            variables: Default::default(),
            constants: Default::default(),
            functions: Default::default(),
            binary_functions: Default::default(),
            unary_functions: Default::default(),
            config: Config::new(),
        }
    }

    /// Constructs a new `Context` with no variables, constants or functions, using the
    /// specified `Config`.
    #[inline]
    pub fn empty_with_config(config: Config) -> Self {
        DefaultContext {
            variables: Default::default(),
            constants: Default::default(),
            functions: Default::default(),
            binary_functions: Default::default(),
            unary_functions: Default::default(),
            config,
        }
    }

    /// Gets a reference to the variable values of this context.
    #[inline]
    pub fn variables(&self) -> &HashMap<IgnoreCaseString, N> {
        &self.variables
    }

    /// Gets a reference to the constant values of this context.
    #[inline]
    pub fn constants(&self) -> &HashMap<IgnoreCaseString, N> {
        &self.constants
    }

    /// Gets a reference to the functions of this context.
    #[inline]
    pub fn functions(&self) -> &HashMap<IgnoreCaseString, Rc<dyn Function<N> + 'a>> {
        &self.functions
    }

    /// Gets a reference to the unary functions of this context.
    #[inline]
    pub fn unary_functions(&self) -> &HashMap<IgnoreCaseString, Rc<dyn UnaryFunction<N> + 'a>> {
        &self.unary_functions
    }

    /// Gets a reference to the binary functions of this context.
    #[inline]
    pub fn binary_functions(&self) -> &HashMap<IgnoreCaseString, Rc<dyn BinaryFunction<N> + 'a>> {
        &self.binary_functions
    }

    /// Adds the specified function to the context using the given name.
    ///
    /// # Remarks
    /// - This allows to use a function with an alias.
    ///
    /// # Examples
    /// ```
    /// use math_engine::context::{DefaultContext, Context};
    /// use math_engine::ops::math::MaxFunction;
    ///
    /// let mut context : DefaultContext<f64> = DefaultContext::empty();
    /// context.add_function(MaxFunction);
    /// context.add_function_as(MaxFunction, "Maximum");
    /// ```
    #[inline]
    pub fn add_function_as<F: Function<N> + 'a>(&mut self, func: F, name: &str) {
        let function_name = IgnoreCaseString::from(name);
        if self.functions.contains_key(&function_name) {
            panic!("A function named '{}' already exists", function_name);
        } else {
            self.functions.insert(function_name, Rc::new(func));
        }
    }

    /// Adds the specified unary function to the context using the given name.
    ///
    /// # Remarks
    /// - This allows to use an unary function with an alias.
    #[inline]
    pub fn add_unary_function_as<F: UnaryFunction<N> + 'a>(&mut self, func: F, name: &str) {
        let function_name = IgnoreCaseString::from(name);
        if self.unary_functions.contains_key(&function_name) {
            panic!("An unary function named '{}' already exists", function_name);
        } else {
            self.unary_functions.insert(function_name, Rc::new(func));
        }
    }

    /// Adds the specified binary function to the context using the given name.
    ///
    /// # Remarks
    /// - This allows to use a binary function with an alias.
    ///
    /// # Examples
    /// ```
    /// use math_engine::context::{DefaultContext, Context};
    /// use math_engine::ops::unchecked::AddOperator;
    ///
    /// let mut context : DefaultContext<f64> = DefaultContext::empty();
    /// context.add_binary_function(AddOperator);
    /// context.add_binary_function_as(AddOperator, "Plus");
    /// ```
    #[inline]
    pub fn add_binary_function_as<F: BinaryFunction<N> + 'a>(&mut self, func: F, name: &str) {
        let function_name = IgnoreCaseString::from(name);
        if self.binary_functions.contains_key(&function_name) {
            panic!("A binary function named '{}' already exists", function_name);
        } else {
            self.binary_functions.insert(function_name, Rc::new(func));
        }
    }
}

impl<'a, N> Context<'a, N> for DefaultContext<'a, N> {
    #[inline]
    fn config(&self) -> &Config {
        &self.config
    }

    #[inline]
    fn add_function<F: Function<N> + 'a>(&mut self, func: F) {
        validator::check_function_name(func.name(), validator::Kind::Function);
        let name = func.name().to_string();
        self.add_function_as(func, &name)
    }

    #[inline]
    fn add_unary_function<F: UnaryFunction<N> + 'a>(&mut self, func: F) {
        validator::check_function_name(func.name(), validator::Kind::Operator);
        let name = func.name().to_string();
        self.add_unary_function_as(func, &name)
    }

    #[inline]
    fn add_binary_function<F: BinaryFunction<N> + 'a>(&mut self, func: F) {
        validator::check_function_name(
            func.name(),
            if func.name().len() > 1 {
                validator::Kind::Function
            } else {
                validator::Kind::Operator
            },
        );
        let name = func.name().to_string();
        self.add_binary_function_as(func, &name)
    }

    #[inline]
    fn add_constant(&mut self, name: &str, value: N) {
        validator::check_variable_name(name);
        self.constants.insert(IgnoreCaseString::from(name), value);
    }

    #[inline]
    fn set_variable(&mut self, name: &str, value: N) -> Option<N> {
        validator::check_variable_name(name);
        let string = IgnoreCaseString::from(name);
        if self.constants.contains_key(&string) {
            panic!(
                "Invalid variable name, a constant named '{}' already exists",
                string
            )
        } else {
            self.variables.insert(string, value)
        }
    }

    #[inline]
    fn get_variable(&self, name: &str) -> Option<&N> {
        self.variables.get(IgnoreCaseString::from(name).borrow())
    }

    #[inline]
    fn get_constant(&self, name: &str) -> Option<&N> {
        self.constants.get(&IgnoreCaseString::from(name))
    }

    #[inline]
    fn get_function(&self, name: &str) -> Option<&Rc<dyn Function<N> + 'a>> {
        self.functions.get(&IgnoreCaseString::from(name))
    }

    #[inline]
    fn get_unary_function(&self, name: &str) -> Option<&Rc<dyn UnaryFunction<N> + 'a>> {
        self.unary_functions.get(&IgnoreCaseString::from(name))
    }

    #[inline]
    fn get_binary_function(&self, name: &str) -> Option<&Rc<dyn BinaryFunction<N> + 'a>> {
        self.binary_functions.get(&IgnoreCaseString::from(name))
    }
}

impl<'a, N: CheckedNum> DefaultContext<'a, N> {
    /// Gets an instance of a `DefaultContext`.
    ///
    /// # Safety
    /// Stores a cache of the `DefaultContext` used as raw pointers.
    pub unsafe fn instance() -> &'static DefaultContext<'a, N> {
        use crate::utils::lazy::Lazy;
        use crate::utils::untyped::Untyped;
        use std::any::TypeId;;

        static mut CACHE: Lazy<HashMap<TypeId, Untyped>> = Lazy::new(HashMap::new);
        let type_id = TypeId::of::<N>();

        match (*CACHE).get(&type_id) {
            Some(p) => p.cast::<DefaultContext<'a, N>>(),
            None => {
                let context = Box::leak(Box::new(DefaultContext::new_checked()));
                CACHE.insert(type_id, Untyped::new(context));
                context
            }
        }
    }

    /// Constructs a new `Context` with checked functions.
    ///
    /// # Remarks
    /// Some functions may cause overflow exceptions, the functions of this context
    /// ensures will return an error instead of throws an exception.
    #[inline]
    pub fn new_checked() -> Self {
        Self::new_checked_with_config(Config::new())
    }

    /// Constructs a new `Context` using the given `Config` with checked functions.
    ///
    /// # Remarks
    /// Some functions may cause overflow exceptions, the functions of this context
    /// ensures will return an error instead of throws an exception.
    pub fn new_checked_with_config(config: Config) -> Self {
        use crate::ops::checked::*;

        let mut context = Self::empty_with_config(config);
        context.add_constant("PI", N::from_f64(std::f64::consts::PI).unwrap());
        context.add_constant("E", N::from_f64(std::f64::consts::E).unwrap());
        context.add_binary_function(AddOperator);
        context.add_binary_function(SubOperator);
        context.add_binary_function(MulOperator);
        context.add_binary_function(DivOperator);
        context.add_binary_function(PowOperator);
        context.add_binary_function(ModOperator);
        context.add_unary_function(UnaryPlus);
        context.add_unary_function(UnaryMinus);
        context.add_unary_function(Factorial);
        context.add_function(SumFunction);
        context.add_function(ProdFunction);
        context.add_function(AvgFunction);
        context.add_function(MaxFunction);
        context.add_function(MinFunction);
        context.add_function(AbsFunction);
        context.add_function(SqrtFunction);
        context.add_function(LnFunction);
        context.add_function(LogFunction);
        context.add_function(ExpFunction);
        context.add_function(FloorFunction);
        context.add_function(CeilFunction);
        context.add_function(TruncateFunction);
        context.add_function(RoundFunction);
        context.add_function(SignFunction);
        context.add_function(RandFunction);
        context.add_function(SinFunction);
        context.add_function(CosFunction);
        context.add_function(TanFunction);
        context.add_function(CscFunction);
        context.add_function(SecFunction);
        context.add_function(CotFunction);
        context.add_function(ASinFunction);
        context.add_function(ACosFunction);
        context.add_function(ATanFunction);
        context.add_function(ACscFunction);
        context.add_function(ASecFunction);
        context.add_function(ACotFunction);
        context.add_function(SinhFunction);
        context.add_function(CoshFunction);
        context.add_function(TanhFunction);
        context.add_function(CschFunction);
        context.add_function(SechFunction);
        context.add_function(CothFunction);
        context.add_function(ASinhFunction);
        context.add_function(ACoshFunction);
        context.add_function(ATanhFunction);
        context.add_function(ACschFunction);
        context.add_function(ASechFunction);
        context.add_function(ACothFunction);
        context
    }
}

impl<'a, N: UncheckedNum> DefaultContext<'a, N> {
    /// Constructs a new `Context` with unchecked functions.
    ///
    /// # Remarks
    /// Functions of this context may panic when the value overflows.
    #[inline]
    pub fn new_unchecked() -> Self {
        Self::new_unchecked_with_config(Config::new())
    }

    /// Constructs a new `Context` using the given `Config` with unchecked functions.
    ///
    /// # Remarks
    /// Functions of this context may panic when the value overflows.
    pub fn new_unchecked_with_config(config: Config) -> Self {
        use crate::ops::unchecked::*;

        let mut context = Self::empty_with_config(config);
        context.add_constant("PI", N::from_f64(std::f64::consts::PI).unwrap());
        context.add_constant("E", N::from_f64(std::f64::consts::E).unwrap());
        context.add_binary_function(AddOperator);
        context.add_binary_function(SubOperator);
        context.add_binary_function(MulOperator);
        context.add_binary_function(DivOperator);
        context.add_binary_function(PowOperator);
        context.add_binary_function(ModOperator);
        context.add_unary_function(UnaryPlus);
        context.add_unary_function(UnaryMinus);
        context.add_unary_function(Factorial);
        context.add_function(SumFunction);
        context.add_function(AvgFunction);
        context.add_function(ProdFunction);
        context.add_function(MaxFunction);
        context.add_function(MinFunction);
        context.add_function(SqrtFunction);
        context.add_function(LnFunction);
        context.add_function(LogFunction);
        context.add_function(RandFunction);
        context.add_function(ExpFunction);
        context.add_function(SinFunction);
        context.add_function(CosFunction);
        context.add_function(TanFunction);
        context.add_function(CscFunction);
        context.add_function(SecFunction);
        context.add_function(CotFunction);
        context.add_function(ASinFunction);
        context.add_function(ACosFunction);
        context.add_function(ATanFunction);
        context.add_function(ACscFunction);
        context.add_function(ASecFunction);
        context.add_function(ACotFunction);
        context.add_function(SinhFunction);
        context.add_function(CoshFunction);
        context.add_function(TanhFunction);
        context.add_function(CschFunction);
        context.add_function(SechFunction);
        context.add_function(CothFunction);
        context.add_function(ASinhFunction);
        context.add_function(ACoshFunction);
        context.add_function(ATanhFunction);
        context.add_function(ACschFunction);
        context.add_function(ASechFunction);
        context.add_function(ACothFunction);
        context
    }
}

/// Represents the configuration used by a `Context`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Config {
    /// Allows implicit multiplication.
    implicit_mul: bool,
    /// Allows complex numbers.
    complex_number: bool,
    /// Allows using custom grouping symbols for function calls, eg: Max[1,2,3], Sum<2,4,6>
    custom_function_call: bool,
    /// Stores the grouping symbols as: `(`, `)`, `[`, `]`.
    grouping: HashMap<char, GroupingSymbol>,
}

impl Config {
    /// Constructs a new `Config` using the default grouping symbol: `(`, `)`,
    /// if is need an empty `Config` use `Default` instead.
    #[inline]
    pub fn new() -> Self {
        Config::default().with_group_symbol('(', ')')
    }

    /// Enables implicit multiplication for this `Config`.
    #[inline]
    pub fn with_implicit_mul(mut self) -> Config {
        self.implicit_mul = true;
        self
    }

    /// Enables complex number usage for this `Config`.
    ///
    /// # Remarks
    /// [`Tokenizer`] checks for this value when parsing expressions.
    ///
    /// [`Tokenizer`]: ../tokenizer/struct.Tokenizer.html
    #[inline]
    pub fn with_complex_number(mut self) -> Config {
        self.complex_number = true;
        self
    }

    /// Enables custom function calls groping symbols.
    ///
    /// # Remarks
    /// Function calls are only allowed within parentheses, eg: Product(3, 6, 6),
    /// but `with_custom_function_call` allow to use others, eg: Max[1,2,3], Sum<2,4,6>.
    #[inline]
    pub fn with_custom_function_call(mut self) -> Config {
        self.custom_function_call = true;
        self
    }

    /// Adds a pair of grouping symbols to this `Config`.
    ///
    /// # Panics
    /// If the config already contains the given symbol.
    ///
    /// # Example
    /// ```
    /// use math_engine::context::Config;
    ///
    /// // `Default` allows to create an empty config
    /// let mut config = Config::default()
    ///     .with_group_symbol('(', ')')
    ///     .with_group_symbol('[', ']');
    /// ```
    pub fn with_group_symbol(mut self, open_group: char, close_group: char) -> Config {
        let grouping = &mut self.grouping;
        let grouping_symbol = GroupingSymbol::new(open_group, close_group);

        if grouping.insert(open_group, grouping_symbol).is_some() {
            panic!("Duplicated symbol: `{}`", open_group);
        }

        if grouping.insert(close_group, grouping_symbol).is_some() {
            panic!("Duplicated symbol: `{}`", close_group);
        }

        self
    }

    /// Checks if the context allow implicit multiplication.
    #[inline]
    pub fn implicit_mul(&self) -> bool {
        self.implicit_mul
    }

    /// Checks if the context allows work with complex numbers.
    #[inline]
    pub fn complex_number(&self) -> bool {
        self.complex_number
    }

    /// Checks if the context allows custom function calls, eg: Max[1,2,3], Sum<2,4,6>
    #[inline]
    pub fn custom_function_call(&self) -> bool {
        self.custom_function_call
    }

    /// Gets a grouping symbol pair from this `Config`.
    ///
    /// # Examples
    /// ```
    /// use math_engine::context::Config;
    ///
    /// let mut config = Config::new().with_group_symbol('[', ']');
    /// assert_eq!(('(', ')'), config.get_group_symbol('(').unwrap().as_tuple());
    /// ```
    #[inline]
    pub fn get_group_symbol(&self, symbol: char) -> Option<&GroupingSymbol> {
        self.grouping.get(&symbol)
    }

    /// Gets the grouping close for the specified grouping open.
    ///
    /// # Example
    /// ```
    /// use math_engine::context::Config;
    ///
    /// let config = Config::default()
    ///     .with_group_symbol('(', ')')
    ///     .with_group_symbol('[', ']');
    ///
    /// assert_eq!(Some('('), config.get_group_open_for(')'));
    /// assert_eq!(Some('['), config.get_group_open_for(']'));
    /// assert_eq!(None, config.get_group_open_for('['));
    /// ```
    #[inline]
    pub fn get_group_open_for(&self, group_close: char) -> Option<char> {
        match self.get_group_symbol(group_close) {
            Some(s) if s.group_close == group_close => Some(s.group_open),
            _ => None,
        }
    }

    /// Gets the grouping close for the specified grouping open.
    ///
    /// # Example
    /// ```
    /// use math_engine::context::Config;
    ///
    /// let config = Config::default()
    ///     .with_group_symbol('(', ')')
    ///     .with_group_symbol('[', ']');
    ///
    /// assert_eq!(Some(')'), config.get_group_close_for('('));
    /// assert_eq!(Some(']'), config.get_group_close_for('['));
    /// assert_eq!(None, config.get_group_close_for(']'));
    /// ```
    #[inline]
    pub fn get_group_close_for(&self, group_open: char) -> Option<char> {
        match self.get_group_symbol(group_open) {
            Some(s) if s.group_open == group_open => Some(s.group_close),
            _ => None,
        }
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Config {
            implicit_mul: false,
            complex_number: false,
            custom_function_call: false,
            grouping: Default::default(),
        }
    }
}

/// Represents a grouping symbol.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GroupingSymbol {
    /// The open symbol of teh grouping.
    pub group_open: char,
    /// The close symbol of teh grouping.
    pub group_close: char,
}

impl GroupingSymbol {
    /// Constructs a new `GroupingSymbol`.
    #[inline]
    pub fn new(group_open: char, group_close: char) -> Self {
        assert_ne!(group_open, group_close);
        GroupingSymbol {
            group_open,
            group_close,
        }
    }

    /// Gets the open and close `char` symbols of this grouping.
    #[inline]
    pub fn as_tuple(&self) -> (char, char) {
        (self.group_open, self.group_close)
    }
}

#[cfg(debug_assertions)]
mod validator {
    pub enum Kind {
        /// Validates a function
        Function,
        /// Validates an operator
        Operator,
    }

    pub fn check_function_name(name: &str, kind: Kind) {
        debug_assert!(
            !name.is_empty(),
            "function and operators names cannot be empty"
        );

        match kind {
            Kind::Function => {
                debug_assert!(
                    name.len() > 1,
                    "function names must have more than 1 character: `{}`",
                    name
                );
                debug_assert!(
                    name.chars().all(char::is_alphanumeric),
                    "function names must only include letters and numbers: `{}`",
                    name
                );

                debug_assert!(
                    name.chars().next().unwrap().is_alphabetic(),
                    "function names must start with a letter: `{}`",
                    name
                );
            }
            Kind::Operator => {
                debug_assert!(
                    name.len() == 1,
                    "operators must have an 1 character: `{}`",
                    name
                );
                debug_assert!(
                    name.chars().next().unwrap().is_ascii_punctuation(),
                    "operators names must be a symbol: `{}`",
                    name
                )
            }
        }
    }

    pub fn check_variable_name(name: &str) {
        debug_assert!(
            !name.is_empty(),
            "variables and constants names cannot be empty"
        );
        debug_assert!(
            name.chars().all(char::is_alphanumeric),
            "variables and constants names must only include letters and numbers: `{}`",
            name
        );
        debug_assert!(
            name.chars().next().unwrap().is_alphabetic(),
            "variables and constants names must start with a letter: `{}`",
            name
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_context_test() {
        let context: DefaultContext<f64> = DefaultContext::new_checked();

        let a = context.get_constant("E").unwrap();
        let b = context.get_constant("e").unwrap();
        assert_eq!(a, b);

        assert!(context.get_constant("Pi").is_some());
        assert!(context.get_binary_function("+").is_some());
        assert!(context.get_binary_function("-").is_some());
        assert!(context.get_binary_function("/").is_some());
        assert!(context.get_binary_function("*").is_some());

        assert!(context.get_function("SUM").is_some());
        assert!(context.get_function("AvG").is_some());
        assert!(context.get_function("Max").is_some());
        assert!(context.get_function("min").is_some());
    }

    #[test]
    fn config_test() {
        let config = Config::default()
            .with_group_symbol('(', ')')
            .with_group_symbol('[', ']');

        assert_eq!(
            config.get_group_symbol('(').unwrap(),
            &GroupingSymbol::new('(', ')')
        );
        assert_eq!(
            config.get_group_symbol(')').unwrap(),
            &GroupingSymbol::new('(', ')')
        );
        assert_eq!(
            config.get_group_symbol('[').unwrap(),
            &GroupingSymbol::new('[', ']')
        );
        assert_eq!(
            config.get_group_symbol(']').unwrap(),
            &GroupingSymbol::new('[', ']')
        );
    }
}
