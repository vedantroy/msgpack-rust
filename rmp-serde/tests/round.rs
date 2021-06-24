#[macro_use]
extern crate serde_derive;

use rmp_serde as rmps;
use rmps::config::{DefaultConfig, SerializerConfig};
use rmps::decode::ReadReader;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::io::Cursor;

#[test]
fn round_trip_option() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Foo {
        v: Option<Vec<u8>>,
    }

    let expected = Foo { v: None };

    let mut buf = Vec::new();
    expected.serialize(&mut Serializer::new(&mut buf)).unwrap();

    let mut de = Deserializer::new(Cursor::new(&buf[..]));

    assert_eq!(expected, Deserialize::deserialize(&mut de).unwrap());
}

#[test]
fn round_trip_nested_option() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Struct {
        f1: Option<Option<u32>>,
        f2: Option<Option<u32>>,
    }

    let expected = Struct {
        f1: Some(Some(13)),
        f2: None,
    };

    let mut buf = Vec::new();
    expected.serialize(&mut Serializer::new(&mut buf)).unwrap();

    let mut de = Deserializer::new(Cursor::new(&buf[..]));

    assert_eq!(expected, Deserialize::deserialize(&mut de).unwrap());
}

#[test]
fn round_trip_optional_enum() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum SimpleEnum {
        Variant,
    }
    let expected = Some(SimpleEnum::Variant);

    let mut buf = Vec::new();
    expected.serialize(&mut Serializer::new(&mut buf)).unwrap();

    let mut de = Deserializer::new(Cursor::new(&buf[..]));
    assert_eq!(expected, Deserialize::deserialize(&mut de).unwrap());
}

#[test]
fn round_trip_cow() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Foo<'a> {
        v: Cow<'a, [u8]>,
    }

    let expected = Foo {
        v: Cow::Borrowed(&[]),
    };

    let mut buf = Vec::new();
    expected.serialize(&mut Serializer::new(&mut buf)).unwrap();

    let mut de = Deserializer::new(Cursor::new(&buf[..]));

    assert_eq!(expected, Deserialize::deserialize(&mut de).unwrap());
}

#[test]
fn round_trip_option_cow() {
    use serde::Serialize;
    use std::borrow::Cow;
    use std::io::Cursor;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Foo<'a> {
        v: Option<Cow<'a, [u8]>>,
    }

    let expected = Foo { v: None };

    let mut buf = Vec::new();
    expected.serialize(&mut Serializer::new(&mut buf)).unwrap();

    let mut de = Deserializer::new(Cursor::new(&buf[..]));

    assert_eq!(expected, Deserialize::deserialize(&mut de).unwrap());
}

#[test]
fn round_struct_like_enum() {
    use serde::Serialize;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum Enum {
        A { data: u32 },
    }

    let expected = Enum::A { data: 42 };
    let mut buf = Vec::new();
    expected.serialize(&mut Serializer::new(&mut buf)).unwrap();

    let mut de = Deserializer::new(&buf[..]);

    assert_eq!(expected, Deserialize::deserialize(&mut de).unwrap());
}

#[test]
fn round_struct_like_enum_with_struct_map() {
    use serde::Serialize;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum Enum {
        A { data: u32 },
    }

    let expected = Enum::A { data: 42 };
    let mut buf = Vec::new();
    expected
        .serialize(&mut Serializer::new(&mut buf).with_struct_map())
        .unwrap();

    let mut de = Deserializer::new(&buf[..]);

    assert_eq!(expected, Deserialize::deserialize(&mut de).unwrap());
}

#[test]
fn round_struct_like_enum_with_struct_tuple() {
    use serde::Serialize;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum Enum {
        A { data: u32 },
    }

    let expected = Enum::A { data: 42 };
    let mut buf = Vec::new();
    expected
        .serialize(&mut Serializer::new(&mut buf).with_struct_tuple())
        .unwrap();

    let mut de = Deserializer::new(&buf[..]);

    assert_eq!(expected, Deserialize::deserialize(&mut de).unwrap());
}

#[test]
fn round_enum_with_newtype_struct() {
    use serde::Serialize;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Newtype(String);

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum Enum {
        A(Newtype),
    }

    let expected = Enum::A(Newtype("le message".into()));
    let mut buf = Vec::new();
    expected.serialize(&mut Serializer::new(&mut buf)).unwrap();

    let mut de = Deserializer::new(&buf[..]);

    assert_eq!(expected, Deserialize::deserialize(&mut de).unwrap());
}

#[test]
fn round_trip_untagged_enum_with_enum_associated_data() {
    //#[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[derive(Debug, PartialEq)]
    //#[serde(untagged)]
    enum Foo {
        A(Bar),
    }

    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        //macro_rules! try {
        //    ($ __expr : expr) =>
        //    {
        //        match $ __expr
        //        {
        //            _serde :: __private :: Ok(__val) => __val, _serde ::
        //            __private :: Err(__err) =>
        //            { return _serde :: __private :: Err(__err) ; }
        //        }
        //    }
        //}
        #[automatically_derived]
        impl _serde::Serialize for Foo {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Foo::A(ref __field0) => _serde::Serialize::serialize(__field0, __serializer),
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        //macro_rules! try {
        //    ($ __expr : expr) =>
        //    {
        //        match $ __expr
        //        {
        //            _serde :: __private :: Ok(__val) => __val, _serde ::
        //            __private :: Err(__err) =>
        //            { return _serde :: __private :: Err(__err) ; }
        //        }
        //    }
        //}
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Foo {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let __content =
                    match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                        __deserializer,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
                    <Bar as _serde::Deserialize>::deserialize(
                        _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                            &__content,
                        ),
                    ),
                    Foo::A,
                ) {
                    return _serde::__private::Ok(__ok);
                }
                _serde::__private::Err(_serde::de::Error::custom(
                    "data did not match any variant of untagged enum Foo",
                ))
            }
        }
    };

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    //#[derive(Debug, PartialEq)]
    enum Bar {
        B,
        C(String),
        D(u64, u64, u64),
        E { f1: String },
    }

    /*
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            #[allow(unused_extern_crates, clippy :: useless_attribute)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            //macro_rules! try {
            //    ($ __expr : expr) =>
            //    {
            //        match $ __expr
            //        {
            //            _serde :: __private :: Ok(__val) => __val, _serde ::
            //            __private :: Err(__err) =>
            //            { return _serde :: __private :: Err(__err) ; }
            //        }
            //    }
            //}
            #[automatically_derived]
            impl _serde::Serialize for Bar {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::__private::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    match *self {
                        Bar::B =>
                        _serde::Serializer::serialize_unit_variant(__serializer,
                                                                   "Bar",
                                                                   0u32, "B"),
                        Bar::C(ref __field0) =>
                        _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                      "Bar",
                                                                      1u32,
                                                                      "C",
                                                                      __field0),
                        Bar::D(ref __field0, ref __field1, ref __field2) => {
                            let mut __serde_state =
                                match _serde::Serializer::serialize_tuple_variant(__serializer,
                                                                                  "Bar",
                                                                                  2u32,
                                                                                  "D",
                                                                                  0
                                                                                      +
                                                                                      1
                                                                                      +
                                                                                      1
                                                                                      +
                                                                                      1)
                                    {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            match _serde::ser::SerializeTupleVariant::serialize_field(&mut __serde_state,
                                                                                      __field0)
                                {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            match _serde::ser::SerializeTupleVariant::serialize_field(&mut __serde_state,
                                                                                      __field1)
                                {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            match _serde::ser::SerializeTupleVariant::serialize_field(&mut __serde_state,
                                                                                      __field2)
                                {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::ser::SerializeTupleVariant::end(__serde_state)
                        }
                        Bar::E { ref f1 } => {
                            let mut __serde_state =
                                match _serde::Serializer::serialize_struct_variant(__serializer,
                                                                                   "Bar",
                                                                                   3u32,
                                                                                   "E",
                                                                                   0
                                                                                       +
                                                                                       1)
                                    {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            match _serde::ser::SerializeStructVariant::serialize_field(&mut __serde_state,
                                                                                       "f1",
                                                                                       f1)
                                {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::ser::SerializeStructVariant::end(__serde_state)
                        }
                    }
                }
            }
        };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () =
        {
            #[allow(unused_extern_crates, clippy :: useless_attribute)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            //macro_rules! try {
            //    ($ __expr : expr) =>
            //    {
            //        match $ __expr
            //        {
            //            _serde :: __private :: Ok(__val) => __val, _serde ::
            //            __private :: Err(__err) =>
            //            { return _serde :: __private :: Err(__err) ; }
            //        }
            //    }
            //}
            #[automatically_derived]
            impl <'de> _serde::Deserialize<'de> for Bar {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::__private::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __field3, }
                    struct __FieldVisitor;
                    impl <'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::__private::Formatter)
                         -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                                                    "variant identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                         -> _serde::__private::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                0u64 =>
                                _serde::__private::Ok(__Field::__field0),
                                1u64 =>
                                _serde::__private::Ok(__Field::__field1),
                                2u64 =>
                                _serde::__private::Ok(__Field::__field2),
                                3u64 =>
                                _serde::__private::Ok(__Field::__field3),
                                _ =>
                                _serde::__private::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                        &"variant index 0 <= i < 4")),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                         -> _serde::__private::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                "B" =>
                                _serde::__private::Ok(__Field::__field0),
                                "C" =>
                                _serde::__private::Ok(__Field::__field1),
                                "D" =>
                                _serde::__private::Ok(__Field::__field2),
                                "E" =>
                                _serde::__private::Ok(__Field::__field3),
                                _ => {
                                    _serde::__private::Err(_serde::de::Error::unknown_variant(__value,
                                                                                              VARIANTS))
                                }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                         -> _serde::__private::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                b"B" =>
                                _serde::__private::Ok(__Field::__field0),
                                b"C" =>
                                _serde::__private::Ok(__Field::__field1),
                                b"D" =>
                                _serde::__private::Ok(__Field::__field2),
                                b"E" =>
                                _serde::__private::Ok(__Field::__field3),
                                _ => {
                                    let __value =
                                        &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(_serde::de::Error::unknown_variant(__value,
                                                                                              VARIANTS))
                                }
                            }
                        }
                    }
                    impl <'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D)
                         -> _serde::__private::Result<Self, __D::Error> where
                         __D: _serde::Deserializer<'de> {
                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                         __FieldVisitor)
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Bar>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Bar;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::__private::Formatter)
                         -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter,
                                                                    "enum Bar")
                        }
                        fn visit_enum<__A>(self, __data: __A)
                         -> _serde::__private::Result<Self::Value, __A::Error>
                         where __A: _serde::de::EnumAccess<'de> {
                            match match _serde::de::EnumAccess::variant(__data)
                                      {
                                      _serde::__private::Ok(__val) => __val,
                                      _serde::__private::Err(__err) => {
                                          return _serde::__private::Err(__err);
                                      }
                                  } {
                                (__Field::__field0, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant)
                                        {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                    _serde::__private::Ok(Bar::B)
                                }
                                (__Field::__field1, __variant) =>
                                _serde::__private::Result::map(_serde::de::VariantAccess::newtype_variant::<String>(__variant),
                                                               Bar::C),
                                (__Field::__field2, __variant) => {
                                    struct __Visitor<'de> {
                                        marker: _serde::__private::PhantomData<Bar>,
                                        lifetime: _serde::__private::PhantomData<&'de ()>,
                                    }
                                    impl <'de> _serde::de::Visitor<'de> for
                                     __Visitor<'de> {
                                        type Value = Bar;
                                        fn expecting(&self,
                                                     __formatter:
                                                         &mut _serde::__private::Formatter)
                                         -> _serde::__private::fmt::Result {
                                            _serde::__private::Formatter::write_str(__formatter,
                                                                                    "tuple variant Bar::D")
                                        }
                                        #[inline]
                                        fn visit_seq<__A>(self,
                                                          mut __seq: __A)
                                         ->
                                             _serde::__private::Result<Self::Value,
                                                                       __A::Error>
                                         where
                                         __A: _serde::de::SeqAccess<'de> {
                                            let __field0 =
                                                match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq)
                                                          {
                                                          _serde::__private::Ok(__val)
                                                          => __val,
                                                          _serde::__private::Err(__err)
                                                          => {
                                                              return _serde::__private::Err(__err);
                                                          }
                                                      } {
                                                    _serde::__private::Some(__value)
                                                    => __value,
                                                    _serde::__private::None =>
                                                    {
                                                        return _serde::__private::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                                        &"tuple variant Bar::D with 3 elements"));
                                                    }
                                                };
                                            let __field1 =
                                                match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq)
                                                          {
                                                          _serde::__private::Ok(__val)
                                                          => __val,
                                                          _serde::__private::Err(__err)
                                                          => {
                                                              return _serde::__private::Err(__err);
                                                          }
                                                      } {
                                                    _serde::__private::Some(__value)
                                                    => __value,
                                                    _serde::__private::None =>
                                                    {
                                                        return _serde::__private::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                                        &"tuple variant Bar::D with 3 elements"));
                                                    }
                                                };
                                            let __field2 =
                                                match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq)
                                                          {
                                                          _serde::__private::Ok(__val)
                                                          => __val,
                                                          _serde::__private::Err(__err)
                                                          => {
                                                              return _serde::__private::Err(__err);
                                                          }
                                                      } {
                                                    _serde::__private::Some(__value)
                                                    => __value,
                                                    _serde::__private::None =>
                                                    {
                                                        return _serde::__private::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                                        &"tuple variant Bar::D with 3 elements"));
                                                    }
                                                };
                                            _serde::__private::Ok(Bar::D(__field0,
                                                                         __field1,
                                                                         __field2))
                                        }
                                    }
                                    _serde::de::VariantAccess::tuple_variant(__variant,
                                                                             3usize,
                                                                             __Visitor{marker:
                                                                                           _serde::__private::PhantomData::<Bar>,
                                                                                       lifetime:
                                                                                           _serde::__private::PhantomData,})
                                }
                                (__Field::__field3, __variant) => {
                                    #[allow(non_camel_case_types)]
                                    enum __Field { __field0, __ignore, }
                                    struct __FieldVisitor;
                                    impl <'de> _serde::de::Visitor<'de> for
                                     __FieldVisitor {
                                        type Value = __Field;
                                        fn expecting(&self,
                                                     __formatter:
                                                         &mut _serde::__private::Formatter)
                                         -> _serde::__private::fmt::Result {
                                            _serde::__private::Formatter::write_str(__formatter,
                                                                                    "field identifier")
                                        }
                                        fn visit_u64<__E>(self, __value: u64)
                                         ->
                                             _serde::__private::Result<Self::Value,
                                                                       __E>
                                         where __E: _serde::de::Error {
                                            match __value {
                                                0u64 =>
                                                _serde::__private::Ok(__Field::__field0),
                                                _ =>
                                                _serde::__private::Ok(__Field::__ignore),
                                            }
                                        }
                                        fn visit_str<__E>(self, __value: &str)
                                         ->
                                             _serde::__private::Result<Self::Value,
                                                                       __E>
                                         where __E: _serde::de::Error {
                                            match __value {
                                                "f1" =>
                                                _serde::__private::Ok(__Field::__field0),
                                                _ => {
                                                    _serde::__private::Ok(__Field::__ignore)
                                                }
                                            }
                                        }
                                        fn visit_bytes<__E>(self,
                                                            __value: &[u8])
                                         ->
                                             _serde::__private::Result<Self::Value,
                                                                       __E>
                                         where __E: _serde::de::Error {
                                            match __value {
                                                b"f1" =>
                                                _serde::__private::Ok(__Field::__field0),
                                                _ => {
                                                    _serde::__private::Ok(__Field::__ignore)
                                                }
                                            }
                                        }
                                    }
                                    impl <'de> _serde::Deserialize<'de> for
                                     __Field {
                                        #[inline]
                                        fn deserialize<__D>(__deserializer:
                                                                __D)
                                         ->
                                             _serde::__private::Result<Self,
                                                                       __D::Error>
                                         where
                                         __D: _serde::Deserializer<'de> {
                                            _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                                         __FieldVisitor)
                                        }
                                    }
                                    struct __Visitor<'de> {
                                        marker: _serde::__private::PhantomData<Bar>,
                                        lifetime: _serde::__private::PhantomData<&'de ()>,
                                    }
                                    impl <'de> _serde::de::Visitor<'de> for
                                     __Visitor<'de> {
                                        type Value = Bar;
                                        fn expecting(&self,
                                                     __formatter:
                                                         &mut _serde::__private::Formatter)
                                         -> _serde::__private::fmt::Result {
                                            _serde::__private::Formatter::write_str(__formatter,
                                                                                    "struct variant Bar::E")
                                        }
                                        #[inline]
                                        fn visit_seq<__A>(self,
                                                          mut __seq: __A)
                                         ->
                                             _serde::__private::Result<Self::Value,
                                                                       __A::Error>
                                         where
                                         __A: _serde::de::SeqAccess<'de> {
                                            let __field0 =
                                                match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                                          {
                                                          _serde::__private::Ok(__val)
                                                          => __val,
                                                          _serde::__private::Err(__err)
                                                          => {
                                                              return _serde::__private::Err(__err);
                                                          }
                                                      } {
                                                    _serde::__private::Some(__value)
                                                    => __value,
                                                    _serde::__private::None =>
                                                    {
                                                        return _serde::__private::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                                        &"struct variant Bar::E with 1 element"));
                                                    }
                                                };
                                            _serde::__private::Ok(Bar::E{f1:
                                                                             __field0,})
                                        }
                                        #[inline]
                                        fn visit_map<__A>(self,
                                                          mut __map: __A)
                                         ->
                                             _serde::__private::Result<Self::Value,
                                                                       __A::Error>
                                         where
                                         __A: _serde::de::MapAccess<'de> {
                                            let mut __field0:
                                                    _serde::__private::Option<String> =
                                                _serde::__private::None;
                                            while let _serde::__private::Some(__key)
                                                      =
                                                      match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                                          {
                                                          _serde::__private::Ok(__val)
                                                          => __val,
                                                          _serde::__private::Err(__err)
                                                          => {
                                                              return _serde::__private::Err(__err);
                                                          }
                                                      } {
                                                match __key {
                                                    __Field::__field0 => {
                                                        if _serde::__private::Option::is_some(&__field0)
                                                           {
                                                            return _serde::__private::Err(<__A::Error
                                                                                              as
                                                                                              _serde::de::Error>::duplicate_field("f1"));
                                                        }
                                                        __field0 =
                                                            _serde::__private::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                                                        {
                                                                                        _serde::__private::Ok(__val)
                                                                                        =>
                                                                                        __val,
                                                                                        _serde::__private::Err(__err)
                                                                                        =>
                                                                                        {
                                                                                            return _serde::__private::Err(__err);
                                                                                        }
                                                                                    });
                                                    }
                                                    _ => {
                                                        let _ =
                                                            match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                                                {
                                                                _serde::__private::Ok(__val)
                                                                => __val,
                                                                _serde::__private::Err(__err)
                                                                => {
                                                                    return _serde::__private::Err(__err);
                                                                }
                                                            };
                                                    }
                                                }
                                            }
                                            let __field0 =
                                                match __field0 {
                                                    _serde::__private::Some(__field0)
                                                    => __field0,
                                                    _serde::__private::None =>
                                                    match _serde::__private::de::missing_field("f1")
                                                        {
                                                        _serde::__private::Ok(__val)
                                                        => __val,
                                                        _serde::__private::Err(__err)
                                                        => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    },
                                                };
                                            _serde::__private::Ok(Bar::E{f1:
                                                                             __field0,})
                                        }
                                    }
                                    const FIELDS: &'static [&'static str] =
                                        &["f1"];
                                    _serde::de::VariantAccess::struct_variant(__variant,
                                                                              FIELDS,
                                                                              __Visitor{marker:
                                                                                            _serde::__private::PhantomData::<Bar>,
                                                                                        lifetime:
                                                                                            _serde::__private::PhantomData,})
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] =
                        &["B", "C", "D", "E"];
                    _serde::Deserializer::deserialize_enum(__deserializer,
                                                           "Bar", VARIANTS,
                                                           __Visitor{marker:
                                                                         _serde::__private::PhantomData::<Bar>,
                                                                     lifetime:
                                                                         _serde::__private::PhantomData,})
                }
            }
        };
        */

    let data1_1 = Foo::A(Bar::B);
    let bytes_1 = rmps::to_vec(&data1_1).unwrap();
    println!("bytes: {:?}", bytes_1);
    let data1_2 = rmps::from_slice(&bytes_1).unwrap();
    assert_eq!(data1_1, data1_2);

    let data2_1 = Foo::A(Bar::C("Hello".into()));
    let bytes_2 = rmps::to_vec(&data2_1).unwrap();
    let data2_2 = rmps::from_slice(&bytes_2).unwrap();
    assert_eq!(data2_1, data2_2);

    let data3_1 = Foo::A(Bar::D(1, 2, 3));
    let bytes_3 = rmps::to_vec(&data3_1).unwrap();
    let data3_2 = rmps::from_slice(&bytes_3).unwrap();
    assert_eq!(data3_1, data3_2);

    let data4_1 = Foo::A(Bar::E { f1: "Hello".into() });
    let bytes_4 = rmps::to_vec(&data4_1).unwrap();
    let data4_2 = rmps::from_slice(&bytes_4).unwrap();
    assert_eq!(data4_1, data4_2);
}

// Checks whether deserialization and serialization can both work with structs as maps
#[test]
fn round_struct_as_map() {
    use crate::rmps::decode::from_slice;
    use crate::rmps::to_vec_named;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Dog1 {
        name: String,
        age: u16,
    }
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Dog2 {
        age: u16,
        name: String,
    }

    let dog1 = Dog1 {
        name: "Frankie".into(),
        age: 42,
    };

    let serialized: Vec<u8> = to_vec_named(&dog1).unwrap();
    let deserialized: Dog2 = from_slice(&serialized).unwrap();

    let check = Dog1 {
        age: deserialized.age,
        name: deserialized.name,
    };

    assert_eq!(dog1, check);
}

#[test]
fn round_struct_as_map_in_vec() {
    // See: issue #205
    use crate::rmps::decode::from_slice;
    use crate::rmps::to_vec_named;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Dog1 {
        name: String,
        age: u16,
    }
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Dog2 {
        age: u16,
        name: String,
    }

    let dog1 = Dog1 {
        name: "Frankie".into(),
        age: 42,
    };

    let data = vec![dog1];

    let serialized: Vec<u8> = to_vec_named(&data).unwrap();
    let deserialized: Vec<Dog2> = from_slice(&serialized).unwrap();

    let dog2 = &deserialized[0];

    assert_eq!(dog2.name, "Frankie");
    assert_eq!(dog2.age, 42);
}

#[test]
fn round_trip_unit_struct() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Message1 {
        data: u8,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Message2;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    enum Messages {
        Message1(Message1),
        Message2(Message2),
    }

    let msg2 = Messages::Message2(Message2);

    // struct-as-tuple
    {
        let serialized: Vec<u8> = rmps::to_vec(&msg2).unwrap();
        let deserialized: Messages = rmps::from_slice(&serialized).unwrap();
        assert_eq!(deserialized, msg2);
    }

    // struct-as-map
    {
        let serialized: Vec<u8> = rmps::to_vec_named(&msg2).unwrap();
        let deserialized: Messages = rmps::from_slice(&serialized).unwrap();
        assert_eq!(deserialized, msg2);
    }
}

#[test]
#[ignore]
fn round_trip_unit_struct_untagged_enum() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct UnitStruct;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct MessageA {
        some_int: i32,
        unit: UnitStruct,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(untagged)]
    enum Messages {
        MessageA(MessageA),
    }

    let msga = Messages::MessageA(MessageA {
        some_int: 32,
        unit: UnitStruct,
    });

    // struct-as-tuple
    {
        let serialized: Vec<u8> = rmps::to_vec(&msga).unwrap();
        let deserialized: Messages = rmps::from_slice(&serialized).unwrap();
        assert_eq!(deserialized, msga);
    }

    // struct-as-map
    {
        let serialized: Vec<u8> = rmps::to_vec_named(&msga).unwrap();
        let deserialized: Messages = rmps::from_slice(&serialized).unwrap();
        assert_eq!(deserialized, msga);
    }
}

#[test]
fn round_trip_struct_with_flattened_map_field() {
    use std::collections::BTreeMap;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Struct {
        f1: u32,
        // not flattend!
        f2: BTreeMap<String, String>,
        #[serde(flatten)]
        f3: BTreeMap<String, String>,
    }

    let strct = Struct {
        f1: 0,
        f2: {
            let mut map = BTreeMap::new();
            map.insert("german".to_string(), "Hallo Welt!".to_string());
            map
        },
        f3: {
            let mut map = BTreeMap::new();
            map.insert("english".to_string(), "Hello World!".to_string());
            map
        },
    };

    let serialized: Vec<u8> = rmps::to_vec(&strct).unwrap();
    let deserialized: Struct = rmps::from_slice(&serialized).unwrap();
    assert_eq!(deserialized, strct);
}

#[test]
fn round_trip_struct_with_flattened_struct_field() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Struct {
        f1: u32,
        // not flattend!
        f2: InnerStruct,
        #[serde(flatten)]
        f3: InnerStruct,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct InnerStruct {
        f4: u32,
        f5: u32,
    }

    let strct = Struct {
        f1: 0,
        f2: InnerStruct { f4: 8, f5: 13 },
        f3: InnerStruct { f4: 21, f5: 34 },
    };

    // struct-as-tuple
    {
        let serialized: Vec<u8> = rmps::to_vec(&strct).unwrap();
        let deserialized: Struct = rmps::from_slice(&serialized).unwrap();
        assert_eq!(deserialized, strct);
    }

    // struct-as-map
    {
        let serialized: Vec<u8> = rmps::to_vec_named(&strct).unwrap();
        let deserialized: Struct = rmps::from_slice(&serialized).unwrap();
        assert_eq!(deserialized, strct);
    }
}

// Checks whether deserialization and serialization can both work with enum variants as strings
#[test]
fn round_variant_string() {
    use crate::rmps::decode::from_slice;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    enum Animal1 {
        Dog { breed: String },
        Cat,
        Emu,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    enum Animal2 {
        Emu,
        Dog { breed: String },
        Cat,
    }

    // use helper macro so that we can test many combinations at once. Needs to be a macro to deal
    // with the serializer owning a reference to the Vec.
    macro_rules! do_test {
        ($ser:expr) => {{
            let animal1 = Animal1::Dog {
                breed: "Pitbull".to_owned(),
            };
            let expected = Animal2::Dog {
                breed: "Pitbull".to_owned(),
            };
            let mut buf = Vec::new();
            animal1.serialize(&mut $ser(&mut buf)).unwrap();

            let deserialized: Animal2 = from_slice(&buf).unwrap();
            assert_eq!(deserialized, expected);
        }};
    }

    do_test!(|b| Serializer::new(b).with_string_variants());
    do_test!(|b| Serializer::new(b).with_struct_map().with_string_variants());
    do_test!(|b| Serializer::new(b)
        .with_struct_tuple()
        .with_string_variants());
    do_test!(|b| Serializer::new(b).with_string_variants().with_struct_map());
    do_test!(|b| Serializer::new(b)
        .with_string_variants()
        .with_struct_tuple());
    do_test!(|b| {
        Serializer::new(b)
            .with_string_variants()
            .with_struct_tuple()
            .with_struct_map()
            .with_struct_tuple()
            .with_struct_map()
    });
    do_test!(|b| Serializer::new(b)
        .with_integer_variants()
        .with_string_variants());
}

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[test]
fn roundtrip_ipv4addr() {
    assert_roundtrips(Ipv4Addr::new(127, 0, 0, 1));
}

#[test]
fn roundtrip_ipv6addr() {
    assert_roundtrips(Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8));
}

#[test]
fn roundtrip_ipaddr_ipv4addr() {
    assert_roundtrips(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
}

#[test]
fn roundtrip_ipaddr_ipv6addr() {
    assert_roundtrips(IpAddr::V6(Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8)));
}

#[test]
fn roundtrip_result_ipv4addr() {
    let val: Result<Ipv4Addr, ()> = Ok(Ipv4Addr::new(127, 0, 0, 1));
    assert_roundtrips(val);
}

#[test]
fn roundtrip_result_num() {
    assert_roundtrips(Ok::<u32, u32>(42));
    assert_roundtrips(Err::<(), _>(222));
}

#[test]
fn roundtrip_simple_enum() {
    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    enum SimpleEnum {
        V1(u32),
        V2(String),
    }

    assert_roundtrips(SimpleEnum::V1(42));
    assert_roundtrips(SimpleEnum::V2("hello".into()));
}

#[test]
fn roundtrip_some() {
    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    struct Wrapper<T>(T);

    assert_roundtrips(Some(99));
    assert_roundtrips(Wrapper(Some(99)));
    assert_roundtrips(Some(Wrapper(99)));
    assert_roundtrips(Some("hi".to_string()));
}

#[ignore]
#[test]
fn roundtrip_some_failures() {
    // FIXME
    assert_roundtrips(Some(None::<()>));
}

#[cfg(test)]
fn assert_roundtrips<T: PartialEq + std::fmt::Debug + Serialize + for<'a> Deserialize<'a>>(val: T) {
    assert_roundtrips_config(&val, "default", |s| s, |d| d);
    assert_roundtrips_config(&val, ".with_struct_map()", |s| s.with_struct_map(), |d| d);
    assert_roundtrips_config(
        &val,
        ".with_string_variants()",
        |s| s.with_string_variants(),
        |d| d,
    );
    assert_roundtrips_config(
        &val,
        ".with_struct_map().with_string_variants()",
        |s| s.with_struct_map().with_string_variants(),
        |d| d,
    );
    assert_roundtrips_config(
        &val,
        ".with_human_readable()",
        |s| s.with_human_readable(),
        |d| d.with_human_readable(),
    );
    assert_roundtrips_config(
        &val,
        ".with_human_readable().with_struct_map()",
        |s| s.with_human_readable().with_struct_map(),
        |d| d.with_human_readable(),
    );
    assert_roundtrips_config(
        &val,
        ".with_human_readable().with_string_variants()",
        |s| s.with_human_readable().with_string_variants(),
        |d| d.with_human_readable(),
    );
    assert_roundtrips_config(
        &val,
        ".with_human_readable().with_struct_map().with_string_variants()",
        |s| {
            s.with_human_readable()
                .with_struct_map()
                .with_string_variants()
        },
        |d| d.with_human_readable(),
    );
}

#[cfg(test)]
fn assert_roundtrips_config<T, CSF, SC, CDF, DC>(
    val: &T,
    desc: &str,
    config_serializer: CSF,
    config_deserializer: CDF,
) where
    T: PartialEq + std::fmt::Debug + Serialize + for<'a> Deserialize<'a>,
    CSF: FnOnce(Serializer<Vec<u8>, DefaultConfig>) -> Serializer<Vec<u8>, SC>,
    SC: SerializerConfig,
    CDF: FnOnce(
        Deserializer<ReadReader<&[u8]>, DefaultConfig>,
    ) -> Deserializer<ReadReader<&[u8]>, DC>,
    DC: SerializerConfig,
{
    let mut serializer = config_serializer(Serializer::new(Vec::new()));
    if let Err(e) = val.serialize(&mut serializer) {
        panic!(
            "Failed to serialize: {}\nConfig: {}\nValue: {:?}\n",
            e, desc, val
        );
    }
    let serialized = serializer.into_inner();

    let mut deserializer = config_deserializer(Deserializer::new(serialized.as_slice()));
    let val2: T = match T::deserialize(&mut deserializer) {
        Ok(t) => t,
        Err(e) => {
            panic!(
                "Does not deserialize: {}\nConfig: {}\nSerialized {:?}\nGot {:?}\n",
                e,
                desc,
                val,
                rmpv::decode::value::read_value(&mut serialized.as_slice())
                    .expect("rmp didn't serialize correctly at all")
            );
        }
    };

    assert_eq!(val, &val2, "Config: {}", desc);
}
