pub use token_wrapper_handler_contract::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod token_wrapper_handler_contract {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("bridgeAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialResourceIDs"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes32[]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "initialContractAddresses",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address[]"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("_bridgeAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_bridgeAddress"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_contractAddressToResourceID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_contractAddressToResourceID",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_contractWhitelist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_contractWhitelist"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_resourceIDToContractAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_resourceIDToContractAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeProposal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("resourceID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("migrateBridge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("migrateBridge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newBridge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setResource"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setResource"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("resourceID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static TOKENWRAPPERHANDLERCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x0Cp8\x03\x80b\0\x0Cp\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x03 V[\x80Q\x82Q\x14b\0\0\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FTokenWrapperHandler: initialReso`D\x82\x01R\x7FurceIDs and initialContractAddre`d\x82\x01Rp\x0Enl\xAEd\r\x8C\xAD\xC4\r\xAD.m\xAC.\x8Cm`{\x1B`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16b\0\x01:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FTokenWrapperHandler: Bridge addr`D\x82\x01Rm\x06W72\x066\x16\xE2wB\x06&R\x03`\x94\x1B`d\x82\x01R`\x84\x01b\0\0\xC2V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x81U[\x82Q\x81\x10\x15b\0\x01\xBFWb\0\x01\xAA\x83\x82\x81Q\x81\x10b\0\x01yWb\0\x01yb\0\x03\xFBV[` \x02` \x01\x01Q\x83\x83\x81Q\x81\x10b\0\x01\x96Wb\0\x01\x96b\0\x03\xFBV[` \x02` \x01\x01Qb\0\x01\xC9` \x1B` \x1CV[\x80b\0\x01\xB6\x81b\0\x04\x11V[\x91PPb\0\x01VV[PPPPb\0\x049V[`\0\x82\x81R`\x01` \x81\x81R`@\x80\x84 \x80T`\x01`\x01`\xA0\x1B\x03\x90\x96\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x96\x16\x86\x17\x90U\x93\x83R`\x02\x81R\x83\x83 \x94\x90\x94U`\x03\x90\x93R \x80T`\xFF\x19\x16\x90\x91\x17\x90UV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x020W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x02vWb\0\x02vb\0\x025V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15b\0\x02\x9AWb\0\x02\x9Ab\0\x025V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12b\0\x02\xB6W`\0\x80\xFD[\x81Q` b\0\x02\xCFb\0\x02\xC9\x83b\0\x02~V[b\0\x02KV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\0\x02\xEFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0\x03\x15Wb\0\x03\x07\x81b\0\x02\x18V[\x83R\x91\x83\x01\x91\x83\x01b\0\x02\xF3V[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x036W`\0\x80\xFD[b\0\x03A\x84b\0\x02\x18V[` \x85\x81\x01Q\x91\x94P\x90`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x03aW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12b\0\x03vW`\0\x80\xFD[\x81Qb\0\x03\x87b\0\x02\xC9\x82b\0\x02~V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8A\x83\x11\x15b\0\x03\xA7W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15b\0\x03\xC7W\x84Q\x82R\x93\x85\x01\x93\x90\x85\x01\x90b\0\x03\xACV[`@\x8A\x01Q\x90\x97P\x94PPP\x80\x83\x11\x15b\0\x03\xE1W`\0\x80\xFD[PPb\0\x03\xF1\x86\x82\x87\x01b\0\x02\xA4V[\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01b\0\x042WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[a\x08'\x80b\0\x04I`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xC5L*\x11\x11a\0[W\x80c\xC5L*\x11\x14a\0\xFAW\x80c\xD7\xF5\xB3Y\x14a\x01#W\x80c\xE2H\xCF\xF2\x14a\x016W\x80c\xEC\x97\xD3\xB4\x14a\x01IW`\0\x80\xFD[\x80c1\x8C\x13n\x14a\0\x82W\x80c\x7Fy\xBE\xA8\x14a\0\xB2W\x80c\xB8\xFA76\x14a\0\xE5W[`\0\x80\xFD[`\0Ta\0\x95\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD5a\0\xC06`\x04a\x064V[`\x03` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xA9V[a\0\xF8a\0\xF36`\x04a\x06VV[a\x01wV[\0[a\0\x95a\x01\x086`\x04a\x06\x82V[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xF8a\x0116`\x04a\x064V[a\x01\xCFV[a\0\xF8a\x01D6`\x04a\x06\x9BV[a\x02fV[a\x01ia\x01W6`\x04a\x064V[`\x02` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xA9V[a\x01\x7Fa\x05\xA5V[`\0\x82\x81R`\x01` \x81\x81R`@\x80\x84 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x87\x16\x90\x81\x17\x90\x91U\x84R`\x02\x82R\x80\x84 \x86\x90U`\x03\x90\x91R\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90UPPV[a\x01\xD7a\x05\xA5V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FHandlerHelpers: Bridge address c`D\x82\x01Rh\x06\x16\xE2wB\x06&R\x03`\xBC\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x02na\x05\xA5V[`\0\x806\x81a\x02\x80` \x82\x87\x89a\x07\x17V[a\x02\x89\x91a\x07AV[\x93Pa\x02\x99`$` \x87\x89a\x07\x17V[a\x02\xA2\x91a\x07`V[\x92Pa\x02\xB1\x85`$\x81\x89a\x07\x17V[`\0\x89\x81R`\x01` R`@\x90 T\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x16\x80c\x01\xEA\x8D\xC7`\xE5\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x01a\x03\x93W`\0a\x02\xF6`\x04\x82\x86\x88a\x07\x17V[a\x02\xFF\x91a\x07`V[`\xE0\x1C\x90P`\0a\x03\x14`\x06`\x04\x87\x89a\x07\x17V[a\x03\x1D\x91a\x07\x90V[`@Qc\x06\x15r9`\xE5\x1B\x81R`\xF0\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x83\x01R\x91P`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xC2\xAEG \x90`D\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03tW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x88W=`\0\x80>=`\0\xFD[PPPPPPa\x05\x9AV[c\x01\xB4,\xD7`\xE1\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x01a\x04\"W`\0a\x03\xBA`\x04\x82\x86\x88a\x07\x17V[a\x03\xC3\x91a\x07`V[`\xE0\x1C\x90P`\0a\x03\xD8`\x18`\x04\x87\x89a\x07\x17V[a\x03\xE1\x91a\x07\xBEV[`@Qc~K\xD3)`\xE1\x1B\x81R``\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x83\x01R\x91P`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xFC\x97\xA6R\x90`D\x01a\x03ZV[cq\xDA\xF5\xE5`\xE1\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x01a\x04\xB1W`\0a\x04I`\x04\x82\x86\x88a\x07\x17V[a\x04R\x91a\x07`V[`\xE0\x1C\x90P`\0a\x04g`\x18`\x04\x87\x89a\x07\x17V[a\x04p\x91a\x07\xBEV[`@Qc\x0E%\n\x1B`\xE1\x1B\x81R``\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x83\x01R\x91P`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x1CJ\x146\x90`D\x01a\x03ZV[c>9\xEC9`\xE2\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x01a\x05@W`\0a\x04\xD8`\x04\x82\x86\x88a\x07\x17V[a\x04\xE1\x91a\x07`V[`\xE0\x1C\x90P`\0a\x04\xF6`\x18`\x04\x87\x89a\x07\x17V[a\x04\xFF\x91a\x07\xBEV[`@Qc\x01\xC6\x13\xC7`\xE2\x1B\x81R``\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x83\x01R\x91P`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x07\x18O\x1C\x90`D\x01a\x03ZV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FTokenWrapperHandler: Invalid fun`D\x82\x01Rhction sig`\xB8\x1B`d\x82\x01R`\x84\x01a\x02;V[PPPPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FHandlerHelpers: sender must be b`D\x82\x01Rm\x1C\x9AY\x19\xD9H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x92\x1B`d\x82\x01R`\x84\x01a\x02;V[V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06/W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x06FW`\0\x80\xFD[a\x06O\x82a\x06\x18V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06iW`\0\x80\xFD[\x825\x91Pa\x06y` \x84\x01a\x06\x18V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x06\x94W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x06\xB0W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xCFW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x06\xE3W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x06\xF2W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x07\x04W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80\x85\x85\x11\x15a\x07'W`\0\x80\xFD[\x83\x86\x11\x15a\x074W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a\x07ZW`\0\x19` \x84\x90\x03`\x03\x1B\x1B\x16[\x92\x91PPV[`\x01`\x01`\xE0\x1B\x03\x19\x815\x81\x81\x16\x91`\x04\x85\x10\x15a\x07\x88W\x80\x81\x86`\x04\x03`\x03\x1B\x1B\x83\x16\x16\x92P[PP\x92\x91PPV[`\x01`\x01`\xF0\x1B\x03\x19\x815\x81\x81\x16\x91`\x02\x85\x10\x15a\x07\x88W`\x02\x94\x90\x94\x03`\x03\x1B\x84\x90\x1B\x16\x90\x92\x16\x92\x91PPV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x815\x81\x81\x16\x91`\x14\x85\x10\x15a\x07\x88W`\x14\x94\x90\x94\x03`\x03\x1B\x84\x90\x1B\x16\x90\x92\x16\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x94\xB8~d\x16M\x01\x96\xBB[\xAA\xCD\xD3\xFBe\xD6\xF39\xC3\x91\xE2\xBC<O\xF1\x12\xE0\x99a\x16\xE9\xA2dsolcC\0\x08\x12\x003";
    /// The bytecode of the contract.
    pub static TOKENWRAPPERHANDLERCONTRACT_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xC5L*\x11\x11a\0[W\x80c\xC5L*\x11\x14a\0\xFAW\x80c\xD7\xF5\xB3Y\x14a\x01#W\x80c\xE2H\xCF\xF2\x14a\x016W\x80c\xEC\x97\xD3\xB4\x14a\x01IW`\0\x80\xFD[\x80c1\x8C\x13n\x14a\0\x82W\x80c\x7Fy\xBE\xA8\x14a\0\xB2W\x80c\xB8\xFA76\x14a\0\xE5W[`\0\x80\xFD[`\0Ta\0\x95\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD5a\0\xC06`\x04a\x064V[`\x03` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xA9V[a\0\xF8a\0\xF36`\x04a\x06VV[a\x01wV[\0[a\0\x95a\x01\x086`\x04a\x06\x82V[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xF8a\x0116`\x04a\x064V[a\x01\xCFV[a\0\xF8a\x01D6`\x04a\x06\x9BV[a\x02fV[a\x01ia\x01W6`\x04a\x064V[`\x02` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xA9V[a\x01\x7Fa\x05\xA5V[`\0\x82\x81R`\x01` \x81\x81R`@\x80\x84 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x87\x16\x90\x81\x17\x90\x91U\x84R`\x02\x82R\x80\x84 \x86\x90U`\x03\x90\x91R\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90UPPV[a\x01\xD7a\x05\xA5V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FHandlerHelpers: Bridge address c`D\x82\x01Rh\x06\x16\xE2wB\x06&R\x03`\xBC\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x02na\x05\xA5V[`\0\x806\x81a\x02\x80` \x82\x87\x89a\x07\x17V[a\x02\x89\x91a\x07AV[\x93Pa\x02\x99`$` \x87\x89a\x07\x17V[a\x02\xA2\x91a\x07`V[\x92Pa\x02\xB1\x85`$\x81\x89a\x07\x17V[`\0\x89\x81R`\x01` R`@\x90 T\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x16\x80c\x01\xEA\x8D\xC7`\xE5\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x01a\x03\x93W`\0a\x02\xF6`\x04\x82\x86\x88a\x07\x17V[a\x02\xFF\x91a\x07`V[`\xE0\x1C\x90P`\0a\x03\x14`\x06`\x04\x87\x89a\x07\x17V[a\x03\x1D\x91a\x07\x90V[`@Qc\x06\x15r9`\xE5\x1B\x81R`\xF0\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x83\x01R\x91P`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xC2\xAEG \x90`D\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03tW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x88W=`\0\x80>=`\0\xFD[PPPPPPa\x05\x9AV[c\x01\xB4,\xD7`\xE1\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x01a\x04\"W`\0a\x03\xBA`\x04\x82\x86\x88a\x07\x17V[a\x03\xC3\x91a\x07`V[`\xE0\x1C\x90P`\0a\x03\xD8`\x18`\x04\x87\x89a\x07\x17V[a\x03\xE1\x91a\x07\xBEV[`@Qc~K\xD3)`\xE1\x1B\x81R``\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x83\x01R\x91P`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xFC\x97\xA6R\x90`D\x01a\x03ZV[cq\xDA\xF5\xE5`\xE1\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x01a\x04\xB1W`\0a\x04I`\x04\x82\x86\x88a\x07\x17V[a\x04R\x91a\x07`V[`\xE0\x1C\x90P`\0a\x04g`\x18`\x04\x87\x89a\x07\x17V[a\x04p\x91a\x07\xBEV[`@Qc\x0E%\n\x1B`\xE1\x1B\x81R``\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x83\x01R\x91P`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x1CJ\x146\x90`D\x01a\x03ZV[c>9\xEC9`\xE2\x1B`\x01`\x01`\xE0\x1B\x03\x19\x86\x16\x01a\x05@W`\0a\x04\xD8`\x04\x82\x86\x88a\x07\x17V[a\x04\xE1\x91a\x07`V[`\xE0\x1C\x90P`\0a\x04\xF6`\x18`\x04\x87\x89a\x07\x17V[a\x04\xFF\x91a\x07\xBEV[`@Qc\x01\xC6\x13\xC7`\xE2\x1B\x81R``\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x83\x01R\x91P`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x07\x18O\x1C\x90`D\x01a\x03ZV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FTokenWrapperHandler: Invalid fun`D\x82\x01Rhction sig`\xB8\x1B`d\x82\x01R`\x84\x01a\x02;V[PPPPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FHandlerHelpers: sender must be b`D\x82\x01Rm\x1C\x9AY\x19\xD9H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x92\x1B`d\x82\x01R`\x84\x01a\x02;V[V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06/W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x06FW`\0\x80\xFD[a\x06O\x82a\x06\x18V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06iW`\0\x80\xFD[\x825\x91Pa\x06y` \x84\x01a\x06\x18V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x06\x94W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x06\xB0W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xCFW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x06\xE3W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x06\xF2W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x07\x04W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80\x85\x85\x11\x15a\x07'W`\0\x80\xFD[\x83\x86\x11\x15a\x074W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a\x07ZW`\0\x19` \x84\x90\x03`\x03\x1B\x1B\x16[\x92\x91PPV[`\x01`\x01`\xE0\x1B\x03\x19\x815\x81\x81\x16\x91`\x04\x85\x10\x15a\x07\x88W\x80\x81\x86`\x04\x03`\x03\x1B\x1B\x83\x16\x16\x92P[PP\x92\x91PPV[`\x01`\x01`\xF0\x1B\x03\x19\x815\x81\x81\x16\x91`\x02\x85\x10\x15a\x07\x88W`\x02\x94\x90\x94\x03`\x03\x1B\x84\x90\x1B\x16\x90\x92\x16\x92\x91PPV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x815\x81\x81\x16\x91`\x14\x85\x10\x15a\x07\x88W`\x14\x94\x90\x94\x03`\x03\x1B\x84\x90\x1B\x16\x90\x92\x16\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x94\xB8~d\x16M\x01\x96\xBB[\xAA\xCD\xD3\xFBe\xD6\xF39\xC3\x91\xE2\xBC<O\xF1\x12\xE0\x99a\x16\xE9\xA2dsolcC\0\x08\x12\x003";
    /// The deployed bytecode of the contract.
    pub static TOKENWRAPPERHANDLERCONTRACT_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct TokenWrapperHandlerContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TokenWrapperHandlerContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TokenWrapperHandlerContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TokenWrapperHandlerContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TokenWrapperHandlerContract<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TokenWrapperHandlerContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TokenWrapperHandlerContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                TOKENWRAPPERHANDLERCONTRACT_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                TOKENWRAPPERHANDLERCONTRACT_ABI.clone(),
                TOKENWRAPPERHANDLERCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `_bridgeAddress` (0x318c136e) function
        pub fn bridge_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([49, 140, 19, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_contractAddressToResourceID` (0xec97d3b4) function
        pub fn contract_address_to_resource_id(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([236, 151, 211, 180], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_contractWhitelist` (0x7f79bea8) function
        pub fn contract_whitelist(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([127, 121, 190, 168], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_resourceIDToContractAddress` (0xc54c2a11) function
        pub fn resource_id_to_contract_address(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([197, 76, 42, 17], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeProposal` (0xe248cff2) function
        pub fn execute_proposal(
            &self,
            resource_id: [u8; 32],
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 72, 207, 242], (resource_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `migrateBridge` (0xd7f5b359) function
        pub fn migrate_bridge(
            &self,
            new_bridge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 245, 179, 89], new_bridge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setResource` (0xb8fa3736) function
        pub fn set_resource(
            &self,
            resource_id: [u8; 32],
            contract_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [184, 250, 55, 54],
                    (resource_id, contract_address),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>>
        for TokenWrapperHandlerContract<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `_bridgeAddress` function with signature `_bridgeAddress()` and selector `0x318c136e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "_bridgeAddress", abi = "_bridgeAddress()")]
    pub struct BridgeAddressCall;
    ///Container type for all input parameters for the `_contractAddressToResourceID` function with signature `_contractAddressToResourceID(address)` and selector `0xec97d3b4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "_contractAddressToResourceID",
        abi = "_contractAddressToResourceID(address)"
    )]
    pub struct ContractAddressToResourceIDCall(
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `_contractWhitelist` function with signature `_contractWhitelist(address)` and selector `0x7f79bea8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "_contractWhitelist", abi = "_contractWhitelist(address)")]
    pub struct ContractWhitelistCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `_resourceIDToContractAddress` function with signature `_resourceIDToContractAddress(bytes32)` and selector `0xc54c2a11`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "_resourceIDToContractAddress",
        abi = "_resourceIDToContractAddress(bytes32)"
    )]
    pub struct ResourceIDToContractAddressCall(pub [u8; 32]);
    ///Container type for all input parameters for the `executeProposal` function with signature `executeProposal(bytes32,bytes)` and selector `0xe248cff2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "executeProposal", abi = "executeProposal(bytes32,bytes)")]
    pub struct ExecuteProposalCall {
        pub resource_id: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `migrateBridge` function with signature `migrateBridge(address)` and selector `0xd7f5b359`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "migrateBridge", abi = "migrateBridge(address)")]
    pub struct MigrateBridgeCall {
        pub new_bridge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setResource` function with signature `setResource(bytes32,address)` and selector `0xb8fa3736`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setResource", abi = "setResource(bytes32,address)")]
    pub struct SetResourceCall {
        pub resource_id: [u8; 32],
        pub contract_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum TokenWrapperHandlerContractCalls {
        BridgeAddress(BridgeAddressCall),
        ContractAddressToResourceID(ContractAddressToResourceIDCall),
        ContractWhitelist(ContractWhitelistCall),
        ResourceIDToContractAddress(ResourceIDToContractAddressCall),
        ExecuteProposal(ExecuteProposalCall),
        MigrateBridge(MigrateBridgeCall),
        SetResource(SetResourceCall),
    }
    impl ::ethers::core::abi::AbiDecode for TokenWrapperHandlerContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <BridgeAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::BridgeAddress(decoded));
            }
            if let Ok(decoded) = <ContractAddressToResourceIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ContractAddressToResourceID(decoded));
            }
            if let Ok(decoded) = <ContractWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ContractWhitelist(decoded));
            }
            if let Ok(decoded) = <ResourceIDToContractAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ResourceIDToContractAddress(decoded));
            }
            if let Ok(decoded) =
                <ExecuteProposalCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ExecuteProposal(decoded));
            }
            if let Ok(decoded) =
                <MigrateBridgeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::MigrateBridge(decoded));
            }
            if let Ok(decoded) =
                <SetResourceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SetResource(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TokenWrapperHandlerContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BridgeAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContractAddressToResourceID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContractWhitelist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResourceIDToContractAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteProposal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MigrateBridge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetResource(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for TokenWrapperHandlerContractCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::BridgeAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContractAddressToResourceID(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContractWhitelist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ResourceIDToContractAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteProposal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MigrateBridge(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetResource(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BridgeAddressCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(value: BridgeAddressCall) -> Self {
            Self::BridgeAddress(value)
        }
    }
    impl ::core::convert::From<ContractAddressToResourceIDCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(value: ContractAddressToResourceIDCall) -> Self {
            Self::ContractAddressToResourceID(value)
        }
    }
    impl ::core::convert::From<ContractWhitelistCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(value: ContractWhitelistCall) -> Self {
            Self::ContractWhitelist(value)
        }
    }
    impl ::core::convert::From<ResourceIDToContractAddressCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(value: ResourceIDToContractAddressCall) -> Self {
            Self::ResourceIDToContractAddress(value)
        }
    }
    impl ::core::convert::From<ExecuteProposalCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(value: ExecuteProposalCall) -> Self {
            Self::ExecuteProposal(value)
        }
    }
    impl ::core::convert::From<MigrateBridgeCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(value: MigrateBridgeCall) -> Self {
            Self::MigrateBridge(value)
        }
    }
    impl ::core::convert::From<SetResourceCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(value: SetResourceCall) -> Self {
            Self::SetResource(value)
        }
    }
    ///Container type for all return fields from the `_bridgeAddress` function with signature `_bridgeAddress()` and selector `0x318c136e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BridgeAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `_contractAddressToResourceID` function with signature `_contractAddressToResourceID(address)` and selector `0xec97d3b4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ContractAddressToResourceIDReturn(pub [u8; 32]);
    ///Container type for all return fields from the `_contractWhitelist` function with signature `_contractWhitelist(address)` and selector `0x7f79bea8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ContractWhitelistReturn(pub bool);
    ///Container type for all return fields from the `_resourceIDToContractAddress` function with signature `_resourceIDToContractAddress(bytes32)` and selector `0xc54c2a11`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ResourceIDToContractAddressReturn(
        pub ::ethers::core::types::Address,
    );
}
