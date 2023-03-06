pub use treasury_handler_contract::*;
#[doc = r" This module was auto-generated with ethers-rs Abigen."]
#[doc = r" More information at: <https://github.com/gakonst/ethers-rs>"]
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod treasury_handler_contract {
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"bridgeAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"initialResourceIDs\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"initialContractAddresses\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_bridgeAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_contractAddressToResourceID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_contractWhitelist\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_resourceIDToContractAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeProposal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newBridge\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"migrateBridge\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"contractAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setResource\",\"outputs\":[]}]" ;
    #[doc = "The parsed JSON ABI of the contract."]
    pub static TREASURYHANDLERCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    # [rustfmt :: skip] const __BYTECODE : & [u8] = & [96 , 128 , 96 , 64 , 82 , 52 , 128 , 21 , 98 , 0 , 0 , 17 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 64 , 81 , 98 , 0 , 10 , 143 , 56 , 3 , 128 , 98 , 0 , 10 , 143 , 131 , 57 , 129 , 1 , 96 , 64 , 129 , 144 , 82 , 98 , 0 , 0 , 52 , 145 , 98 , 0 , 2 , 46 , 86 , 91 , 128 , 81 , 130 , 81 , 20 , 98 , 0 , 0 , 176 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 60 , 96 , 36 , 130 , 1 , 82 , 127 , 105 , 110 , 105 , 116 , 105 , 97 , 108 , 82 , 101 , 115 , 111 , 117 , 114 , 99 , 101 , 73 , 68 , 115 , 32 , 97 , 110 , 100 , 32 , 105 , 110 , 105 , 116 , 105 , 97 , 108 , 67 , 111 , 96 , 68 , 130 , 1 , 82 , 127 , 110 , 116 , 114 , 97 , 99 , 116 , 65 , 100 , 100 , 114 , 101 , 115 , 115 , 101 , 115 , 32 , 108 , 101 , 110 , 32 , 109 , 105 , 115 , 109 , 97 , 116 , 99 , 104 , 0 , 0 , 0 , 0 , 96 , 100 , 130 , 1 , 82 , 96 , 132 , 1 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 253 , 91 , 96 , 0 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 22 , 23 , 129 , 85 , 91 , 130 , 81 , 129 , 16 , 21 , 98 , 0 , 1 , 53 , 87 , 98 , 0 , 1 , 32 , 131 , 130 , 129 , 81 , 129 , 16 , 98 , 0 , 0 , 239 , 87 , 98 , 0 , 0 , 239 , 98 , 0 , 3 , 147 , 86 , 91 , 96 , 32 , 2 , 96 , 32 , 1 , 1 , 81 , 131 , 131 , 129 , 81 , 129 , 16 , 98 , 0 , 1 , 12 , 87 , 98 , 0 , 1 , 12 , 98 , 0 , 3 , 147 , 86 , 91 , 96 , 32 , 2 , 96 , 32 , 1 , 1 , 81 , 98 , 0 , 1 , 63 , 96 , 32 , 27 , 96 , 32 , 28 , 86 , 91 , 128 , 98 , 0 , 1 , 44 , 129 , 98 , 0 , 3 , 105 , 86 , 91 , 145 , 80 , 80 , 98 , 0 , 0 , 204 , 86 , 91 , 80 , 80 , 80 , 80 , 98 , 0 , 3 , 191 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 1 , 96 , 32 , 129 , 129 , 82 , 96 , 64 , 128 , 132 , 32 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 144 , 150 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 144 , 150 , 22 , 134 , 23 , 144 , 85 , 147 , 131 , 82 , 96 , 2 , 129 , 82 , 131 , 131 , 32 , 148 , 144 , 148 , 85 , 96 , 3 , 144 , 147 , 82 , 32 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 145 , 23 , 144 , 85 , 86 , 91 , 128 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 129 , 22 , 129 , 20 , 98 , 0 , 1 , 166 , 87 , 96 , 0 , 128 , 253 , 91 , 145 , 144 , 80 , 86 , 91 , 96 , 0 , 130 , 96 , 31 , 131 , 1 , 18 , 98 , 0 , 1 , 189 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 81 , 96 , 32 , 98 , 0 , 1 , 214 , 98 , 0 , 1 , 208 , 131 , 98 , 0 , 3 , 67 , 86 , 91 , 98 , 0 , 3 , 16 , 86 , 91 , 128 , 131 , 130 , 82 , 130 , 130 , 1 , 145 , 80 , 130 , 134 , 1 , 135 , 132 , 134 , 96 , 5 , 27 , 137 , 1 , 1 , 17 , 21 , 98 , 0 , 1 , 247 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 91 , 133 , 129 , 16 , 21 , 98 , 0 , 2 , 33 , 87 , 98 , 0 , 2 , 14 , 130 , 98 , 0 , 1 , 142 , 86 , 91 , 132 , 82 , 146 , 132 , 1 , 146 , 144 , 132 , 1 , 144 , 96 , 1 , 1 , 98 , 0 , 1 , 250 , 86 , 91 , 80 , 144 , 151 , 150 , 80 , 80 , 80 , 80 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 0 , 96 , 96 , 132 , 134 , 3 , 18 , 21 , 98 , 0 , 2 , 68 , 87 , 96 , 0 , 128 , 253 , 91 , 98 , 0 , 2 , 79 , 132 , 98 , 0 , 1 , 142 , 86 , 91 , 96 , 32 , 133 , 129 , 1 , 81 , 145 , 148 , 80 , 144 , 96 , 1 , 96 , 1 , 96 , 64 , 27 , 3 , 128 , 130 , 17 , 21 , 98 , 0 , 2 , 111 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 135 , 1 , 145 , 80 , 135 , 96 , 31 , 131 , 1 , 18 , 98 , 0 , 2 , 132 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 81 , 98 , 0 , 2 , 149 , 98 , 0 , 1 , 208 , 130 , 98 , 0 , 3 , 67 , 86 , 91 , 128 , 130 , 130 , 82 , 133 , 130 , 1 , 145 , 80 , 133 , 133 , 1 , 139 , 135 , 133 , 96 , 5 , 27 , 136 , 1 , 1 , 17 , 21 , 98 , 0 , 2 , 182 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 149 , 80 , 91 , 131 , 134 , 16 , 21 , 98 , 0 , 2 , 219 , 87 , 128 , 81 , 131 , 82 , 96 , 1 , 149 , 144 , 149 , 1 , 148 , 145 , 134 , 1 , 145 , 134 , 1 , 98 , 0 , 2 , 187 , 86 , 91 , 80 , 96 , 64 , 138 , 1 , 81 , 144 , 151 , 80 , 148 , 80 , 80 , 80 , 128 , 131 , 17 , 21 , 98 , 0 , 2 , 246 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 80 , 98 , 0 , 3 , 6 , 134 , 130 , 135 , 1 , 98 , 0 , 1 , 171 , 86 , 91 , 145 , 80 , 80 , 146 , 80 , 146 , 80 , 146 , 86 , 91 , 96 , 64 , 81 , 96 , 31 , 130 , 1 , 96 , 31 , 25 , 22 , 129 , 1 , 96 , 1 , 96 , 1 , 96 , 64 , 27 , 3 , 129 , 17 , 130 , 130 , 16 , 23 , 21 , 98 , 0 , 3 , 59 , 87 , 98 , 0 , 3 , 59 , 98 , 0 , 3 , 169 , 86 , 91 , 96 , 64 , 82 , 145 , 144 , 80 , 86 , 91 , 96 , 0 , 96 , 1 , 96 , 1 , 96 , 64 , 27 , 3 , 130 , 17 , 21 , 98 , 0 , 3 , 95 , 87 , 98 , 0 , 3 , 95 , 98 , 0 , 3 , 169 , 86 , 91 , 80 , 96 , 5 , 27 , 96 , 32 , 1 , 144 , 86 , 91 , 96 , 0 , 96 , 0 , 25 , 130 , 20 , 21 , 98 , 0 , 3 , 140 , 87 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 17 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 80 , 96 , 1 , 1 , 144 , 86 , 91 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 50 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 99 , 78 , 72 , 123 , 113 , 96 , 224 , 27 , 96 , 0 , 82 , 96 , 65 , 96 , 4 , 82 , 96 , 36 , 96 , 0 , 253 , 91 , 97 , 6 , 192 , 128 , 98 , 0 , 3 , 207 , 96 , 0 , 57 , 96 , 0 , 243 , 254 , 96 , 128 , 96 , 64 , 82 , 52 , 128 , 21 , 97 , 0 , 16 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 4 , 54 , 16 , 97 , 0 , 125 , 87 , 96 , 0 , 53 , 96 , 224 , 28 , 128 , 99 , 197 , 76 , 42 , 17 , 17 , 97 , 0 , 91 , 87 , 128 , 99 , 197 , 76 , 42 , 17 , 20 , 97 , 0 , 250 , 87 , 128 , 99 , 215 , 245 , 179 , 89 , 20 , 97 , 1 , 35 , 87 , 128 , 99 , 226 , 72 , 207 , 242 , 20 , 97 , 1 , 54 , 87 , 128 , 99 , 236 , 151 , 211 , 180 , 20 , 97 , 1 , 73 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 49 , 140 , 19 , 110 , 20 , 97 , 0 , 130 , 87 , 128 , 99 , 127 , 121 , 190 , 168 , 20 , 97 , 0 , 178 , 87 , 128 , 99 , 184 , 250 , 55 , 54 , 20 , 97 , 0 , 229 , 87 , 91 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 84 , 97 , 0 , 149 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 129 , 86 , 91 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 144 , 145 , 22 , 129 , 82 , 96 , 32 , 1 , 91 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 243 , 91 , 97 , 0 , 213 , 97 , 0 , 192 , 54 , 96 , 4 , 97 , 4 , 251 , 86 , 91 , 96 , 3 , 96 , 32 , 82 , 96 , 0 , 144 , 129 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 129 , 86 , 91 , 96 , 64 , 81 , 144 , 21 , 21 , 129 , 82 , 96 , 32 , 1 , 97 , 0 , 169 , 86 , 91 , 97 , 0 , 248 , 97 , 0 , 243 , 54 , 96 , 4 , 97 , 5 , 54 , 86 , 91 , 97 , 1 , 119 , 86 , 91 , 0 , 91 , 97 , 0 , 149 , 97 , 1 , 8 , 54 , 96 , 4 , 97 , 5 , 29 , 86 , 91 , 96 , 1 , 96 , 32 , 82 , 96 , 0 , 144 , 129 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 129 , 86 , 91 , 97 , 0 , 248 , 97 , 1 , 49 , 54 , 96 , 4 , 97 , 4 , 251 , 86 , 91 , 97 , 1 , 207 , 86 , 91 , 97 , 0 , 248 , 97 , 1 , 68 , 54 , 96 , 4 , 97 , 5 , 98 , 86 , 91 , 97 , 1 , 249 , 86 , 91 , 97 , 1 , 105 , 97 , 1 , 87 , 54 , 96 , 4 , 97 , 4 , 251 , 86 , 91 , 96 , 2 , 96 , 32 , 82 , 96 , 0 , 144 , 129 , 82 , 96 , 64 , 144 , 32 , 84 , 129 , 86 , 91 , 96 , 64 , 81 , 144 , 129 , 82 , 96 , 32 , 1 , 97 , 0 , 169 , 86 , 91 , 97 , 1 , 127 , 97 , 4 , 131 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 1 , 96 , 32 , 129 , 129 , 82 , 96 , 64 , 128 , 132 , 32 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 135 , 22 , 144 , 129 , 23 , 144 , 145 , 85 , 132 , 82 , 96 , 2 , 130 , 82 , 128 , 132 , 32 , 134 , 144 , 85 , 96 , 3 , 144 , 145 , 82 , 144 , 145 , 32 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 145 , 23 , 144 , 85 , 80 , 80 , 86 , 91 , 97 , 1 , 215 , 97 , 4 , 131 , 86 , 91 , 96 , 0 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 146 , 144 , 146 , 22 , 145 , 144 , 145 , 23 , 144 , 85 , 86 , 91 , 97 , 2 , 1 , 97 , 4 , 131 , 86 , 91 , 96 , 0 , 128 , 54 , 129 , 97 , 2 , 19 , 96 , 32 , 130 , 135 , 137 , 97 , 5 , 222 , 86 , 91 , 97 , 2 , 28 , 145 , 97 , 6 , 61 , 86 , 91 , 147 , 80 , 97 , 2 , 44 , 96 , 36 , 96 , 32 , 135 , 137 , 97 , 5 , 222 , 86 , 91 , 97 , 2 , 53 , 145 , 97 , 6 , 92 , 86 , 91 , 146 , 80 , 97 , 2 , 68 , 133 , 96 , 36 , 129 , 137 , 97 , 5 , 222 , 86 , 91 , 96 , 0 , 137 , 129 , 82 , 96 , 1 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 145 , 147 , 80 , 145 , 80 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 128 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 133 , 22 , 99 , 114 , 193 , 173 , 3 , 96 , 224 , 27 , 20 , 21 , 97 , 3 , 38 , 87 , 96 , 0 , 97 , 2 , 138 , 96 , 4 , 130 , 134 , 136 , 97 , 5 , 222 , 86 , 91 , 97 , 2 , 147 , 145 , 97 , 6 , 92 , 86 , 91 , 96 , 224 , 28 , 144 , 80 , 96 , 0 , 97 , 2 , 168 , 96 , 24 , 96 , 4 , 135 , 137 , 97 , 5 , 222 , 86 , 91 , 97 , 2 , 177 , 145 , 97 , 6 , 8 , 86 , 91 , 96 , 64 , 81 , 99 , 114 , 193 , 173 , 3 , 96 , 224 , 27 , 129 , 82 , 96 , 96 , 145 , 144 , 145 , 28 , 96 , 4 , 130 , 1 , 129 , 144 , 82 , 99 , 255 , 255 , 255 , 255 , 132 , 22 , 96 , 36 , 131 , 1 , 82 , 145 , 80 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 144 , 99 , 114 , 193 , 173 , 3 , 144 , 96 , 68 , 1 , 96 , 0 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 96 , 0 , 135 , 128 , 59 , 21 , 128 , 21 , 97 , 3 , 7 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 90 , 241 , 21 , 128 , 21 , 97 , 3 , 27 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 80 , 80 , 97 , 4 , 120 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 133 , 22 , 99 , 98 , 44 , 119 , 217 , 96 , 224 , 27 , 20 , 21 , 97 , 4 , 52 , 87 , 96 , 0 , 97 , 3 , 78 , 96 , 4 , 130 , 134 , 136 , 97 , 5 , 222 , 86 , 91 , 97 , 3 , 87 , 145 , 97 , 6 , 92 , 86 , 91 , 96 , 224 , 28 , 144 , 80 , 96 , 0 , 97 , 3 , 108 , 96 , 24 , 96 , 4 , 135 , 137 , 97 , 5 , 222 , 86 , 91 , 97 , 3 , 117 , 145 , 97 , 6 , 8 , 86 , 91 , 96 , 96 , 28 , 144 , 80 , 96 , 0 , 97 , 3 , 138 , 96 , 44 , 96 , 24 , 136 , 138 , 97 , 5 , 222 , 86 , 91 , 97 , 3 , 147 , 145 , 97 , 6 , 8 , 86 , 91 , 96 , 96 , 28 , 144 , 80 , 96 , 0 , 97 , 3 , 168 , 96 , 76 , 96 , 44 , 137 , 139 , 97 , 5 , 222 , 86 , 91 , 97 , 3 , 177 , 145 , 97 , 6 , 61 , 86 , 91 , 96 , 64 , 81 , 99 , 98 , 44 , 119 , 217 , 96 , 224 , 27 , 129 , 82 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 129 , 22 , 96 , 4 , 131 , 1 , 82 , 132 , 129 , 22 , 96 , 36 , 131 , 1 , 82 , 96 , 68 , 130 , 1 , 131 , 144 , 82 , 99 , 255 , 255 , 255 , 255 , 135 , 22 , 96 , 100 , 131 , 1 , 82 , 145 , 146 , 80 , 144 , 134 , 22 , 144 , 99 , 98 , 44 , 119 , 217 , 144 , 96 , 132 , 1 , 96 , 0 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 96 , 0 , 135 , 128 , 59 , 21 , 128 , 21 , 97 , 4 , 19 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 90 , 241 , 21 , 128 , 21 , 97 , 4 , 39 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 80 , 80 , 80 , 80 , 97 , 4 , 120 , 86 , 91 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 20 , 96 , 36 , 130 , 1 , 82 , 115 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 102 , 117 , 110 , 99 , 116 , 105 , 111 , 110 , 32 , 115 , 105 , 103 , 96 , 96 , 27 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 91 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 253 , 91 , 80 , 80 , 80 , 80 , 80 , 80 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 4 , 221 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 30 , 96 , 36 , 130 , 1 , 82 , 127 , 115 , 101 , 110 , 100 , 101 , 114 , 32 , 109 , 117 , 115 , 116 , 32 , 98 , 101 , 32 , 98 , 114 , 105 , 100 , 103 , 101 , 32 , 99 , 111 , 110 , 116 , 114 , 97 , 99 , 116 , 0 , 0 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 4 , 111 , 86 , 91 , 86 , 91 , 128 , 53 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 129 , 22 , 129 , 20 , 97 , 4 , 246 , 87 , 96 , 0 , 128 , 253 , 91 , 145 , 144 , 80 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 5 , 13 , 87 , 96 , 0 , 128 , 253 , 91 , 97 , 5 , 22 , 130 , 97 , 4 , 223 , 86 , 91 , 147 , 146 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 5 , 47 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 53 , 145 , 144 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 5 , 73 , 87 , 96 , 0 , 128 , 253 , 91 , 130 , 53 , 145 , 80 , 97 , 5 , 89 , 96 , 32 , 132 , 1 , 97 , 4 , 223 , 86 , 91 , 144 , 80 , 146 , 80 , 146 , 144 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 0 , 96 , 64 , 132 , 134 , 3 , 18 , 21 , 97 , 5 , 119 , 87 , 96 , 0 , 128 , 253 , 91 , 131 , 53 , 146 , 80 , 96 , 32 , 132 , 1 , 53 , 103 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 128 , 130 , 17 , 21 , 97 , 5 , 150 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 134 , 1 , 145 , 80 , 134 , 96 , 31 , 131 , 1 , 18 , 97 , 5 , 170 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 53 , 129 , 129 , 17 , 21 , 97 , 5 , 185 , 87 , 96 , 0 , 128 , 253 , 91 , 135 , 96 , 32 , 130 , 133 , 1 , 1 , 17 , 21 , 97 , 5 , 203 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 32 , 131 , 1 , 148 , 80 , 128 , 147 , 80 , 80 , 80 , 80 , 146 , 80 , 146 , 80 , 146 , 86 , 91 , 96 , 0 , 128 , 133 , 133 , 17 , 21 , 97 , 5 , 238 , 87 , 96 , 0 , 128 , 253 , 91 , 131 , 134 , 17 , 21 , 97 , 5 , 251 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 80 , 130 , 1 , 147 , 145 , 144 , 146 , 3 , 145 , 80 , 86 , 91 , 107 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 25 , 129 , 53 , 129 , 129 , 22 , 145 , 96 , 20 , 133 , 16 , 21 , 97 , 6 , 53 , 87 , 128 , 129 , 134 , 96 , 20 , 3 , 96 , 3 , 27 , 27 , 131 , 22 , 22 , 146 , 80 , 91 , 80 , 80 , 146 , 145 , 80 , 80 , 86 , 91 , 128 , 53 , 96 , 32 , 131 , 16 , 21 , 97 , 6 , 86 , 87 , 96 , 0 , 25 , 96 , 32 , 132 , 144 , 3 , 96 , 3 , 27 , 27 , 22 , 91 , 146 , 145 , 80 , 80 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 129 , 53 , 129 , 129 , 22 , 145 , 96 , 4 , 133 , 16 , 21 , 97 , 6 , 53 , 87 , 96 , 4 , 148 , 144 , 148 , 3 , 96 , 3 , 27 , 132 , 144 , 27 , 22 , 144 , 146 , 22 , 146 , 145 , 80 , 80 , 86 , 254 , 162 , 100 , 105 , 112 , 102 , 115 , 88 , 34 , 18 , 32 , 250 , 82 , 97 , 157 , 172 , 167 , 178 , 176 , 154 , 245 , 180 , 96 , 151 , 7 , 141 , 57 , 136 , 102 , 217 , 171 , 198 , 248 , 184 , 132 , 175 , 252 , 31 , 131 , 60 , 116 , 148 , 238 , 100 , 115 , 111 , 108 , 99 , 67 , 0 , 8 , 5 , 0 , 51] ;
    #[doc = "The bytecode of the contract."]
    pub static TREASURYHANDLERCONTRACT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    # [rustfmt :: skip] const __DEPLOYED_BYTECODE : & [u8] = & [96 , 128 , 96 , 64 , 82 , 52 , 128 , 21 , 97 , 0 , 16 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 96 , 4 , 54 , 16 , 97 , 0 , 125 , 87 , 96 , 0 , 53 , 96 , 224 , 28 , 128 , 99 , 197 , 76 , 42 , 17 , 17 , 97 , 0 , 91 , 87 , 128 , 99 , 197 , 76 , 42 , 17 , 20 , 97 , 0 , 250 , 87 , 128 , 99 , 215 , 245 , 179 , 89 , 20 , 97 , 1 , 35 , 87 , 128 , 99 , 226 , 72 , 207 , 242 , 20 , 97 , 1 , 54 , 87 , 128 , 99 , 236 , 151 , 211 , 180 , 20 , 97 , 1 , 73 , 87 , 96 , 0 , 128 , 253 , 91 , 128 , 99 , 49 , 140 , 19 , 110 , 20 , 97 , 0 , 130 , 87 , 128 , 99 , 127 , 121 , 190 , 168 , 20 , 97 , 0 , 178 , 87 , 128 , 99 , 184 , 250 , 55 , 54 , 20 , 97 , 0 , 229 , 87 , 91 , 96 , 0 , 128 , 253 , 91 , 96 , 0 , 84 , 97 , 0 , 149 , 144 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 129 , 86 , 91 , 96 , 64 , 81 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 144 , 145 , 22 , 129 , 82 , 96 , 32 , 1 , 91 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 243 , 91 , 97 , 0 , 213 , 97 , 0 , 192 , 54 , 96 , 4 , 97 , 4 , 251 , 86 , 91 , 96 , 3 , 96 , 32 , 82 , 96 , 0 , 144 , 129 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 255 , 22 , 129 , 86 , 91 , 96 , 64 , 81 , 144 , 21 , 21 , 129 , 82 , 96 , 32 , 1 , 97 , 0 , 169 , 86 , 91 , 97 , 0 , 248 , 97 , 0 , 243 , 54 , 96 , 4 , 97 , 5 , 54 , 86 , 91 , 97 , 1 , 119 , 86 , 91 , 0 , 91 , 97 , 0 , 149 , 97 , 1 , 8 , 54 , 96 , 4 , 97 , 5 , 29 , 86 , 91 , 96 , 1 , 96 , 32 , 82 , 96 , 0 , 144 , 129 , 82 , 96 , 64 , 144 , 32 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 129 , 86 , 91 , 97 , 0 , 248 , 97 , 1 , 49 , 54 , 96 , 4 , 97 , 4 , 251 , 86 , 91 , 97 , 1 , 207 , 86 , 91 , 97 , 0 , 248 , 97 , 1 , 68 , 54 , 96 , 4 , 97 , 5 , 98 , 86 , 91 , 97 , 1 , 249 , 86 , 91 , 97 , 1 , 105 , 97 , 1 , 87 , 54 , 96 , 4 , 97 , 4 , 251 , 86 , 91 , 96 , 2 , 96 , 32 , 82 , 96 , 0 , 144 , 129 , 82 , 96 , 64 , 144 , 32 , 84 , 129 , 86 , 91 , 96 , 64 , 81 , 144 , 129 , 82 , 96 , 32 , 1 , 97 , 0 , 169 , 86 , 91 , 97 , 1 , 127 , 97 , 4 , 131 , 86 , 91 , 96 , 0 , 130 , 129 , 82 , 96 , 1 , 96 , 32 , 129 , 129 , 82 , 96 , 64 , 128 , 132 , 32 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 135 , 22 , 144 , 129 , 23 , 144 , 145 , 85 , 132 , 82 , 96 , 2 , 130 , 82 , 128 , 132 , 32 , 134 , 144 , 85 , 96 , 3 , 144 , 145 , 82 , 144 , 145 , 32 , 128 , 84 , 96 , 255 , 25 , 22 , 144 , 145 , 23 , 144 , 85 , 80 , 80 , 86 , 91 , 97 , 1 , 215 , 97 , 4 , 131 , 86 , 91 , 96 , 0 , 128 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 25 , 22 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 146 , 144 , 146 , 22 , 145 , 144 , 145 , 23 , 144 , 85 , 86 , 91 , 97 , 2 , 1 , 97 , 4 , 131 , 86 , 91 , 96 , 0 , 128 , 54 , 129 , 97 , 2 , 19 , 96 , 32 , 130 , 135 , 137 , 97 , 5 , 222 , 86 , 91 , 97 , 2 , 28 , 145 , 97 , 6 , 61 , 86 , 91 , 147 , 80 , 97 , 2 , 44 , 96 , 36 , 96 , 32 , 135 , 137 , 97 , 5 , 222 , 86 , 91 , 97 , 2 , 53 , 145 , 97 , 6 , 92 , 86 , 91 , 146 , 80 , 97 , 2 , 68 , 133 , 96 , 36 , 129 , 137 , 97 , 5 , 222 , 86 , 91 , 96 , 0 , 137 , 129 , 82 , 96 , 1 , 96 , 32 , 82 , 96 , 64 , 144 , 32 , 84 , 145 , 147 , 80 , 145 , 80 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 128 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 133 , 22 , 99 , 114 , 193 , 173 , 3 , 96 , 224 , 27 , 20 , 21 , 97 , 3 , 38 , 87 , 96 , 0 , 97 , 2 , 138 , 96 , 4 , 130 , 134 , 136 , 97 , 5 , 222 , 86 , 91 , 97 , 2 , 147 , 145 , 97 , 6 , 92 , 86 , 91 , 96 , 224 , 28 , 144 , 80 , 96 , 0 , 97 , 2 , 168 , 96 , 24 , 96 , 4 , 135 , 137 , 97 , 5 , 222 , 86 , 91 , 97 , 2 , 177 , 145 , 97 , 6 , 8 , 86 , 91 , 96 , 64 , 81 , 99 , 114 , 193 , 173 , 3 , 96 , 224 , 27 , 129 , 82 , 96 , 96 , 145 , 144 , 145 , 28 , 96 , 4 , 130 , 1 , 129 , 144 , 82 , 99 , 255 , 255 , 255 , 255 , 132 , 22 , 96 , 36 , 131 , 1 , 82 , 145 , 80 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 132 , 22 , 144 , 99 , 114 , 193 , 173 , 3 , 144 , 96 , 68 , 1 , 96 , 0 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 96 , 0 , 135 , 128 , 59 , 21 , 128 , 21 , 97 , 3 , 7 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 90 , 241 , 21 , 128 , 21 , 97 , 3 , 27 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 80 , 80 , 97 , 4 , 120 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 133 , 22 , 99 , 98 , 44 , 119 , 217 , 96 , 224 , 27 , 20 , 21 , 97 , 4 , 52 , 87 , 96 , 0 , 97 , 3 , 78 , 96 , 4 , 130 , 134 , 136 , 97 , 5 , 222 , 86 , 91 , 97 , 3 , 87 , 145 , 97 , 6 , 92 , 86 , 91 , 96 , 224 , 28 , 144 , 80 , 96 , 0 , 97 , 3 , 108 , 96 , 24 , 96 , 4 , 135 , 137 , 97 , 5 , 222 , 86 , 91 , 97 , 3 , 117 , 145 , 97 , 6 , 8 , 86 , 91 , 96 , 96 , 28 , 144 , 80 , 96 , 0 , 97 , 3 , 138 , 96 , 44 , 96 , 24 , 136 , 138 , 97 , 5 , 222 , 86 , 91 , 97 , 3 , 147 , 145 , 97 , 6 , 8 , 86 , 91 , 96 , 96 , 28 , 144 , 80 , 96 , 0 , 97 , 3 , 168 , 96 , 76 , 96 , 44 , 137 , 139 , 97 , 5 , 222 , 86 , 91 , 97 , 3 , 177 , 145 , 97 , 6 , 61 , 86 , 91 , 96 , 64 , 81 , 99 , 98 , 44 , 119 , 217 , 96 , 224 , 27 , 129 , 82 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 133 , 129 , 22 , 96 , 4 , 131 , 1 , 82 , 132 , 129 , 22 , 96 , 36 , 131 , 1 , 82 , 96 , 68 , 130 , 1 , 131 , 144 , 82 , 99 , 255 , 255 , 255 , 255 , 135 , 22 , 96 , 100 , 131 , 1 , 82 , 145 , 146 , 80 , 144 , 134 , 22 , 144 , 99 , 98 , 44 , 119 , 217 , 144 , 96 , 132 , 1 , 96 , 0 , 96 , 64 , 81 , 128 , 131 , 3 , 129 , 96 , 0 , 135 , 128 , 59 , 21 , 128 , 21 , 97 , 4 , 19 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 90 , 241 , 21 , 128 , 21 , 97 , 4 , 39 , 87 , 61 , 96 , 0 , 128 , 62 , 61 , 96 , 0 , 253 , 91 , 80 , 80 , 80 , 80 , 80 , 80 , 80 , 80 , 97 , 4 , 120 , 86 , 91 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 20 , 96 , 36 , 130 , 1 , 82 , 115 , 73 , 110 , 118 , 97 , 108 , 105 , 100 , 32 , 102 , 117 , 110 , 99 , 116 , 105 , 111 , 110 , 32 , 115 , 105 , 103 , 96 , 96 , 27 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 91 , 96 , 64 , 81 , 128 , 145 , 3 , 144 , 253 , 91 , 80 , 80 , 80 , 80 , 80 , 80 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 84 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 22 , 51 , 20 , 97 , 4 , 221 , 87 , 96 , 64 , 81 , 98 , 70 , 27 , 205 , 96 , 229 , 27 , 129 , 82 , 96 , 32 , 96 , 4 , 130 , 1 , 82 , 96 , 30 , 96 , 36 , 130 , 1 , 82 , 127 , 115 , 101 , 110 , 100 , 101 , 114 , 32 , 109 , 117 , 115 , 116 , 32 , 98 , 101 , 32 , 98 , 114 , 105 , 100 , 103 , 101 , 32 , 99 , 111 , 110 , 116 , 114 , 97 , 99 , 116 , 0 , 0 , 96 , 68 , 130 , 1 , 82 , 96 , 100 , 1 , 97 , 4 , 111 , 86 , 91 , 86 , 91 , 128 , 53 , 96 , 1 , 96 , 1 , 96 , 160 , 27 , 3 , 129 , 22 , 129 , 20 , 97 , 4 , 246 , 87 , 96 , 0 , 128 , 253 , 91 , 145 , 144 , 80 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 5 , 13 , 87 , 96 , 0 , 128 , 253 , 91 , 97 , 5 , 22 , 130 , 97 , 4 , 223 , 86 , 91 , 147 , 146 , 80 , 80 , 80 , 86 , 91 , 96 , 0 , 96 , 32 , 130 , 132 , 3 , 18 , 21 , 97 , 5 , 47 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 53 , 145 , 144 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 64 , 131 , 133 , 3 , 18 , 21 , 97 , 5 , 73 , 87 , 96 , 0 , 128 , 253 , 91 , 130 , 53 , 145 , 80 , 97 , 5 , 89 , 96 , 32 , 132 , 1 , 97 , 4 , 223 , 86 , 91 , 144 , 80 , 146 , 80 , 146 , 144 , 80 , 86 , 91 , 96 , 0 , 128 , 96 , 0 , 96 , 64 , 132 , 134 , 3 , 18 , 21 , 97 , 5 , 119 , 87 , 96 , 0 , 128 , 253 , 91 , 131 , 53 , 146 , 80 , 96 , 32 , 132 , 1 , 53 , 103 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 128 , 130 , 17 , 21 , 97 , 5 , 150 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 134 , 1 , 145 , 80 , 134 , 96 , 31 , 131 , 1 , 18 , 97 , 5 , 170 , 87 , 96 , 0 , 128 , 253 , 91 , 129 , 53 , 129 , 129 , 17 , 21 , 97 , 5 , 185 , 87 , 96 , 0 , 128 , 253 , 91 , 135 , 96 , 32 , 130 , 133 , 1 , 1 , 17 , 21 , 97 , 5 , 203 , 87 , 96 , 0 , 128 , 253 , 91 , 96 , 32 , 131 , 1 , 148 , 80 , 128 , 147 , 80 , 80 , 80 , 80 , 146 , 80 , 146 , 80 , 146 , 86 , 91 , 96 , 0 , 128 , 133 , 133 , 17 , 21 , 97 , 5 , 238 , 87 , 96 , 0 , 128 , 253 , 91 , 131 , 134 , 17 , 21 , 97 , 5 , 251 , 87 , 96 , 0 , 128 , 253 , 91 , 80 , 80 , 130 , 1 , 147 , 145 , 144 , 146 , 3 , 145 , 80 , 86 , 91 , 107 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 25 , 129 , 53 , 129 , 129 , 22 , 145 , 96 , 20 , 133 , 16 , 21 , 97 , 6 , 53 , 87 , 128 , 129 , 134 , 96 , 20 , 3 , 96 , 3 , 27 , 27 , 131 , 22 , 22 , 146 , 80 , 91 , 80 , 80 , 146 , 145 , 80 , 80 , 86 , 91 , 128 , 53 , 96 , 32 , 131 , 16 , 21 , 97 , 6 , 86 , 87 , 96 , 0 , 25 , 96 , 32 , 132 , 144 , 3 , 96 , 3 , 27 , 27 , 22 , 91 , 146 , 145 , 80 , 80 , 86 , 91 , 96 , 1 , 96 , 1 , 96 , 224 , 27 , 3 , 25 , 129 , 53 , 129 , 129 , 22 , 145 , 96 , 4 , 133 , 16 , 21 , 97 , 6 , 53 , 87 , 96 , 4 , 148 , 144 , 148 , 3 , 96 , 3 , 27 , 132 , 144 , 27 , 22 , 144 , 146 , 22 , 146 , 145 , 80 , 80 , 86 , 254 , 162 , 100 , 105 , 112 , 102 , 115 , 88 , 34 , 18 , 32 , 250 , 82 , 97 , 157 , 172 , 167 , 178 , 176 , 154 , 245 , 180 , 96 , 151 , 7 , 141 , 57 , 136 , 102 , 217 , 171 , 198 , 248 , 184 , 132 , 175 , 252 , 31 , 131 , 60 , 116 , 148 , 238 , 100 , 115 , 111 , 108 , 99 , 67 , 0 , 8 , 5 , 0 , 51] ;
    #[doc = "The deployed bytecode of the contract."]
    pub static TREASURYHANDLERCONTRACT_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct TreasuryHandlerContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TreasuryHandlerContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TreasuryHandlerContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TreasuryHandlerContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TreasuryHandlerContract<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(TreasuryHandlerContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TreasuryHandlerContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers` client at"]
        #[doc = r" `address`. The contract derefs to a `ethers::Contract` object."]
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                TREASURYHANDLERCONTRACT_ABI.clone(),
                client,
            ))
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" - If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" - The default poll duration is 7 seconds."]
        #[doc = r" - The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter, "../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                TREASURYHANDLERCONTRACT_ABI.clone(),
                TREASURYHANDLERCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `_bridgeAddress` (0x318c136e) function"]
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
        #[doc = "Calls the contract's `_contractAddressToResourceID` (0xec97d3b4) function"]
        pub fn contract_address_to_resource_id(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([236, 151, 211, 180], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_contractWhitelist` (0x7f79bea8) function"]
        pub fn contract_whitelist(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([127, 121, 190, 168], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_resourceIDToContractAddress` (0xc54c2a11) function"]
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
        #[doc = "Calls the contract's `executeProposal` (0xe248cff2) function"]
        pub fn execute_proposal(
            &self,
            resource_id: [u8; 32],
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 72, 207, 242], (resource_id, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `migrateBridge` (0xd7f5b359) function"]
        pub fn migrate_bridge(
            &self,
            new_bridge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 245, 179, 89], new_bridge)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setResource` (0xb8fa3736) function"]
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
        From<::ethers::contract::Contract<M>> for TreasuryHandlerContract<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[doc = "Container type for all input parameters for the `_bridgeAddress` function with signature `_bridgeAddress()` and selector `0x318c136e`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "_bridgeAddress", abi = "_bridgeAddress()")]
    pub struct BridgeAddressCall;
    #[doc = "Container type for all input parameters for the `_contractAddressToResourceID` function with signature `_contractAddressToResourceID(address)` and selector `0xec97d3b4`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
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
    #[doc = "Container type for all input parameters for the `_contractWhitelist` function with signature `_contractWhitelist(address)` and selector `0x7f79bea8`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "_contractWhitelist", abi = "_contractWhitelist(address)")]
    pub struct ContractWhitelistCall(pub ::ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `_resourceIDToContractAddress` function with signature `_resourceIDToContractAddress(bytes32)` and selector `0xc54c2a11`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
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
    #[doc = "Container type for all input parameters for the `executeProposal` function with signature `executeProposal(bytes32,bytes)` and selector `0xe248cff2`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
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
    #[doc = "Container type for all input parameters for the `migrateBridge` function with signature `migrateBridge(address)` and selector `0xd7f5b359`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
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
    #[doc = "Container type for all input parameters for the `setResource` function with signature `setResource(bytes32,address)` and selector `0xb8fa3736`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
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
    #[doc = "Container type for all of the contract's call "]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum TreasuryHandlerContractCalls {
        BridgeAddress(BridgeAddressCall),
        ContractAddressToResourceID(ContractAddressToResourceIDCall),
        ContractWhitelist(ContractWhitelistCall),
        ResourceIDToContractAddress(ResourceIDToContractAddressCall),
        ExecuteProposal(ExecuteProposalCall),
        MigrateBridge(MigrateBridgeCall),
        SetResource(SetResourceCall),
    }
    impl ::ethers::core::abi::AbiDecode for TreasuryHandlerContractCalls {
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
            if let Ok (decoded) = < ContractAddressToResourceIDCall as :: ethers :: core :: abi :: AbiDecode > :: decode (data) { return Ok (Self :: ContractAddressToResourceID (decoded)) }
            if let Ok (decoded) = < ContractWhitelistCall as :: ethers :: core :: abi :: AbiDecode > :: decode (data) { return Ok (Self :: ContractWhitelist (decoded)) }
            if let Ok (decoded) = < ResourceIDToContractAddressCall as :: ethers :: core :: abi :: AbiDecode > :: decode (data) { return Ok (Self :: ResourceIDToContractAddress (decoded)) }
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
    impl ::ethers::core::abi::AbiEncode for TreasuryHandlerContractCalls {
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
    impl ::core::fmt::Display for TreasuryHandlerContractCalls {
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
    impl ::core::convert::From<BridgeAddressCall> for TreasuryHandlerContractCalls {
        fn from(value: BridgeAddressCall) -> Self {
            Self::BridgeAddress(value)
        }
    }
    impl ::core::convert::From<ContractAddressToResourceIDCall>
        for TreasuryHandlerContractCalls
    {
        fn from(value: ContractAddressToResourceIDCall) -> Self {
            Self::ContractAddressToResourceID(value)
        }
    }
    impl ::core::convert::From<ContractWhitelistCall>
        for TreasuryHandlerContractCalls
    {
        fn from(value: ContractWhitelistCall) -> Self {
            Self::ContractWhitelist(value)
        }
    }
    impl ::core::convert::From<ResourceIDToContractAddressCall>
        for TreasuryHandlerContractCalls
    {
        fn from(value: ResourceIDToContractAddressCall) -> Self {
            Self::ResourceIDToContractAddress(value)
        }
    }
    impl ::core::convert::From<ExecuteProposalCall>
        for TreasuryHandlerContractCalls
    {
        fn from(value: ExecuteProposalCall) -> Self {
            Self::ExecuteProposal(value)
        }
    }
    impl ::core::convert::From<MigrateBridgeCall> for TreasuryHandlerContractCalls {
        fn from(value: MigrateBridgeCall) -> Self {
            Self::MigrateBridge(value)
        }
    }
    impl ::core::convert::From<SetResourceCall> for TreasuryHandlerContractCalls {
        fn from(value: SetResourceCall) -> Self {
            Self::SetResource(value)
        }
    }
    #[doc = "Container type for all return fields from the `_bridgeAddress` function with signature `_bridgeAddress()` and selector `0x318c136e`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BridgeAddressReturn(pub ::ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `_contractAddressToResourceID` function with signature `_contractAddressToResourceID(address)` and selector `0xec97d3b4`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ContractAddressToResourceIDReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `_contractWhitelist` function with signature `_contractWhitelist(address)` and selector `0x7f79bea8`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ContractWhitelistReturn(pub bool);
    #[doc = "Container type for all return fields from the `_resourceIDToContractAddress` function with signature `_resourceIDToContractAddress(bytes32)` and selector `0xc54c2a11`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
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
