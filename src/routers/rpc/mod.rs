mod function_id;
mod function_set;
mod path;

pub use function_id::*;
pub use function_set::*;
pub use path::*;

// Base on SceneRPCInterface::get_rpc_md5
pub fn hash_function_set(names: &[String]) -> String {
    let mut hash_context = md5::Context::new();

    // Alphabetical order as per SceneRPCInterface::_parse_rpc_config
    let mut names = names.to_vec();
    names.sort();
    for name in names {
        hash_context.consume(name);
    }

    let result = hash_context.finalize();

    format!("{:x}", result)
}

pub fn get_name_id(name: &String, names: &[String]) -> u32 {
    // Alphabetical order as per SceneRPCInterface::_parse_rpc_config
    let mut names = names.to_vec();
    names.sort();

    return names
        .iter()
        .position(|n| n == name)
        .map(|pos| pos as u32)
        .unwrap_or(u32::MAX);
}

pub const fn sort_names<const N: usize>(names: [&str; N]) -> [&str; N] {
    let mut sorted_names = names;
    let mut i = 0;
    while i < N {
        let mut j = i + 1;
        while j < N {
            if {
                let mut k = 0;
                let mut less = false;
                while k < sorted_names[j].len() && k < sorted_names[i].len() {
                    if sorted_names[j].as_bytes()[k] < sorted_names[i].as_bytes()[k] {
                        less = true;
                        break;
                    } else if sorted_names[j].as_bytes()[k] > sorted_names[i].as_bytes()[k] {
                        break;
                    }
                    k += 1;
                }
                less || (k == sorted_names[j].len()
                    && sorted_names[j].len() < sorted_names[i].len())
            } {
                let temp = sorted_names[i];
                sorted_names[i] = sorted_names[j];
                sorted_names[j] = temp;
            }
            j += 1;
        }
        i += 1;
    }
    sorted_names
}

/// Sort a list of names at compile time and return the sorted list.
#[macro_export]
macro_rules! sort_names {
    [$($name:expr),+] => {{
        $crate::routers::sort_names([$($name),+])
    }};
}

pub const fn find_name<const N: usize>(name: &str, names: [&str; N]) -> u32 {
    let mut index = 0;
    while index < N {
        let a = names[index].as_bytes();
        let b = name.as_bytes();
        if a.len() == b.len() && {
            let mut equal = true;
            let mut i = 0;
            while i < a.len() {
                if a[i] != b[i] {
                    equal = false;
                    break;
                }
                i += 1;
            }
            equal
        } {
            return index as u32;
        }
        index += 1;
    }
    u32::MAX
}

/// Get the ID of a name from a sorted list of names at compile time.
/// Returns u32::MAX if the name is not found.
///
/// NOTE: The names must be sorted in alphabetical order. You may use the [sort_names!](crate::sort_names) macro to sort them.
#[macro_export]
macro_rules! name_id {
    ($name:expr, $names:expr) => {{ $crate::routers::find_name($name, $names) }};
}

// TODO: Const Hashing
