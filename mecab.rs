/*!
MeCab bindings for Rust

Copyright (C) 2012 Tycho Sci
Copyright (C) 2013 Yegor Alexeyev

This binding is licensed under the same license of MeCab.
*/

#[link(name = "mecab",
       package_id = "mecab",
       vers = "0.2",
       uuid = "157601c8-818c-4898-b1dc-890eeeab4936",
       url  = "https://github.com/tychosci/rust-mecab")];

#[comment = "MeCab bindings for Rust"];
#[license = "GPL/LGPL/BSD"];
#[crate_type = "lib"];

#[allow(non_camel_case_types)];

use std::libc::types::os::arch::c95::c_char;
use std::libc::types::os::arch::c95::c_int;
use std::libc::types::os::arch::c95::c_float;
use std::libc::types::os::arch::c95::c_uint;
use std::libc::types::os::arch::c95::c_long;
use std::libc::types::os::arch::c95::size_t;

#[cfg(test)]
mod test;

// NB: Need to expand `mecab-config --libs-only-L` at link time
#[nolink]
#[link(name= "mecab")]
extern {
    fn mecab_new(argc: c_int, argv: **c_char) -> *mecab_t;
    fn mecab_new2(arg: *c_char) -> *mecab_t;
    fn mecab_destroy(mecab: *mecab_t);
    fn mecab_strerror(mecab: *mecab_t) -> *c_char;
    fn mecab_do(argc: c_int, argv: **c_char) -> c_int;
    fn mecab_sparse_tostr(mecab: *mecab_t, input: *c_char) -> *c_char;
    fn mecab_sparse_tostr2(mecab: *mecab_t, input: *c_char, len: size_t) -> *c_char;
    fn mecab_sparse_tonode(mecab: *mecab_t, input: *c_char) -> *mecab_node_t;
    fn mecab_sparse_tonode2(mecab: *mecab_t, input: *c_char, len: size_t) -> *mecab_node_t;
    fn mecab_parse_lattice(mecab: *mecab_t, lattice: *mecab_lattice_t) -> c_int;
    fn mecab_dictionary_info(mecab: *mecab_t) -> *mecab_dictionary_info_t;
    fn mecab_version() -> *c_char;

    fn mecab_model_new(argc: c_int, argv: **c_char) -> *mecab_model_t;
    fn mecab_model_new2(arg: *c_char) -> *mecab_model_t;
    fn mecab_model_new_tagger(model: *mecab_model_t) -> *mecab_t;
    fn mecab_model_new_lattice(model: *mecab_model_t) -> *mecab_lattice_t;
    fn mecab_model_destroy(model: *mecab_model_t);

    fn mecab_lattice_set_sentence2(lattice: *mecab_lattice_t, input: *c_char, len: size_t);
    fn mecab_lattice_tostr(lattice: *mecab_lattice_t) -> *c_char;
    fn mecab_lattice_get_size(lattice: *mecab_lattice_t) -> size_t;
    fn mecab_lattice_get_bos_node(lattice: *mecab_lattice_t) -> *mecab_node_t;
    fn mecab_lattice_get_eos_node(lattice: *mecab_lattice_t) -> *mecab_node_t;
    fn mecab_lattice_get_begin_nodes(lattice: *mecab_lattice_t, pos: size_t) -> *mecab_node_t;
    fn mecab_lattice_get_end_nodes(lattice: *mecab_lattice_t, pos: size_t) -> *mecab_node_t;
    fn mecab_lattice_destroy(lattice: *mecab_lattice_t);
    fn mecab_lattice_strerror(lattice: *mecab_lattice_t) -> *c_char;
}

struct mecab_t;

struct mecab_model_t;

struct mecab_lattice_t;

/**
Same structure of `mecab::mecab_path_t` that documented in
<http://mecab.sourceforge.net/doxygen/structmecab__path__t.html>
*/
struct mecab_path_t {
    priv rnode: *mecab_node_t,
    priv rnext: *mecab_path_t,
    priv lnode: *mecab_node_t,
    priv lnext: *mecab_path_t,
    priv cost:   c_int,
    priv prob:   c_float,
}

/**
Same structure of `mecab::mecab_node_t` that documented in
<http://mecab.sourceforge.net/doxygen/structmecab__node__t.html>
*/
struct mecab_node_t {
    priv prev:      *mecab_node_t,
    priv next:      *mecab_node_t,
    priv enext:     *mecab_node_t,
    priv bnext:     *mecab_node_t,
    priv rpath:     *mecab_path_t,
    priv lpath:     *mecab_path_t,
    priv surface:   *c_char,
    priv feature:   *c_char,
    priv id:         c_uint,
    priv length:     u16,
    priv rlength:    u16,
    priv rcAttr:     u16,
    priv lcAttr:     u16,
    priv posid:      u16,
    priv char_type:  u8,
    priv stat:       u8,
    priv isbest:     u8,
    priv alpha:      c_float,
    priv beta:       c_float,
    priv prob:       c_float,
    priv wcost:      i16,
    priv cost:       c_long,
}

/**
Same structure of `mecab::mecab_dictionary_info_t` that documented in
<http://mecab.sourceforge.net/doxygen/structmecab__dictionary__info__t.html>
*/
struct mecab_dictionary_info_t {
    priv filename: *c_char,
    priv charset:  *c_char,
    priv size:      c_uint,
    priv ty:        c_int,
    priv lsize:     c_uint,
    priv rsize:     c_uint,
    priv version:   u16,
    priv next:     *mecab_dictionary_info_t,
}

/**
Parameters for `mecab_node_t.stat` Normal node
defined in the dictionary.
*/
pub static NOR_NODE: u8 = 0u8;

/**
Parameters for `mecab_node_t.stat` Unknown node
not defined in the dictionary.
*/
pub static UNK_NODE: u8 = 1u8;

/**
Parameters for `mecab_node_t.stat` Virtual node
representing a beginning of the sentence.
*/
pub static BOS_NODE: u8 = 2u8;

/**
Parameters for `mecab_node_t.stat` Virtual node
representing a end of the sentence.
*/
pub static EOS_NODE: u8 = 3u8;

/**
Parameters for `mecab_node_t.stat` Virtual node
representing a end of the N-best enumeration.
*/
pub static EON_NODE: u8 = 4u8;

/// Wrapped structure for `mecab_dictionary_info_t`.
pub struct DictionaryInfo {
    priv dict: *mecab_dictionary_info_t
}

pub struct DictionaryInfoIterator {
    priv position: *mecab_dictionary_info_t
}


impl Iterator<DictionaryInfo> for DictionaryInfoIterator {
    fn next(&mut self) -> Option<DictionaryInfo> {
        Some( DictionaryInfo { dict: self.position } )
    }
}

impl DictionaryInfo {
    pub fn iter(&self) -> DictionaryInfoIterator {
        DictionaryInfoIterator { position: self.dict }
    }
}

//TODO TaggerNode and LatticeNode should expose the common interface

pub struct TaggerNode<'owner, 'tagger_owner> {
    priv owner: &'owner Tagger<'tagger_owner>, 
    priv node: *mecab_node_t
}

pub struct LatticeNode<'owner, 'lattice_owner> {
    priv owner: &'owner Lattice<'lattice_owner>, 
    priv node: *mecab_node_t
}




pub struct NodeIterator {
    priv position: *mecab_node_t
}


impl Iterator<*mecab_node_t> for NodeIterator {
    fn next(&mut self) -> Option<*mecab_node_t> {
        if self.position.is_not_null() {
          let current_position = self.position;
          unsafe {
              self.position=(*self.position).next;
          }
          Some( current_position )
        } else {
          None
        }
        
        //Some( Node { node: self.position } )
    }
}

impl<'owner, 'tagger_owner> TaggerNode<'owner, 'tagger_owner> {
    pub fn iter(&self) -> NodeIterator {
        NodeIterator { position: self.node }
    }
}

impl<'owner, 'tagger_owner> LatticeNode<'owner, 'tagger_owner> {
    pub fn iter(&self) -> NodeIterator {
        NodeIterator { position: self.node }
    }
}


///
/// Wrapped structure for `mecab_t`.
pub struct Tagger<'owner> {
    priv owner: Option<&'owner Model>,
    priv mecab: *mecab_t
}

/// Wrapped structure for `mecab_model_t`.
pub struct Model {
    priv model: *mecab_model_t,
}

/// Wrapped structure for `mecab_lattice_t`.
pub struct Lattice<'owner> {
    priv owner: &'owner Model,
    lattice: *mecab_lattice_t
}


impl Drop for DictionaryInfo {
    fn drop(&mut self) {}
}

#[unsafe_destructor]
impl<'owner, 'tagger_owner> Drop for TaggerNode<'owner, 'tagger_owner> {
    fn drop(&mut self) {}
}

#[unsafe_destructor]
impl<'owner, 'tagger_owner> Drop for LatticeNode<'owner, 'tagger_owner> {
    fn drop(&mut self) {}
}

#[unsafe_destructor]
impl<'owner> Drop for Tagger<'owner> {
    fn drop(& mut self) {
        unsafe { mecab_destroy(self.mecab); }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe { mecab_model_destroy(self.model); }
    }
}

#[unsafe_destructor]
impl<'owner> Drop for Lattice<'owner> {
    fn drop(&mut self) {
        unsafe { mecab_lattice_destroy(self.lattice); }
    }
}

pub trait IDict {
    fn get_filename(&self) -> ~str;
    fn get_charset(&self) -> ~str;
    fn get_size(&self) -> uint;
    fn get_type(&self) -> int;
    fn get_lsize(&self) -> uint;
    fn get_rsize(&self) -> uint;
    fn get_version(&self) -> uint;
}

pub trait INode {
    fn get_surface<'v>(&'v self) -> &'v str;
    fn get_feature(&self) -> ~str;
    fn get_status(&self) -> u8;
    fn get_posid(&self) -> u16;
    fn get_prob(&self) -> c_float;

    fn is_best(&self) -> bool;
}

impl IDict for mecab_dictionary_info_t {
    /// Returns `mecab_dictionary_info_t.filename`.
    fn get_filename(&self) -> ~str {
        unsafe { std::str::raw::from_c_str(self.filename) }
    }

    /// Returns `mecab_dictionary_info_t.charset`.
    fn get_charset(&self) -> ~str {
        unsafe { std::str::raw::from_c_str(self.charset) }
    }

    /// Returns `mecab_dictionary_info_t.size`.
    fn get_size(&self) -> uint {
        self.size as uint
    }

    /// Returns `mecab_dictionary_info_t.type`.
    fn get_type(&self) -> int {
        self.ty as int
    }

    /// Returns `mecab_dictionary_info_t.lsize`.
    fn get_lsize(&self) -> uint {
        self.lsize as uint
    }

    /// Returns `mecab_dictionary_info_t.rsize`.
    fn get_rsize(&self) -> uint {
        self.rsize as uint
    }

    /// Returns `mecab_dictionary_info_t.version`.
    fn get_version(&self) -> uint {
        self.version as uint
    }
}

impl INode for mecab_node_t {
    fn get_surface<'v>(&'v self) -> &'v str {
        unsafe {
            let s = std::vec::raw::buf_as_slice(self.surface as *u8, self.length as uint, |v| { std::cast::transmute(v) });
            return std::str::from_utf8(s);
        }
    }

    /// Returns `mecab_node_t.feature`.
    fn get_feature(&self) -> ~str {
        unsafe { std::str::raw::from_c_str(self.feature) }
    }

    /// Returns `mecab_node_t.status`.
    fn get_status(&self) -> u8 {
        self.stat
    }

    /// Returns `mecab_node_t.posid`.
    fn get_posid(&self) -> u16 {
        self.posid
    }

    /// Returns `mecab_node_t.prob`.
    fn get_prob(&self) -> c_float {
        self.prob
    }

    fn is_best(&self) -> bool {
        self.isbest == 1
    }
}
/*
impl BaseIter<mecab_dictionary_info_t> for DictionaryInfo {
    fn size_hint(&self) -> Option<uint> { None }

    fn each(&self, blk: &fn(&mecab_dictionary_info_t) -> bool) {
        let mut p = self.dict;

        while p.is_not_null() {
            if !blk(unsafe { cast::transmute(p) }) { break; }
            unsafe { p = (*p).next; }
        }
    }
}

impl BaseIter<mecab_node_t> for Node {
    fn size_hint(&self) -> Option<uint> { None }

    fn each(&self, blk: &fn(&mecab_node_t) -> bool) {
        let mut p = self.node;

        while p.is_not_null() {
            if !blk(unsafe { cast::transmute(p) }) { break; }
            unsafe { p = (*p).next; }
        }
    }
}
*/

impl<'owner> Tagger<'owner> {

    /// The wrapper of `mecab::mecab_new` that may return `Tagger`.
    pub fn new(args: &[~str]) -> Tagger {
        let argc = args.len() as c_int;

        let mut argptrs = ~[];
        let mut tmps = ~[];

        for arg in args.iter() {
            //let t = @copy *arg;
            let t = (*arg).clone();
            tmps.push(t.clone());
            argptrs.push(t.to_c_str().with_ref(|b| b));
        }
        argptrs.push(std::ptr::null());

        let mecab = unsafe { mecab_new( argc, argptrs.as_ptr() ) };

        if mecab.is_null() {
            fail!(~"failed to create new instance");
        } else {
            Tagger { mecab: mecab, owner: None }
        }
    }

    /// The wrapper of `mecab::mecab_new2` that may return `Tagger`.
    pub fn new2(arg: &str) -> Tagger {
        let mecab = arg.to_c_str().with_ref( |buf| {
            unsafe {
                mecab_new2(buf)
            }
        });

        if mecab.is_null() {
            fail!(~"failed to create new instance");
        } else {
            Tagger { mecab: mecab, owner: None }
        }
    }

    /// Parses input and may return the string of result.
    fn parse(&self, input: &str) -> ~str {
        let s = input.to_c_str().with_ref(|buf| {
            unsafe {
                mecab_sparse_tostr(self.mecab, buf)
            }
        });

        if s.is_null() {
            let msg = self.strerror();
            fail!(msg);
        } else {
            unsafe { std::str::raw::from_c_str(s) }
        }
    }

    /// Parses input and returns `Node`.
    pub fn parse_to_node<'tagger>(&'tagger self, input: &str) -> TaggerNode<'tagger, 'owner> {
        unsafe {
            let node = mecab_sparse_tonode2(self.mecab, input.as_bytes().as_ptr() as *c_char, input.len() as u64 );
            if node.is_null() {
                let msg = self.strerror();
                fail!(msg);
            } else {
                TaggerNode { owner: self, node: node }
            }
        }
    }

    /// Parses input in given `lattice` and returns true on success.
    pub fn parse_lattice(&self, lattice: &Lattice) -> bool {
        unsafe {
            let status = mecab_parse_lattice(self.mecab, lattice.lattice);
            status != 0 as c_int
        }
    }

    /// Returns `DictionaryInfo`.
    fn get_dictionary_info(&self) -> DictionaryInfo {
        unsafe {
            let dict = mecab_dictionary_info(self.mecab);

            if dict.is_null() {
                let msg = self.strerror();
                fail!(msg);
            } else {
                DictionaryInfo { dict: dict }
            }
        }
    }

    fn strerror(&self) -> ~str {
        unsafe {
            let s = mecab_strerror(self.mecab);
            std::str::raw::from_c_str(s)
        }
    }
}

impl Model {

    /**

    The wrapper of `mecab::mecab_model_new` that
    may return `Model`.
    */
    pub fn new(args: &[~str]) -> Model {
        let argc = args.len() as c_int;

        let mut argptrs = ~[];
        let mut tmps = ~[];

        for arg in args.iter() {
            let t = (*arg).clone();
            tmps.push(t.clone());
    //TODO I'm not sure
            argptrs.push(t.to_c_str().with_ref( |b| {b}) );
        }
        argptrs.push(std::ptr::null());

        let model = unsafe { mecab_model_new( argc, argptrs.as_ptr() ) };

        if model.is_null() {
            fail!(~"failed to create new Model");
        } else {
            Model { model: model}
        }
    }

    /**
    The wrapper of `mecab::mecab_model_new2` that
    may return `Model`.
    */
    pub fn new2(arg: &str) -> Model {
        let model = arg.to_c_str().with_ref(|buf| {
            unsafe {
                mecab_model_new2(buf)
            }
        });

        if model.is_null() {
            fail!(~"failed to create new Model");
        } else {
            Model { model: model}
        }
    }

    /// Creates new tagger.
    pub fn create_tagger<'model>(&'model self) -> Tagger<'model> {
        unsafe {
            let mecab = mecab_model_new_tagger(self.model);

            if mecab.is_null() {
                fail!(~"failed to create new Tagger");
            } else {
                Tagger {owner: Some(self), mecab: mecab}
            }
        }
    }

    /// Creates new lattice.
    pub fn create_lattice<'model>(&'model self) -> Lattice<'model> {
        unsafe {
            let lattice = mecab_model_new_lattice(self.model);

            if lattice.is_null() {
                fail!(~"failed to create new Lattice");
            } else {
                Lattice { owner: self, lattice: lattice }
            }
        }
    }
}

impl<'owner> ToStr for Lattice<'owner> {
    fn to_str(&self) -> ~str {
        unsafe {
            let s = mecab_lattice_tostr(self.lattice);
            std::str::raw::from_c_str(s)
        }
    }
}

impl<'owner> Lattice<'owner> {
    /// Set input of the lattice.
    pub fn set_sentence(&self, input: &str) {
        unsafe {
            let bytes = input;
            //let bytes = input.as_owned_vec();
            mecab_lattice_set_sentence2(self.lattice, bytes.as_ptr() as *c_char, bytes.len() as size_t); 
        }
    }

    /// Returns the beginning node of the sentence on success.
    pub fn get_bos_node<'lattice>(&'lattice self) -> LatticeNode<'lattice, 'owner> {
        unsafe {
            let node = mecab_lattice_get_bos_node(self.lattice);

            if node.is_null() {
                let msg = self.strerror();
                fail!(msg);
            } else {
                LatticeNode { owner: self, node: node }
            }
        }
    }

    /// Returns the end node of the sentence on success.
    fn get_eos_node<'lattice>(&'lattice self) -> LatticeNode<'lattice, 'owner> {
        unsafe {
            let node = mecab_lattice_get_eos_node(self.lattice);

            if node.is_null() {
                let msg = self.strerror();
                fail!(msg);
            } else {
                LatticeNode { owner: self, node: node }
            }
        }
    }

    fn strerror(&self) -> ~str {
        unsafe {
            let s = mecab_lattice_strerror(self.lattice);
            std::str::raw::from_c_str(s)
        }
    }
}



/**
The wrapper of `mecab::mecab_version` that
returns version-number string.
*/
pub fn version() -> ~str {
    unsafe {
        let vers = mecab_version();
        std::str::raw::from_c_str(vers)
    }
}
