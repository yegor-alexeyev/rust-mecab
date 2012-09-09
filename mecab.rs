//
// mecab.rs - The implementation of MeCab bindings for Rust.
//
// Copyright (C) 2012 Tycho Sci.
//
// This binding is licensed under the same license of MeCab.
//

use str::unsafe;
use libc::*;

#[allow(non_camel_case_types)]
priv enum mecab_t = ();

#[allow(non_camel_case_types)]
/// Same structure of `mecab::mecab_path_t` that documented in
/// <http://mecab.sourceforge.net/doxygen/structmecab__path__t.html>
priv enum mecab_path_t = {
    rnode: *mecab_node_t,
    rnext: *mecab_path_t,
    lnode: *mecab_node_t,
    lnext: *mecab_path_t,
    cost:   c_int,
    prob:   c_float,
};

#[allow(non_camel_case_types)]
/// Same structure of `mecab::mecab_node_t` that documented in
/// <http://mecab.sourceforge.net/doxygen/structmecab__node__t.html>
priv enum mecab_node_t = {
    prev:      *mecab_node_t,
    next:      *mecab_node_t,
    enext:     *mecab_node_t,
    bnext:     *mecab_node_t,
    rpath:     *mecab_path_t,
    lpath:     *mecab_path_t,
    surface:   *c_char,
    feature:   *c_char,
    id:         c_uint,
    length:     u16,
    rlength:    u16,
    rcAttr:     u16,
    lcAttr:     u16,
    posid:      u16,
    char_type:  u8,
    stat:       u8,
    isbest:     u8,
    alpha:      c_float,
    beta:       c_float,
    prob:       c_float,
    wcost:      i16,
    cost:       c_long,
};

#[allow(non_camel_case_types)]
/// Same structure of `mecab::mecab_dictionary_info_t` that documented in
/// <http://mecab.sourceforge.net/doxygen/structmecab__dictionary__info__t.html>
priv enum mecab_dictionary_info_t = {
    filename: *c_char,
    charset:  *c_char,
    size:      c_uint,
    type:      c_int,
    lsize:     c_uint,
    rsize:     c_uint,
    version:   u16,
    next:     *mecab_dictionary_info_t,
};

/// Wrapped structure for `mecab_dictionary_info_t`.
struct MeCabDictionaryInfo {
    priv dict: *mecab_dictionary_info_t,
}

/// Wrapped structure for `mecab_node_t`.
struct MeCabNode {
    priv node: *mecab_node_t,
}

/// Wrapped structure for `mecab_t`.
struct MeCab {
    priv mecab: *mecab_t,
    drop { mecab::mecab_destroy(self.mecab); }
}

impl *mecab_dictionary_info_t {
    /// Returns `mecab_dictionary_info_t.filename`.
    pure fn get_filename() -> ~str { unsafe { unsafe::from_c_str((*self).filename) } }
    /// Returns `mecab_dictionary_info_t.charset`.
    pure fn get_charset()  -> ~str { unsafe { unsafe::from_c_str((*self).charset)  } }
    /// Returns `mecab_dictionary_info_t.size`.
    pure fn get_size()     -> uint { unsafe { (*self).size    as uint } }
    /// Returns `mecab_dictionary_info_t.type`.
    pure fn get_type()     ->  int { unsafe { (*self).type    as  int } }
    /// Returns `mecab_dictionary_info_t.lsize`.
    pure fn get_lsize()    -> uint { unsafe { (*self).lsize   as uint } }
    /// Returns `mecab_dictionary_info_t.rsize`.
    pure fn get_rsize()    -> uint { unsafe { (*self).rsize   as uint } }
    /// Returns `mecab_dictionary_info_t.version`.
    pure fn get_version()  -> uint { unsafe { (*self).version as uint } }
}

impl *mecab_node_t {
    /// Returns pre-sliced `mecab_node_t.surface`.
    pure fn get_surface() -> ~str {
        unsafe {
            let s = unsafe::from_c_str((*self).surface);
            str::slice(s, 0, (*self).length as uint)
        }
    }
    /// Returns `mecab_node_t.feature`.
    pure fn get_feature() -> ~str {
        unsafe { unsafe::from_c_str((*self).feature) }
    }
    /// Returns `mecab_node_t.status`.
    pure fn get_status() -> u8 {
        unsafe { (*self).stat }
    }
}

impl MeCabDictionaryInfo {
    /// Iterates all listed items on `mecab_dictionary_info_t`.
    fn each(blk: fn(*mecab_dictionary_info_t) -> bool) {
        let mut p = self.dict;

        while p.is_not_null() {
            if !blk(p) { break; }
            unsafe { p = (*p).next; }
        }
    }
}

impl MeCabNode {
    /// Iterates all listed items on `mecab_node_t`.
    fn each(blk: fn(*mecab_node_t) -> bool) {
        let mut p = self.node;

        while p.is_not_null() {
            if !blk(p) { break; }
            unsafe { p = (*p).next; }
        }
    }
}

impl MeCab {
    /// Parses input and may return the string of result.
    fn parse(input: &str) -> Result<~str, ~str> {
        let s = str::as_c_str(input, |buf| {
            mecab::mecab_sparse_tostr(self.mecab, buf)
        });

        if s.is_null() {
            let msg = self.strerror();
            Err(msg)
        } else {
            Ok(unsafe { unsafe::from_c_str(s) })
        }
    }
    /// Parses input and may return `MeCabNode`.
    fn parse_to_node(input: &str) -> Result<@MeCabNode, ~str> {
        let node = str::as_c_str(input, |buf| {
            mecab::mecab_sparse_tonode(self.mecab, buf)
        });

        if node.is_null() {
            let msg = self.strerror();
            Err(msg)
        } else {
            Ok(@MeCabNode{node: node})
        }
    }
    /// Returns `MeCabDictionaryInfo`.
    fn get_dictionary_info() -> Result<@MeCabDictionaryInfo, ~str> {
        let dict = mecab::mecab_dictionary_info(self.mecab);

        if dict.is_null() {
            let msg = self.strerror();
            Err(msg)
        } else {
            Ok(@MeCabDictionaryInfo{dict: dict})
        }
    }
    priv fn strerror() -> ~str {
        let s = mecab::mecab_strerror(self.mecab);
        unsafe { unsafe::from_c_str(s) }
    }
}

/// The wrapper of `mecab::mecab_new` that may return `MeCab`.
fn new(args: &[&str]) -> Result<@MeCab, ~str> {
    let argc = args.len() as c_int;

    let mut argptrs = ~[];
    let mut tmps    = ~[];

    for args.each |arg| {
        let t = @arg;
        vec::push(tmps, t);
        vec::push_all(argptrs, str::as_c_str(*t, |b| ~[b]));
    }
    vec::push(argptrs, ptr::null());

    let mecab = vec::as_buf(argptrs, |argv, _len| {
        mecab::mecab_new(argc, argv)
    });

    if mecab.is_null() {
        Err(~"unavailable to create new instance")
    } else {
        Ok(@MeCab{mecab: mecab})
    }
}

/// The wrapper of `mecab::mecab_new2` that may return `MeCab`.
fn new2(arg: &str) -> Result<@MeCab, ~str> {
    let mecab = str::as_c_str(arg, |buf| mecab::mecab_new2(buf));

    if mecab.is_null() {
        Err(~"unavailable to create new instance")
    } else {
        Ok(@MeCab{mecab: mecab})
    }
}

/// The wrapper of `mecab::mecab_version` that returns version-number string.
fn version() -> ~str {
    let vers = mecab::mecab_version();

    unsafe { unsafe::from_c_str(vers) }
}

/// Parameters for `mecab_node_t.stat` Normal node
/// defined in the dictionary.
const NOR_NODE: u8 = 0u8;

/// Parameters for `mecab_node_t.stat` Unknown node
/// not defined in the dictionary.
const UNK_NODE: u8 = 1u8;

/// Parameters for `mecab_node_t.stat` Virtual node
/// representing a beginning of the sentence.
const BOS_NODE: u8 = 2u8;

/// Parameters for `mecab_node_t.stat` Virtual node
/// representing a end of the sentence.
const EOS_NODE: u8 = 3u8;

/// Parameters for `mecab_node_t.stat` Virtual node
/// representing a end of the N-best enumeration.
const EON_NODE: u8 = 4u8;

// NB: Need to expand `mecab-config --libs-only-L` at linking time
extern mod mecab {
    fn mecab_new(argc: c_int, argv: **c_char) -> *mecab_t;
    fn mecab_new2(arg: *c_char) -> *mecab_t;
    fn mecab_destroy(mecab: *mecab_t);
    fn mecab_strerror(mecab: *mecab_t) -> *c_char;
    fn mecab_do(argc: c_int, argv: **c_char) -> c_int;
    fn mecab_sparse_tostr(mecab: *mecab_t, input: *c_char) -> *c_char;
    fn mecab_sparse_tostr2(mecab: *mecab_t, input: *c_char, len: size_t) -> *c_char;
    fn mecab_sparse_tonode(mecab: *mecab_t, input: *c_char) -> *mecab_node_t;
    fn mecab_sparse_tonode2(mecab: *mecab_t, input: *c_char, len: size_t) -> *mecab_node_t;
    fn mecab_dictionary_info(mecab: *mecab_t) -> *mecab_dictionary_info_t;
    fn mecab_version() -> *c_char;
}
