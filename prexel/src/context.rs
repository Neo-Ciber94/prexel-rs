use crate::function::{BinaryFunction, Function, UnaryFunction};
use crate::num::checked::CheckedNum;
use crate::num::unchecked::UncheckedNum;
use crate::ops::math::*;
use crate::utils::ignore_case_str::eq_ignore_case;
use crate::utils::ignore_case_string::IgnoreCaseString;
use std::collections::HashSet;
use std::rc::Rc;
use crate::error::{Error, ErrorKind};

#[cfg(debug_assertions)]
use validate::TokenKind;

/// Trait to provides the variables, constants and functions used for evaluate an expression.
pub trait Context<'a, N> {
    /// Gets the configuration of the context.
    fn config(&self) -> &Config;

    /// Adds a function to the context.
    fn add_function<F: Function<N> + 'a>(&mut self, func: F) -> crate::Result<()>;

    /// Adds an unary function to the context.
    fn add_unary_function<F: UnaryFunction<N> + 'a>(&mut self, func: F) -> crate::Result<()>;

    /// Adds a binary function to the context.
    fn add_binary_function<F: BinaryFunction<N> + 'a>(&mut self, func: F) -> crate::Result<()>;

    /// Adds a constant value to the context.
    fn add_constant(&mut self, name: &str, value: N) -> crate::Result<()>;

    /// Adds or set the value of a variable in the context.
    fn set_variable(&mut self, name: &str, value: N) -> crate::Result<Option<N>>;

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

// Maps used for store the variables, constants and functions.

#[cfg(feature="linked-hashmap")]
type Map<K, V> =  ritelinked::LinkedHashMap<K, V>;

#[cfg(not(feature="linked-hashmap"))]
type Map<K, V> = std::collections::HashMap<K, V>;

/// Provides a default implementation of a math `Context`.
#[derive(Clone)]
pub struct DefaultContext<'a, N> {
    /// The variables.
    variables: Map<String, N>,
    /// The constants.
    constants: Map<IgnoreCaseString, N>,
    /// The functions.
    functions: Map<IgnoreCaseString, Rc<dyn Function<N> + 'a>>,
    /// The unary functions.
    unary_functions: Map<IgnoreCaseString, Rc<dyn UnaryFunction<N> + 'a>>,
    /// The binary functions.
    binary_functions: Map<IgnoreCaseString, Rc<dyn BinaryFunction<N> + 'a>>,
    /// Additional information about this context
    config: Config,
}

#[allow(clippy::map_entry)]
impl<'a, N> DefaultContext<'a, N> {
    /// Constructs a new `Context` with no variables, constants or functions.
    #[inline]
    pub fn new() -> Self {
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
    pub fn with_config(config: Config) -> Self {
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
    pub fn variables(&self) -> &Map<String, N> {
        &self.variables
    }

    /// Gets a reference to the constant values of this context.
    #[inline]
    pub fn constants(&self) -> &Map<IgnoreCaseString, N> {
        &self.constants
    }

    /// Gets a reference to the functions of this context.
    #[inline]
    pub fn functions(&self) -> &Map<IgnoreCaseString, Rc<dyn Function<N> + 'a>> {
        &self.functions
    }

    /// Gets a reference to the unary functions of this context.
    #[inline]
    pub fn unary_functions(&self) -> &Map<IgnoreCaseString, Rc<dyn UnaryFunction<N> + 'a>> {
        &self.unary_functions
    }

    /// Gets a reference to the binary functions of this context.
    #[inline]
    pub fn binary_functions(&self) -> &Map<IgnoreCaseString, Rc<dyn BinaryFunction<N> + 'a>> {
        &self.binary_functions
    }

    /// Adds the specified function to the context using the given name.
    ///
    /// # Remarks
    /// - This allows to use a function with an alias.
    ///
    /// # Examples
    /// ```
    /// use prexel::context::{DefaultContext, Context};
    /// use prexel::ops::math::MaxFunction;
    ///
    /// let mut context : DefaultContext<f64> = DefaultContext::new();
    /// context.add_function(MaxFunction);
    /// context.add_function_as(MaxFunction, "Maximum");
    /// ```
    #[inline]
    pub fn add_function_as<F: Function<N> + 'a>(&mut self, func: F, name: &str) -> crate::Result<()> {
        #[cfg(debug_assertions)]
        validate::check_token_name(TokenKind::Function, name)?;

        let function_name = IgnoreCaseString::from(name);
        if self.functions.contains_key(&function_name) {
            Err(Error::new(ErrorKind::Unknown, format!("A function named '{}' already exists", function_name)))
        } else {
            let func = Rc::new(func);

            if let Some(aliases) = func.aliases() {
                for alias in aliases.iter().map(|s| IgnoreCaseString::from(*s)) {
                    if self.functions.contains_key(&alias) {
                        panic!("A function named '{}' already exists", alias);
                    }

                    self.functions.insert(alias, func.clone());
                }
            }
            self.functions.insert(function_name, func);
            Ok(())
        }
    }

    /// Adds the specified unary function to the context using the given name.
    ///
    /// # Remarks
    /// - This allows to use an unary function with an alias.
    #[inline]
    pub fn add_unary_function_as<F: UnaryFunction<N> + 'a>(&mut self, func: F, name: &str) -> crate::Result<()> {
        #[cfg(debug_assertions)]
        validate::check_token_name(TokenKind::Operator, name)?;

        let function_name = IgnoreCaseString::from(name);
        if self.unary_functions.contains_key(&function_name) {
            Err(Error::new(ErrorKind::Unknown, format!("An unary function named '{}' already exists", function_name)))
        } else {
            let func = Rc::new(func);

            if let Some(aliases) = func.aliases() {
                for alias in aliases.iter().map(|s| IgnoreCaseString::from(*s)) {
                    if self.unary_functions.contains_key(&alias) {
                        panic!("An unary function named '{}' already exists", alias);
                    }

                    self.unary_functions.insert(alias, func.clone());
                }
            }

            self.unary_functions.insert(function_name, func);
            Ok(())
        }
    }

    /// Adds the specified binary function to the context using the given name.
    ///
    /// # Remarks
    /// - This allows to use a binary function with an alias.
    ///
    /// # Examples
    /// ```
    /// use prexel::context::{DefaultContext, Context};
    /// use prexel::ops::unchecked::AddOperator;
    ///
    /// let mut context : DefaultContext<f64> = DefaultContext::new();
    /// context.add_binary_function(AddOperator);
    /// context.add_binary_function_as(AddOperator, "Plus");
    /// ```
    #[inline]
    pub fn add_binary_function_as<F: BinaryFunction<N> + 'a>(&mut self, func: F, name: &str) -> crate::Result<()> {
        #[cfg(debug_assertions)]
        validate::check_token_name(TokenKind::Operator, name)?;

        let function_name = IgnoreCaseString::from(name);
        if self.binary_functions.contains_key(&function_name) {
            Err(Error::new(ErrorKind::Unknown, format!("A binary function named '{}' already exists", function_name)))
        } else {
            let func = Rc::new(func);

            if let Some(aliases) = func.aliases() {
                for alias in aliases.iter().map(|s| IgnoreCaseString::from(*s)) {
                    if self.binary_functions.contains_key(&alias) {
                        panic!("A binary function named '{}' already exists", alias);
                    }

                    self.binary_functions.insert(alias, func.clone());
                }
            }

            self.binary_functions.insert(function_name, func);
            Ok(())
        }
    }
}

impl<'a, N> Default for DefaultContext<'a, N> {
    #[inline]
    fn default() -> Self {
        DefaultContext::new()
    }
}

impl<'a, N> Context<'a, N> for DefaultContext<'a, N> {
    #[inline]
    fn config(&self) -> &Config {
        &self.config
    }

    #[inline]
    fn add_function<F: Function<N> + 'a>(&mut self, func: F) -> crate::Result<()> {
        let name = func.name().to_string();
        self.add_function_as(func, &name)
    }

    #[inline]
    fn add_unary_function<F: UnaryFunction<N> + 'a>(&mut self, func: F) -> crate::Result<()> {
        let name = func.name().to_string();
        self.add_unary_function_as(func, &name)
    }

    #[inline]
    fn add_binary_function<F: BinaryFunction<N> + 'a>(&mut self, func: F) -> crate::Result<()> {
        let name = func.name().to_string();
        self.add_binary_function_as(func, &name)
    }

    #[inline]
    fn add_constant(&mut self, name: &str, value: N) -> crate::Result<()> {
        #[cfg(debug_assertions)]
        validate::check_token_name(TokenKind::Constant, name)?;

        //self.constants.insert(IgnoreCaseString::from(name), value);
        // let string = IgnoreCaseString::from(name);
        if self.variables.keys().any(|k| eq_ignore_case(k, name)) {
            Err(Error::new(ErrorKind::Unknown, format!("Invalid constant name, a variable named `{}` exists", name)))
        } else {
            self.constants.insert(IgnoreCaseString::from(name), value);
            Ok(())
        }
    }

    #[inline]
    fn set_variable(&mut self, name: &str, value: N) -> crate::Result<Option<N>> {
        #[cfg(debug_assertions)]
        validate::check_token_name(TokenKind::Variable, name)?;

        if self
            .constants
            .keys()
            .any(|k| eq_ignore_case(k.as_str(), name))
        {
            Err(Error::new(ErrorKind::Unknown, format!("Invalid variable name, a constant named `{}` exists", name)))
        } else {
            Ok(self.variables.insert(name.to_string(), value))
        }
    }

    #[inline]
    fn get_variable(&self, name: &str) -> Option<&N> {
        self.variables.get(name)
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
    /// Constructs a new `Context` with checked functions.
    ///
    /// # Remarks
    /// Some functions may cause overflow exceptions, the functions of this context
    /// ensures will return an error instead of throws an exception.
    #[inline]
    pub fn new_checked() -> Self {
        Self::with_config_checked(Config::new())
    }

    /// Constructs a new `Context` using the given `Config` with checked functions.
    ///
    /// # Remarks
    /// Some functions may cause overflow exceptions, the functions of this context
    /// ensures will return an error instead of throws an exception.
    pub fn with_config_checked(config: Config) -> Self {
        use crate::ops::checked::*;

        let mut context = Self::with_config(config);
        context.add_constant("PI", N::from_f64(std::f64::consts::PI).unwrap()).unwrap();
        context.add_constant("E", N::from_f64(std::f64::consts::E).unwrap()).unwrap();
        context.add_binary_function(AddOperator).unwrap();
        context.add_binary_function(SubOperator).unwrap();
        context.add_binary_function(MulOperator).unwrap();
        context.add_binary_function(DivOperator).unwrap();
        context.add_binary_function(PowOperator).unwrap();
        context.add_binary_function(ModOperator).unwrap();
        context.add_unary_function(UnaryPlus).unwrap();
        context.add_unary_function(UnaryMinus).unwrap();
        context.add_unary_function(Factorial).unwrap();
        context.add_function(SumFunction).unwrap();
        context.add_function(ProdFunction).unwrap();
        context.add_function(AvgFunction).unwrap();
        context.add_function(MaxFunction).unwrap();
        context.add_function(MinFunction).unwrap();
        context.add_function(AbsFunction).unwrap();
        context.add_function(SqrtFunction).unwrap();
        context.add_function(LnFunction).unwrap();
        context.add_function(LogFunction).unwrap();
        context.add_function(ExpFunction).unwrap();
        context.add_function(FloorFunction).unwrap();
        context.add_function(CeilFunction).unwrap();
        context.add_function(TruncateFunction).unwrap();
        context.add_function(RoundFunction).unwrap();
        context.add_function(SignFunction).unwrap();
        context.add_function(RandFunction).unwrap();
        context.add_function(ToRadiansFunction).unwrap();
        context.add_function(ToDegreesFunction).unwrap();
        context.add_function(SinFunction).unwrap();
        context.add_function(CosFunction).unwrap();
        context.add_function(TanFunction).unwrap();
        context.add_function(CscFunction).unwrap();
        context.add_function(SecFunction).unwrap();
        context.add_function(CotFunction).unwrap();
        context.add_function(ASinFunction).unwrap();
        context.add_function(ACosFunction).unwrap();
        context.add_function(ATanFunction).unwrap();
        context.add_function(ACscFunction).unwrap();
        context.add_function(ASecFunction).unwrap();
        context.add_function(ACotFunction).unwrap();
        context.add_function(SinhFunction).unwrap();
        context.add_function(CoshFunction).unwrap();
        context.add_function(TanhFunction).unwrap();
        context.add_function(CschFunction).unwrap();
        context.add_function(SechFunction).unwrap();
        context.add_function(CothFunction).unwrap();
        context.add_function(ASinhFunction).unwrap();
        context.add_function(ACoshFunction).unwrap();
        context.add_function(ATanhFunction).unwrap();
        context.add_function(ACschFunction).unwrap();
        context.add_function(ASechFunction).unwrap();
        context.add_function(ACothFunction).unwrap();
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
        Self::with_config_unchecked(Config::new())
    }

    /// Constructs a new `Context` using the given `Config` with unchecked functions.
    ///
    /// # Remarks
    /// Functions of this context may panic when the value overflows.
    pub fn with_config_unchecked(config: Config) -> Self {
        use crate::ops::unchecked::*;

        let mut context = Self::with_config(config);
        context.add_constant("PI", N::from_f64(std::f64::consts::PI).unwrap()).unwrap();
        context.add_constant("E", N::from_f64(std::f64::consts::E).unwrap()).unwrap();
        context.add_binary_function(AddOperator).unwrap();
        context.add_binary_function(SubOperator).unwrap();
        context.add_binary_function(MulOperator).unwrap();
        context.add_binary_function(DivOperator).unwrap();
        context.add_binary_function(PowOperator).unwrap();
        context.add_binary_function(ModOperator).unwrap();
        context.add_unary_function(UnaryPlus).unwrap();
        context.add_unary_function(UnaryMinus).unwrap();
        context.add_unary_function(Factorial).unwrap();
        context.add_function(SumFunction).unwrap();
        context.add_function(AvgFunction).unwrap();
        context.add_function(ProdFunction).unwrap();
        context.add_function(MaxFunction).unwrap();
        context.add_function(MinFunction).unwrap();
        context.add_function(SqrtFunction).unwrap();
        context.add_function(LnFunction).unwrap();
        context.add_function(LogFunction).unwrap();
        context.add_function(RandFunction).unwrap();
        context.add_function(ToRadiansFunction).unwrap();
        context.add_function(ToDegreesFunction).unwrap();
        context.add_function(ExpFunction).unwrap();
        context.add_function(SinFunction).unwrap();
        context.add_function(CosFunction).unwrap();
        context.add_function(TanFunction).unwrap();
        context.add_function(CscFunction).unwrap();
        context.add_function(SecFunction).unwrap();
        context.add_function(CotFunction).unwrap();
        context.add_function(ASinFunction).unwrap();
        context.add_function(ACosFunction).unwrap();
        context.add_function(ATanFunction).unwrap();
        context.add_function(ACscFunction).unwrap();
        context.add_function(ASecFunction).unwrap();
        context.add_function(ACotFunction).unwrap();
        context.add_function(SinhFunction).unwrap();
        context.add_function(CoshFunction).unwrap();
        context.add_function(TanhFunction).unwrap();
        context.add_function(CschFunction).unwrap();
        context.add_function(SechFunction).unwrap();
        context.add_function(CothFunction).unwrap();
        context.add_function(ASinhFunction).unwrap();
        context.add_function(ACoshFunction).unwrap();
        context.add_function(ATanhFunction).unwrap();
        context.add_function(ACschFunction).unwrap();
        context.add_function(ASechFunction).unwrap();
        context.add_function(ACothFunction).unwrap();
        context
    }
}

/// Represents the configuration used by a `Context`.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Config {
    /// Allows implicit multiplication.
    pub implicit_mul: bool,
    /// Allows complex numbers.
    pub complex_number: bool,
    /// Allows using custom grouping symbols for function calls, eg: `Max[1,2,3]`, `Sum{2,4,6}`
    pub custom_function_call: bool,
    /// Stores the grouping symbols as: `(`, `)`, `[`, `]`.
    grouping: HashSet<Grouping>,
}

impl Config {
    /// Constructs a new `Config` using the default grouping symbol: `(`, `)`,
    /// if is need an empty `Config` use `Default` instead.
    #[inline]
    pub fn new() -> Self {
        Config::default().with_grouping(Grouping::Parenthesis)
    }

    /// Enables implicit multiplication for this `Config`.
    #[inline]
    pub fn with_implicit_mul(mut self, enable: bool) -> Config {
        self.implicit_mul = enable;
        self
    }

    /// Enables complex number usage for this `Config`.
    ///
    /// # Remarks
    /// [`Tokenizer`] checks for this value when parsing expressions.
    ///
    /// [`Tokenizer`]: ../tokenizer/struct.Tokenizer.html
    #[inline]
    pub fn with_complex_number(mut self, enable: bool) -> Config {
        self.complex_number = enable;
        self
    }

    /// Enables custom function calls groping symbols.
    ///
    /// # Remarks
    /// Function calls are only allowed within parentheses, eg: `Product(3, 6, 6)`,
    /// but `with_custom_function_call` allow to use others, eg: `Max[1,2,3]`, `Sum<2,4,6>`.
    #[inline]
    pub fn with_custom_function_call(mut self, enable: bool) -> Config {
        self.custom_function_call = enable;
        self
    }

    /// Adds a pair of grouping symbols to this `Config`.
    ///
    /// # Example
    /// ```
    /// use prexel::context::{Config, Grouping};
    ///
    /// // `Default` allows to create an empty config
    /// let mut config = Config::default()
    ///     .with_grouping(Grouping::Parenthesis)
    ///     .with_grouping(Grouping::Bracket);
    /// ```
    pub fn with_grouping(mut self, grouping: Grouping) -> Config {
        self.grouping.insert(grouping);
        self
    }

    /// Gets a grouping symbol pair from this `Config`.
    ///
    /// # Examples
    /// ```
    /// use prexel::context::{Config, Grouping};
    ///
    /// let mut config = Config::new().with_grouping(Grouping::Parenthesis);
    /// assert_eq!(('(', ')'), config.get_group_symbol('(').unwrap());
    /// ```
    #[inline]
    pub fn get_group_symbol(&self, symbol: char) -> Option<(char, char)> {
        Grouping::new(symbol).map(|g| g.symbols())
    }

    /// Gets the grouping close for the specified grouping open.
    ///
    /// # Example
    /// ```
    /// use prexel::context::{Config, Grouping};
    ///
    /// let config = Config::default()
    ///     .with_grouping(Grouping::Parenthesis)
    ///     .with_grouping(Grouping::Bracket);
    ///
    /// assert_eq!(Some('('), config.get_group_open_for(')'));
    /// assert_eq!(Some('['), config.get_group_open_for(']'));
    /// assert_eq!(None, config.get_group_open_for('['));
    /// ```
    #[inline]
    pub fn get_group_open_for(&self, group_close: char) -> Option<char> {
        Grouping::new(group_close)
            .map(|g| g.open_for(group_close))
            .flatten()
    }

    /// Gets the grouping close for the specified grouping open.
    ///
    /// # Example
    /// ```
    /// use prexel::context::{Config, Grouping};
    ///
    /// let config = Config::default()
    ///     .with_grouping(Grouping::Parenthesis)
    ///     .with_grouping(Grouping::Bracket);
    ///
    /// assert_eq!(Some(')'), config.get_group_close_for('('));
    /// assert_eq!(Some(']'), config.get_group_close_for('['));
    /// assert_eq!(None, config.get_group_close_for(']'));
    /// ```
    #[inline]
    pub fn get_group_close_for(&self, group_open: char) -> Option<char> {
        Grouping::new(group_open)
            .map(|g| g.close_for(group_open))
            .flatten()
    }

    /// Checks a value indicating if the given `char` is a group close symbol.
    #[inline]
    pub fn is_group_close(&self, group_close: char) -> bool {
        Grouping::new(group_close)
            .map(|g| g.is_close(group_close))
            .unwrap_or(false)
    }

    /// Checks a value indicating if the given `char` is a group open symbol.
    #[inline]
    pub fn is_group_open(&self, group_open: char) -> bool {
        Grouping::new(group_open)
            .map(|g| g.is_open(group_open))
            .unwrap_or(false)
    }
}

/// Represents a grouping symbol pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Grouping {
    /// Grouping using parentheses: `(` and `)`.
    Parenthesis,
    /// Grouping using square brackets: `[` and `]`.
    Bracket,
    /// Grouping using curly braces: `{` and `}`.
    Brace,
}

impl Grouping {
    /// Constructs a `Grouping` from the given `char`s.
    pub fn new(c: char) -> Option<Grouping> {
        match c {
            '(' | ')' => Some(Grouping::Parenthesis),
            '[' | ']' => Some(Grouping::Bracket),
            '{' | '}' => Some(Grouping::Brace),
            _ => None,
        }
    }

    #[inline]
    pub fn symbols(&self) -> (char, char) {
        (self.open(), self.close())
    }

    pub fn open(&self) -> char {
        match self {
            Grouping::Parenthesis => '(',
            Grouping::Bracket => '[',
            Grouping::Brace => '{',
        }
    }

    pub fn close(&self) -> char {
        match self {
            Grouping::Parenthesis => ')',
            Grouping::Bracket => ']',
            Grouping::Brace => '}',
        }
    }

    /// Returns `true` if the given `char` is a grouping open symbol.
    #[inline]
    pub fn is_open(&self, c: char) -> bool {
        self.open() == c
    }

    /// Returns `true` if the given `char` is a grouping close symbol.
    #[inline]
    pub fn is_close(&self, c: char) -> bool {
        self.close() == c
    }

    /// Returns the grouping open symbol for the given `char`.
    pub fn close_for(&self, c: char) -> Option<char> {
        if self.is_open(c) {
            Self::new(c).map(|g| g.close())
        } else {
            None
        }
    }

    /// Returns the grouping close symbol for the given `char`.
    pub fn open_for(&self, c: char) -> Option<char> {
        if self.is_close(c) {
            Self::new(c).map(|g| g.open())
        } else {
            None
        }
    }
}

/// Provides a function for validate token names.
pub mod validate {
    use crate::error::*;
    use crate::Result;
    use std::fmt::{Debug, Display, Formatter};
    use std::result;

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum TokenKind {
        Variable,
        Constant,
        Operator,
        Function,
    }

    impl Display for TokenKind {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                TokenKind::Variable => write!(f, "Variable"),
                TokenKind::Constant => write!(f, "Constant"),
                TokenKind::Operator => write!(f, "Operator"),
                TokenKind::Function => write!(f, "Function"),
            }
        }
    }

    #[allow(clippy::redundant_closure)]
    pub fn check_token_name(kind: TokenKind, name: &str) -> Result<()> {
        if name.trim().is_empty() {
            return Err(Error::new(
                ErrorKind::Empty,
                format!("{} name is empty", kind),
            ));
        }

        if name.chars().any(char::is_whitespace) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("{} names cannot contain whitespaces: `{}`", kind, name),
            ));
        }

        if name.chars().any(char::is_control) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "{} names cannot contain control characters: `{}`",
                    kind, name
                ),
            ));
        }

        Ok(())
    }

    pub trait OrPanic<T, E> {
        fn or_panic(self) -> T;
    }

    impl<T, E: Debug> OrPanic<T, E> for result::Result<T, E> {
        #[inline]
        fn or_panic(self) -> T {
            if self.is_err() {
                panic!("{:?}", self.err().unwrap())
            }

            self.unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::function::{Associativity, Notation, Precedence};
    use crate::Result;

    struct Dummy(String);

    impl BinaryFunction<f64> for Dummy {
        fn name(&self) -> &str {
            &self.0
        }

        fn precedence(&self) -> Precedence {
            unimplemented!()
        }

        fn associativity(&self) -> Associativity {
            unimplemented!()
        }

        fn call(&self, _: f64, _: f64) -> Result<f64> {
            unimplemented!()
        }
    }

    impl UnaryFunction<f64> for Dummy {
        fn name(&self) -> &str {
            &self.0
        }

        fn notation(&self) -> Notation {
            unimplemented!()
        }

        fn call(&self, _: f64) -> Result<f64> {
            unimplemented!()
        }
    }

    impl Function<f64> for Dummy {
        fn name(&self) -> &str {
            &self.0
        }

        fn call(&self, _: &[f64]) -> Result<f64> {
            unimplemented!()
        }
    }

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
            .with_grouping(Grouping::Parenthesis)
            .with_grouping(Grouping::Bracket);

        assert_eq!(config.get_group_symbol('(').unwrap(), ('(', ')'));
        assert_eq!(config.get_group_symbol(')').unwrap(), ('(', ')'));
        assert_eq!(config.get_group_symbol('[').unwrap(), ('[', ']'));
        assert_eq!(config.get_group_symbol(']').unwrap(), ('[', ']'));
    }

    #[test]
    fn operators_symbols_test() {
        let mut context: DefaultContext<f64> = DefaultContext::new_unchecked();
        context.add_constant("∞", f64::INFINITY).unwrap();
        context.add_constant("π", std::f64::consts::PI).unwrap();
        context.add_binary_function(Dummy("√".to_string())).unwrap();
        context.add_binary_function(Dummy("∋".to_string())).unwrap();
        context.add_unary_function(Dummy("ℝ".to_string())).unwrap();
        context.add_unary_function(Dummy("λ".to_string())).unwrap();
        context.add_function(Dummy("f".to_string())).unwrap();

        assert!(context.is_constant("∞"));
        assert!(context.is_constant("π"));
        assert!(context.is_binary_function("√"));
        assert!(context.is_binary_function("∋"));
        assert!(context.is_unary_function("ℝ"));
        assert!(context.is_unary_function("λ"));
        assert!(context.is_function("f"));
    }

    #[test]
    fn function_aliases_test() {
        struct SumFunction;
        impl Function<f64> for SumFunction {
            fn name(&self) -> &str {
                "sum"
            }

            fn call(&self, args: &[f64]) -> Result<f64> {
                Ok(args.iter().sum())
            }

            fn aliases(&self) -> Option<&[&str]> {
                Some(&["add", "∑"])
            }
        }

        let mut context: DefaultContext<f64> = DefaultContext::new();
        context.add_function(SumFunction).unwrap();

        assert!(context.is_function("sum"));
        assert!(context.is_function("add"));
        assert!(context.is_function("∑"));

        assert!(context.get_function("sum").is_some());
        assert!(context.get_function("add").is_some());
        assert!(context.get_function("∑").is_some());
    }

    #[test]
    fn binary_function_alias_test() {
        struct AddFunction;
        impl BinaryFunction<f64> for AddFunction {
            fn name(&self) -> &str {
                "+"
            }

            fn aliases(&self) -> Option<&[&str]> {
                Some(&["add", "plus"])
            }

            fn precedence(&self) -> Precedence {
                Precedence::VERY_LOW
            }

            fn associativity(&self) -> Associativity {
                Associativity::Left
            }

            fn call(&self, left: f64, right: f64) -> Result<f64> {
                Ok(left + right)
            }
        }

        let mut context: DefaultContext<f64> = DefaultContext::new();
        context.add_binary_function(AddFunction).unwrap();

        assert!(context.is_binary_function("+"));
        assert!(context.is_binary_function("add"));
        assert!(context.is_binary_function("plus"));

        assert!(context.get_binary_function("+").is_some());
        assert!(context.get_binary_function("add").is_some());
        assert!(context.get_binary_function("plus").is_some());
    }

    #[test]
    fn unary_function_alias_test() {
        struct NotFunction;
        impl UnaryFunction<i64> for NotFunction {
            fn name(&self) -> &str {
                "not"
            }

            fn aliases(&self) -> Option<&[&str]> {
                Some(&["¬"])
            }

            fn notation(&self) -> Notation {
                Notation::Prefix
            }

            fn call(&self, arg: i64) -> Result<i64> {
                Ok(!arg)
            }
        }

        let mut context: DefaultContext<i64> = DefaultContext::new();
        context.add_unary_function(NotFunction).unwrap();

        assert!(context.is_unary_function("not"));
        assert!(context.is_unary_function("¬"));

        assert!(context.get_unary_function("not").is_some());
        assert!(context.get_unary_function("¬").is_some());
    }
}
