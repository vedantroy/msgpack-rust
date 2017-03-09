//! Contains Value and ValueRef structs and its conversion traits.
//!
//! # Examples
//!
//! ```
//! ```

#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde;
extern crate rmp;
extern crate num_traits;

use std::borrow::Cow;
use std::fmt::{self, Debug, Display};
use std::ops::Index;
use std::str::Utf8Error;

use num_traits::NumCast;

pub mod decode;
pub mod encode;

#[cfg(feature = "with-serde")]
pub mod ext;

#[derive(Copy, Clone, Debug, PartialEq)]
enum IntPriv {
    /// Always non-less than zero.
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
}

/// Represents a MessagePack integer, whether signed or unsigned.
///
/// A `Value` or `ValueRef` that contains integer can be constructed using `From` trait.
#[derive(Copy, Clone, PartialEq)]
pub struct Integer {
    n: IntPriv,
}

impl Integer {
    /// Returns `true` if the integer can be represented as `i64`.
    #[inline]
    pub fn is_i64(&self) -> bool {
        match self.n {
            IntPriv::PosInt(n) => n <= std::i64::MAX as u64,
            IntPriv::NegInt(..) => true,
        }
    }

    /// Returns `true` if the integer can be represented as `u64`.
    #[inline]
    pub fn is_u64(&self) -> bool {
        match self.n {
            IntPriv::PosInt(..) => true,
            IntPriv::NegInt(..) => false,
        }
    }

    /// Returns the integer represented as `i64` if possible, or else `None`.
    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self.n {
            IntPriv::PosInt(n) => NumCast::from(n),
            IntPriv::NegInt(n) => Some(n),
        }
    }

    /// Returns the integer represented as `u64` if possible, or else `None`.
    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        match self.n {
            IntPriv::PosInt(n) => Some(n),
            IntPriv::NegInt(n) => NumCast::from(n),
        }
    }

    /// Returns the integer represented as `f64` if possible, or else `None`.
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self.n {
            IntPriv::PosInt(n) => NumCast::from(n),
            IntPriv::NegInt(n) => NumCast::from(n),
        }
    }
}

impl Debug for Integer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Debug::fmt(&self.n, fmt)
    }
}

impl Display for Integer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.n {
            IntPriv::PosInt(v) => Display::fmt(&v, fmt),
            IntPriv::NegInt(v) => Display::fmt(&v, fmt),
        }
    }
}

impl From<u8> for Integer {
    fn from(n: u8) -> Self {
        Integer { n: IntPriv::PosInt(n as u64) }
    }
}

impl From<u16> for Integer {
    fn from(n: u16) -> Self {
        Integer { n: IntPriv::PosInt(n as u64) }
    }
}

impl From<u32> for Integer {
    fn from(n: u32) -> Self {
        Integer { n: IntPriv::PosInt(n as u64) }
    }
}

impl From<u64> for Integer {
    fn from(n: u64) -> Self {
        Integer { n: IntPriv::PosInt(n as u64) }
    }
}

impl From<usize> for Integer {
    fn from(n: usize) -> Self {
        Integer { n: IntPriv::PosInt(n as u64) }
    }
}

impl From<i8> for Integer {
    fn from(n: i8) -> Self {
        if n < 0 {
            Integer { n: IntPriv::NegInt(n as i64) }
        } else {
            Integer { n: IntPriv::PosInt(n as u64) }
        }
    }
}

impl From<i16> for Integer {
    fn from(n: i16) -> Self {
        if n < 0 {
            Integer { n: IntPriv::NegInt(n as i64) }
        } else {
            Integer { n: IntPriv::PosInt(n as u64) }
        }
    }
}

impl From<i32> for Integer {
    fn from(n: i32) -> Self {
        if n < 0 {
            Integer { n: IntPriv::NegInt(n as i64) }
        } else {
            Integer { n: IntPriv::PosInt(n as u64) }
        }
    }
}

impl From<i64> for Integer {
    fn from(n: i64) -> Self {
        if n < 0 {
            Integer { n: IntPriv::NegInt(n as i64) }
        } else {
            Integer { n: IntPriv::PosInt(n as u64) }
        }
    }
}

impl From<isize> for Integer {
    fn from(n: isize) -> Self {
        if n < 0 {
            Integer { n: IntPriv::NegInt(n as i64) }
        } else {
            Integer { n: IntPriv::PosInt(n as u64) }
        }
    }
}

/// Represents an UTF-8 MessagePack string type.
///
/// According to the MessagePack spec, string objects may contain invalid byte sequence and the
/// behavior of a deserializer depends on the actual implementation when it received invalid byte
/// sequence.
/// Deserializers should provide functionality to get the original byte array so that applications
/// can decide how to handle the object.
///
/// Summarizing, it's prohibited to instantiate a string type with invalid UTF-8 sequences, however
/// it is possible to obtain an underlying bytes that were attempted to convert to a `String`. This
/// may happen when trying to unpack strings that were decoded using older MessagePack spec with
/// raw types instead of string/binary.
#[derive(Clone, Debug, PartialEq)]
pub struct Utf8String {
    s: Result<String, (Vec<u8>, Utf8Error)>,
}

impl Utf8String {
    /// Returns `true` if the string is valid UTF-8.
    pub fn is_str(&self) -> bool {
        self.s.is_ok()
    }

    /// Returns `true` if the string contains invalid UTF-8 sequence.
    pub fn is_err(&self) -> bool {
        self.s.is_err()
    }

    /// Returns the string reference if the string is valid UTF-8, or else `None`.
    pub fn as_str(&self) -> Option<&str> {
        match self.s {
            Ok(ref s) => Some(s.as_str()),
            Err(..) => None,
        }
    }

    /// Returns the underlying `Utf8Error` if the string contains invalud UTF-8 sequence, or
    /// else `None`.
    pub fn as_err(&self) -> Option<&Utf8Error> {
        match self.s {
            Ok(..) => None,
            Err((.., ref err)) => Some(&err),
        }
    }

    /// Returns a byte slice of this `Utf8String`'s contents.
    pub fn as_bytes(&self) -> &[u8] {
        match self.s {
            Ok(ref s) => s.as_bytes(),
            Err(ref err) => &err.0[..],
        }
    }

    /// Consumes this object, yielding the string if the string is valid UTF-8, or else `None`.
    pub fn into_str(self) -> Option<String> {
        self.s.ok()
    }

    /// Converts a `Utf8String` into a byte vector.
    pub fn into_bytes(self) -> Vec<u8> {
        match self.s {
            Ok(s) => s.into_bytes(),
            Err(err) => err.0,
        }
    }
}

impl Display for Utf8String {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.s {
            Ok(ref s) => write!(fmt, "\"{}\"", s),
            Err(ref err) => Debug::fmt(&err.0, fmt),
        }
    }
}

impl<'a> From<String> for Utf8String {
    fn from(val: String) -> Self {
        Utf8String {
            s: Ok(val),
        }
    }
}

impl<'a> From<&'a str> for Utf8String {
    fn from(val: &str) -> Self {
        Utf8String {
            s: Ok(val.into()),
        }
    }
}

impl<'a> From<Cow<'a, str>> for Utf8String {
    fn from(val: Cow<'a, str>) -> Self {
        Utf8String {
            s: Ok(val.into()),
        }
    }
}

/// Represents any valid MessagePack value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Nil represents nil.
    Nil,
    /// Boolean represents true or false.
    Boolean(bool),
    /// Integer represents an integer.
    ///
    /// A value of an `Integer` object is limited from `-(2^63)` upto `(2^64)-1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert_eq!(42, Value::from(42).as_i64().unwrap());
    /// ```
    Integer(Integer),
    /// A 32-bit floating point number.
    F32(f32),
    /// A 64-bit floating point number.
    F64(f64),
    /// String extending Raw type represents a UTF-8 string.
    ///
    /// # Note
    ///
    /// String objects may contain invalid byte sequence and the behavior of a deserializer depends
    /// on the actual implementation when it received invalid byte sequence. Deserializers should
    /// provide functionality to get the original byte array so that applications can decide how to
    /// handle the object
    String(Utf8String),
    /// Binary extending Raw type represents a byte array.
    Binary(Vec<u8>),
    /// Array represents a sequence of objects.
    Array(Vec<Value>),
    /// Map represents key-value pairs of objects.
    Map(Vec<(Value, Value)>),
    /// Extended implements Extension interface: represents a tuple of type information and a byte
    /// array where type information is an integer whose meaning is defined by applications.
    Ext(i8, Vec<u8>),
}

impl Value {
    /// Returns true if the `Value` is a Null. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert!(Value::Nil.is_nil());
    /// ```
    pub fn is_nil(&self) -> bool {
        if let Value::Nil = *self {
            true
        } else {
            false
        }
    }

    /// Returns true if the `Value` is a Boolean. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert!(Value::Boolean(true).is_bool());
    ///
    /// assert!(!Value::Nil.is_bool());
    /// ```
    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    /// Returns true if the `Value` is convertible to an i64. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert!(Value::from(42).is_i64());
    ///
    /// assert!(!Value::from(42.0).is_i64());
    /// ```
    pub fn is_i64(&self) -> bool {
        if let Value::Integer(ref v) = *self {
            v.is_i64()
        } else {
            false
        }
    }

    /// Returns true if the `Value` is convertible to an u64. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert!(Value::from(42).is_u64());
    ///
    /// assert!(!Value::F32(42.0).is_u64());
    /// assert!(!Value::F64(42.0).is_u64());
    /// ```
    pub fn is_u64(&self) -> bool {
        if let Value::Integer(ref v) = *self {
            v.is_u64()
        } else {
            false
        }
    }

    /// Returns true if (and only if) the `Value` is a f32. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert!(Value::F32(42.0).is_f32());
    ///
    /// assert!(!Value::from(42).is_f32());
    /// assert!(!Value::F64(42.0).is_f32());
    /// ```
    pub fn is_f32(&self) -> bool {
        if let Value::F32(..) = *self {
            true
        } else {
            false
        }
    }

    /// Returns true if (and only if) the `Value` is a f64. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert!(Value::F64(42.0).is_f64());
    ///
    /// assert!(!Value::from(42).is_f64());
    /// assert!(!Value::F32(42.0).is_f64());
    /// ```
    pub fn is_f64(&self) -> bool {
        if let Value::F64(..) = *self {
            true
        } else {
            false
        }
    }

    /// Returns true if the `Value` is a Number. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert!(Value::from(42).is_number());
    /// assert!(Value::F32(42.0).is_number());
    /// assert!(Value::F64(42.0).is_number());
    ///
    /// assert!(!Value::Nil.is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        match *self {
            Value::Integer(..) | Value::F32(..) | Value::F64(..) => true,
            _ => false,
        }
    }

    /// Returns true if the `Value` is a String. Returns false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert!(Value::String("value".into()).is_str());
    ///
    /// assert!(!Value::Nil.is_str());
    /// ```
    pub fn is_str(&self) -> bool {
        self.as_str().is_some()
    }

    /// Returns true if the `Value` is a Binary. Returns false otherwise.
    pub fn is_bin(&self) -> bool {
        self.as_slice().is_some()
    }

    /// Returns true if the `Value` is an Array. Returns false otherwise.
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    /// Returns true if the `Value` is a Map. Returns false otherwise.
    pub fn is_map(&self) -> bool {
        self.as_map().is_some()
    }

    /// Returns true if the `Value` is an Ext. Returns false otherwise.
    pub fn is_ext(&self) -> bool {
        self.as_ext().is_some()
    }

    /// If the `Value` is a Boolean, returns the associated bool.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert_eq!(Some(true), Value::Boolean(true).as_bool());
    ///
    /// assert_eq!(None, Value::Nil.as_bool());
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Boolean(val) = *self {
            Some(val)
        } else {
            None
        }
    }

    /// If the `Value` is an integer, return or cast it to a i64.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert_eq!(Some(42i64), Value::from(42).as_i64());
    ///
    /// assert_eq!(None, Value::F64(42.0).as_i64());
    /// ```
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::Integer(ref n) => n.as_i64(),
            _ => None,
        }
    }

    /// If the `Value` is an integer, return or cast it to a u64.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert_eq!(Some(42u64), Value::from(42).as_u64());
    ///
    /// assert_eq!(None, Value::from(-42).as_u64());
    /// assert_eq!(None, Value::F64(42.0).as_u64());
    /// ```
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Value::Integer(ref n) => n.as_u64(),
            _ => None,
        }
    }

    /// If the `Value` is a number, return or cast it to a f64.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert_eq!(Some(42.0), Value::from(42).as_f64());
    /// assert_eq!(Some(42.0), Value::F32(42.0f32).as_f64());
    /// assert_eq!(Some(42.0), Value::F64(42.0f64).as_f64());
    ///
    /// assert_eq!(Some(2147483647.0), Value::from(i32::max_value() as i64).as_f64());
    ///
    /// assert_eq!(None, Value::Nil.as_f64());
    /// ```
    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Value::Integer(ref n) => n.as_f64(),
            Value::F32(n) => Some(From::from(n)),
            Value::F64(n) => Some(n),
            _ => None,
        }
    }

    /// If the `Value` is a String, returns the associated str.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert_eq!(Some("le message"), Value::String("le message".into()).as_str());
    ///
    /// assert_eq!(None, Value::Boolean(true).as_str());
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        if let Value::String(ref val) = *self {
            val.as_str()
        } else {
            None
        }
    }

    /// If the `Value` is a Binary or a String, returns the associated slice.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert_eq!(Some(&[1, 2, 3, 4, 5][..]), Value::Binary(vec![1, 2, 3, 4, 5]).as_slice());
    ///
    /// assert_eq!(None, Value::Boolean(true).as_slice());
    /// ```
    pub fn as_slice(&self) -> Option<&[u8]> {
        if let Value::Binary(ref val) = *self {
            Some(val)
        } else if let Value::String(ref val) = *self {
            Some(val.as_bytes())
        } else {
            None
        }
    }

    /// If the `Value` is an Array, returns the associated vector.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// let val = Value::Array(vec![Value::Nil, Value::Boolean(true)]);
    ///
    /// assert_eq!(Some(&vec![Value::Nil, Value::Boolean(true)]), val.as_array());
    ///
    /// assert_eq!(None, Value::Nil.as_array());
    /// ```
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        if let Value::Array(ref array) = *self {
            Some(&*array)
        } else {
            None
        }
    }

    /// If the `Value` is a Map, returns the associated vector of key-value tuples.
    /// Returns None otherwise.
    ///
    /// # Note
    ///
    /// MessagePack represents map as a vector of key-value tuples.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// let val = Value::Map(vec![(Value::Nil, Value::Boolean(true))]);
    ///
    /// assert_eq!(Some(&vec![(Value::Nil, Value::Boolean(true))]), val.as_map());
    ///
    /// assert_eq!(None, Value::Nil.as_map());
    /// ```
    pub fn as_map(&self) -> Option<&Vec<(Value, Value)>> {
        if let Value::Map(ref map) = *self {
            Some(map)
        } else {
            None
        }
    }

    /// If the `Value` is an Ext, returns the associated tuple with a ty and slice.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::Value;
    ///
    /// assert_eq!(Some((42, &[1, 2, 3, 4, 5][..])), Value::Ext(42, vec![1, 2, 3, 4, 5]).as_ext());
    ///
    /// assert_eq!(None, Value::Boolean(true).as_ext());
    /// ```
    pub fn as_ext(&self) -> Option<(i8, &[u8])> {
        if let Value::Ext(ty, ref buf) = *self {
            Some((ty, buf))
        } else {
            None
        }
    }
}

static NIL: Value = Value::Nil;
static NIL_REF: ValueRef<'static> = ValueRef::Nil;

impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Value {
        self.as_array().and_then(|v| v.get(index)).unwrap_or(&NIL)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Value {
        Value::Boolean(v)
    }
}

impl From<u8> for Value {
    fn from(v: u8) -> Value {
        Value::Integer(From::from(v))
    }
}

impl From<u16> for Value {
    fn from(v: u16) -> Value {
        Value::Integer(From::from(v))
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Value {
        Value::Integer(From::from(v))
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Value {
        Value::Integer(From::from(v))
    }
}

impl From<usize> for Value {
    fn from(v: usize) -> Value {
        Value::Integer(From::from(v))
    }
}

impl From<i8> for Value {
    fn from(v: i8) -> Value {
        Value::Integer(From::from(v))
    }
}

impl From<i16> for Value {
    fn from(v: i16) -> Value {
        Value::Integer(From::from(v))
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Value {
        Value::Integer(From::from(v))
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Value {
        Value::Integer(From::from(v))
    }
}

impl From<isize> for Value {
    fn from(v: isize) -> Value {
        Value::Integer(From::from(v))
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Value {
        Value::F32(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Value {
        Value::F64(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(Utf8String::from(v))
    }
}

impl<'a> From<&'a str> for Value {
    fn from(v: &str) -> Self {
        Value::String(Utf8String::from(v))
    }
}

impl<'a> From<Cow<'a, str>> for Value {
    fn from(v: Cow<'a, str>) -> Self {
        Value::String(Utf8String::from(v))
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Binary(v)
    }
}

impl<'a> From<&'a [u8]> for Value {
    fn from(v: &[u8]) -> Self {
        Value::Binary(v.into())
    }
}

impl<'a> From<Cow<'a, [u8]>> for Value {
    fn from(v: Cow<'a, [u8]>) -> Self {
        Value::Binary(v.into())
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Value::Array(v)
    }
}

impl From<Vec<(Value, Value)>> for Value {
    fn from(v: Vec<(Value, Value)>) -> Self {
        Value::Map(v)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Value::Nil => Display::fmt("nil", f),
            Value::Boolean(val) => write!(f, "{}", val),
            Value::Integer(ref val) => write!(f, "{}", val),
            Value::F32(val) => write!(f, "{}", val),
            Value::F64(val) => write!(f, "{}", val),
            Value::String(ref val) => write!(f, "{}", val),
            Value::Binary(ref val) => write!(f, "{:?}", val),
            Value::Array(ref vec) => {
                // TODO: This can be slower than naive implementation. Need benchmarks for more
                // information.
                let res = vec.iter()
                    .map(|val| format!("{}", val))
                    .collect::<Vec<String>>()
                    .join(", ");

                write!(f, "[{}]", res)
            }
            Value::Map(ref vec) => {
                try!(write!(f, "{{"));

                match vec.iter().take(1).next() {
                    Some(&(ref k, ref v)) => {
                        try!(write!(f, "{}: {}", k, v));
                    }
                    None => {
                        try!(write!(f, ""));
                    }
                }

                for &(ref k, ref v) in vec.iter().skip(1) {
                    try!(write!(f, ", {}: {}", k, v));
                }

                write!(f, "}}")
            }
            Value::Ext(ty, ref data) => {
                write!(f, "[{}, {:?}]", ty, data)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueRef<'a> {
    /// Nil represents nil.
    Nil,
    /// Boolean represents true or false.
    Boolean(bool),
    Integer(Integer),
    /// A 32-bit floating point number.
    F32(f32),
    /// A 64-bit floating point number.
    F64(f64),
    /// String extending Raw type represents a UTF-8 string.
    String(&'a str),
    /// Binary extending Raw type represents a byte array.
    Binary(&'a [u8]),
    /// Array represents a sequence of objects.
    Array(Vec<ValueRef<'a>>),
    /// Map represents key-value pairs of objects.
    Map(Vec<(ValueRef<'a>, ValueRef<'a>)>),
    /// Extended implements Extension interface: represents a tuple of type information and a byte
    /// array where type information is an integer whose meaning is defined by applications.
    Ext(i8, &'a [u8]),
}

impl<'a> ValueRef<'a> {
    /// Converts the current non-owning value to an owned Value.
    ///
    /// This is achieved by deep copying all underlying structures and borrowed buffers.
    ///
    /// # Panics
    ///
    /// Panics in unable to allocate memory to keep all internal structures and buffers.
    ///
    /// # Examples
    /// ```
    /// use rmpv::{Value, ValueRef};
    ///
    /// let val = ValueRef::Array(vec![
    ///    ValueRef::Nil,
    ///    ValueRef::from(42),
    ///    ValueRef::Array(vec![
    ///        ValueRef::String("le message"),
    ///    ])
    /// ]);
    ///
    /// let expected = Value::Array(vec![
    ///     Value::Nil,
    ///     Value::from(42),
    ///     Value::Array(vec![
    ///         Value::String("le message".into())
    ///     ])
    /// ]);
    ///
    /// assert_eq!(expected, val.to_owned());
    /// ```
    pub fn to_owned(&self) -> Value {
        match self {
            &ValueRef::Nil => Value::Nil,
            &ValueRef::Boolean(val) => Value::Boolean(val),
            &ValueRef::Integer(val) => Value::Integer(val),
            &ValueRef::F32(val) => Value::F32(val),
            &ValueRef::F64(val) => Value::F64(val),
            &ValueRef::String(val) => Value::from(val),
            &ValueRef::Binary(val) => Value::Binary(val.to_vec()),
            &ValueRef::Array(ref val) => {
                Value::Array(val.iter().map(|v| v.to_owned()).collect())
            }
            &ValueRef::Map(ref val) => {
                Value::Map(val.iter().map(|&(ref k, ref v)| (k.to_owned(), v.to_owned())).collect())
            }
            &ValueRef::Ext(ty, buf) => Value::Ext(ty, buf.to_vec()),
        }
    }

    pub fn index(&self, index: usize) -> &ValueRef {
        self.as_array().and_then(|v| v.get(index)).unwrap_or(&NIL_REF)
    }

    /// If the `ValueRef` is an integer, return or cast it to a u64.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::ValueRef;
    ///
    /// assert_eq!(Some(42), ValueRef::from(42).as_u64());
    /// ```
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            ValueRef::Integer(ref n) => n.as_u64(),
            _ => None,
        }
    }

    /// If the `ValueRef` is an Array, returns the associated vector.
    /// Returns None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rmpv::ValueRef;
    ///
    /// let val = ValueRef::Array(vec![ValueRef::Nil, ValueRef::Boolean(true)]);
    ///
    /// assert_eq!(Some(&vec![ValueRef::Nil, ValueRef::Boolean(true)]), val.as_array());
    /// assert_eq!(None, ValueRef::Nil.as_array());
    /// ```
    pub fn as_array(&self) -> Option<&Vec<ValueRef>> {
        if let ValueRef::Array(ref array) = *self {
            Some(&*array)
        } else {
            None
        }
    }

    pub fn into_array(self) -> Option<Vec<ValueRef<'a>>> {
        if let ValueRef::Array(array) = self {
            Some(array)
        } else {
            None
        }
    }
}

impl<'a> From<u8> for ValueRef<'a> {
    fn from(v: u8) -> Self {
        ValueRef::Integer(From::from(v))
    }
}

impl<'a> From<u16> for ValueRef<'a> {
    fn from(v: u16) -> Self {
        ValueRef::Integer(From::from(v))
    }
}

impl<'a> From<u32> for ValueRef<'a> {
    fn from(v: u32) -> Self {
        ValueRef::Integer(From::from(v))
    }
}

impl<'a> From<u64> for ValueRef<'a> {
    fn from(v: u64) -> Self {
        ValueRef::Integer(From::from(v))
    }
}

impl<'a> From<usize> for ValueRef<'a> {
    fn from(v: usize) -> Self {
        ValueRef::Integer(From::from(v))
    }
}

impl<'a> From<i8> for ValueRef<'a> {
    fn from(v: i8) -> Self {
        ValueRef::Integer(From::from(v))
    }
}

impl<'a> From<i16> for ValueRef<'a> {
    fn from(v: i16) -> Self {
        ValueRef::Integer(From::from(v))
    }
}

impl<'a> From<i32> for ValueRef<'a> {
    fn from(v: i32) -> Self {
        ValueRef::Integer(From::from(v))
    }
}

impl<'a> From<i64> for ValueRef<'a> {
    fn from(v: i64) -> Self {
        ValueRef::Integer(From::from(v))
    }
}

impl<'a> From<isize> for ValueRef<'a> {
    fn from(v: isize) -> Self {
        ValueRef::Integer(From::from(v))
    }
}

impl<'a> Display for ValueRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ValueRef::Nil => write!(f, "nil"),
            ValueRef::Boolean(val) => write!(f, "{}", val),
            ValueRef::Integer(ref val) => write!(f, "{}", val),
            ValueRef::F32(val) => write!(f, "{}", val),
            ValueRef::F64(val) => write!(f, "{}", val),
            ValueRef::String(ref val) => write!(f, "\"{}\"", val),
            ValueRef::Binary(ref val) => write!(f, "{:?}", val),
            ValueRef::Array(ref vec) => {
                let res = vec.iter()
                    .map(|val| format!("{}", val))
                    .collect::<Vec<String>>()
                    .join(", ");

                write!(f, "[{}]", res)
            }
            ValueRef::Map(ref vec) => {
                try!(write!(f, "{{"));

                match vec.iter().take(1).next() {
                    Some(&(ref k, ref v)) => {
                        try!(write!(f, "{}: {}", k, v));
                    }
                    None => {
                        try!(write!(f, ""));
                    }
                }

                for &(ref k, ref v) in vec.iter().skip(1) {
                    try!(write!(f, ", {}: {}", k, v));
                }

                write!(f, "}}")
            }
            ValueRef::Ext(ty, ref data) => {
                write!(f, "[{}, {:?}]", ty, data)
            }
        }
    }
}
