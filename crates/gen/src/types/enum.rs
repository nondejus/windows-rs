use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Enum(pub TypeDef);

impl Enum {
    pub fn signature(&self) -> String {
        format!(
            "enum({}.{};{})",
            self.0.namespace(),
            self.0.name(),
            self.enum_type()
        )
    }

    fn enum_type(&self) -> &str {
        for field in self.0.fields() {
            if let Some(constant) = field.constant() {
                match constant.value_type() {
                    ElementType::I32 => return "i4",
                    ElementType::U32 => return "u4",
                    _ => unexpected!(),
                };
            }
        }

        unexpected!();
    }

    pub fn gen(&self, _: Gen) -> TokenStream {
        quote! {}
    }
}


// #[derive(Debug)]
// pub struct Enum {
//     pub name: TypeName,
//     pub fields: Vec<(&'static str, EnumConstant)>,
//     pub underlying_type: winmd::ElementType,
//     pub signature: String,
// }

// #[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
// pub enum EnumConstant {
//     U32(u32),
//     I32(i32),
// }

// impl EnumConstant {
//     fn next(&self) -> Self {
//         match self {
//             Self::U32(value) => Self::U32(value + 1),
//             Self::I32(value) => Self::I32(value + 1),
//         }
//     }
// }

// impl Enum {
//     pub fn from_type_name(name: TypeName) -> Self {
//         let signature = if name.def.is_winrt() {
//             name.enum_signature()
//         } else {
//             String::new()
//         };

//         let mut fields = Vec::new();
//         let mut underlying_type = None;

//         for field in name.def.fields() {
//             if field.flags().literal() {
//                 if let Some(constant) = field.constant() {
//                     let value = match constant.value() {
//                         winmd::ConstantValue::I32(value) => EnumConstant::I32(value),
//                         winmd::ConstantValue::U32(value) => EnumConstant::U32(value),
//                         _ => unexpected!(),
//                     };

//                     fields.push((field.name(), value));
//                 } else if fields.is_empty() {
//                     fields.push((field.name(), EnumConstant::I32(0)));
//                 } else {
//                     fields.push((field.name(), fields.last().unwrap().1.next()));
//                 }
//             } else {
//                 let blob = &mut field.blob();
//                 blob.read_unsigned();
//                 blob.read_modifiers();

//                 blob.read_expected(0x1D);
//                 blob.read_modifiers();

//                 underlying_type = Some(winmd::ElementType::from_blob(blob, &[]));
//             }
//         }
//         Self {
//             name,
//             fields,
//             underlying_type: underlying_type.expect("Enum.from_type_name"),
//             signature,
//         }
//     }

//     pub fn gen(&self) -> TokenStream {
//         let name = self.name.gen();

//         let (underlying_type, bitwise) = match self.underlying_type {
//             winmd::ElementType::I32 => (format_ident!("i32"), TokenStream::new()),
//             winmd::ElementType::U32 => (
//                 format_ident!("u32"),
//                 quote! {
//                     impl ::std::ops::BitOr for #name {
//                         type Output = Self;

//                         fn bitor(self, rhs: Self) -> Self {
//                             Self(self.0 | rhs.0)
//                         }
//                     }
//                     impl ::std::ops::BitAnd for #name {
//                         type Output = Self;

//                         fn bitand(self, rhs: Self) -> Self {
//                             Self(self.0 & rhs.0)
//                         }
//                     }
//                 },
//             ),
//             _ => unexpected!(),
//         };

//         let fields = self.fields.iter().map(|(name, value)| {
//             let name = to_ident(&name);
//             let value = match value {
//                 EnumConstant::U32(value) => quote! { #value },
//                 EnumConstant::I32(value) => quote! { #value },
//             };

//             quote! {
//                 pub const #name: Self = Self(#value);
//             }
//         });

//         let runtime_type = if self.signature.is_empty() {
//             TokenStream::new()
//         } else {
//             let signature = Literal::byte_string(&self.signature.as_bytes());

//             quote! {
//                 unsafe impl ::windows::RuntimeType for #name {
//                     type DefaultType = Self;
//                     const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(#signature);
//                 }
//             }
//         };

//         quote! {
//             #[allow(non_camel_case_types)]
//             #[derive(PartialEq, Eq)]
//             #[repr(transparent)]
//             pub struct #name(pub #underlying_type);
//             impl ::std::convert::From<#underlying_type> for #name {
//                 fn from(value: #underlying_type) -> Self {
//                     Self(value)
//                 }
//             }
//             impl ::std::clone::Clone for #name {
//                 fn clone(&self) -> Self {
//                     Self(self.0)
//                 }
//             }
//             impl ::std::default::Default for #name {
//                 fn default() -> Self {
//                     Self(0)
//                 }
//             }
//             impl ::std::fmt::Debug for #name {
//                 fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
//                     write!(f, "{:?}", self.0)
//                 }
//             }
//             impl ::std::marker::Copy for #name {}
//             impl #name {
//                 #![allow(non_upper_case_globals)]
//                 #(#fields)*
//             }
//             unsafe impl ::windows::Abi for #name {
//                 type Abi = Self;
//             }
//             #runtime_type
//             #bitwise
//         }
//     }
// }