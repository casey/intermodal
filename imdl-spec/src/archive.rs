//! The Intermodal Archive Format
//! =============================
//!
//! The Intermodal archive format, or imdl-archive, is a general-purpose archive
//! format. This document contains a non-normative introduction to the format,
//! followed by a detailed specificiation.
//!
//! Introduction
//! ------------
//!
//! One of the primary motivating use-cases for the Intermodal archive format is
//! to serve as an improved replacement for simple file verification files, also
//! known as SFV files.
//!
//! This use case is described first, to motivate the format.
//!
//! ### Improvements over SFV Files
//!
//! SFV files, usually seen with the `.sfv` file extension, are packaged
//! alongside other files intended for distribution, and allow corruption in
//! those files to be detected.
//!
//! SFV files include CRC32 checksums. Although CRC32 is fast to compute, it is
//! not a cryptographic hash function. Thus, SFV files can only protect against
//! accidental corruption, and not a malicious attacker. It is computationally
//! feasible for an attacker to produce a file with semi-arbitrary contents, but
//! which matches a desired CRC32 checksum.
//!
//! The Intermodal archive format has a number of improvements over SFV files.
//!
//! Future additions that provide functionality above and beyond file
//! verification will not interfere with the SFV-replacement use-case.
//!
//! If all you need is simple file verification, you shouldn't be concerned that
//! future upgrades will break you use case.
//!
//! #### Manifest
//!
//! Although imdl-archive is a general purpose archive format, supporting
//! embedded files, it also supports usage as a manfest, where files are not
//! embedded inside the archive file, but instead distributed alongside it.
//!
//! This allows an imdl-archive file to be a drop-in replacement, or complement,
//! to SFV files.
//!
//! It is not required that the files for distribution be packed inside the
//! archive file, so disruption to existing workflows and expections
//! around directory structure and file naming is avoided.
//!
//! When used as manifest, the imdl archive file is named `manifest.idml`, and
//! distributed in the root directory of the collection of files for release.
//!
//! #### Fast Cryptographic Hashes
//!
//! The Intermodal archive format uses the [Blake3](https://blake3.io)
//! hash function to protect against data corruption. Blake3 is a a
//! cryptographic hash function with a number of useful properties that make it
//! a good choice for file verification.
//!
//! First, it is extremely fast. When both are implemented in software, Blake3
//! is much faster than even CRC32.
//!
//! Second, it is a tree hash. Blake3 is structured as a binary Merkle tree with
//! 1 KiB leaf nodes. This opens up a number of exciting possibilities:
//!
//! - Secure random access: Data in the middle of a file can be verified and
//!   retrieved without needing to verify the whole file from the beginning.
//!
//! - Secure streaming: Verified blocks can be streamed, without needing to
//!   verify the whole file.
//!
//! - Corruption detection and repair: Blocks damaged by bit flips during
//!   storage or transfer can be identified and re-downloaded, without needing
//!   to throw away or re-download the whole file.
//!
//! - Download resumption: File downloads can continue if interrupted, without
//!   the possibility of corruption.
//!
//! - Piecewise file synchronization: Files can be synced piece-by-piece over
//!   the network, either from a single or multiple sources.
//!
//! #### Digital Signature Friendly
//!
//! The Intermodal archive format is designed to be used with digital
//! signatures. It is possible to digitally sign a collection of files, and
//! embed that signature within the archive for later verification.
//!
//! If the public key of the archive author is known to the recipient, they may
//! verify not just the integrity of the archive, but also its provenance,
//! regardless of the source from which it was obtained.
//!
//! #### Extensible
//!
//! The Intermodal archive format is designed to be highly extensible. Future
//! enhancements to the format can be easily integrated in a
//! backwards-compatible fashion.
//!
//! ### Why design a new archive format?
//!
//! - control of hash coverage
//! - highly extensible
//! - hash-linked
//!
//! - can derive an internal cryptographic hash of the contents and metadata, so
//!   that signatures can be included in the archive (this must be separate from
//!   the hash of the overall archive, since otherwise the addition of
//!   signatures would change the hash the signatures were over, and thus
//!   invalidate the signatures.
//!
//! Hashes:
//! - designed so the archive itself and sub-objects have stable hashes
//!
//! ### why not git?
//! - git doesn't use a tree hash, which is a requirement for performance,
//!   incremental sync, and syncing a file from multiple sources
//! - git has no single-file archive format
//! - git signatures are embedded in objects, making it impossible to sign
//!   commits after the fact, or attach multiple signatures to commits
//! - however, even though it isn't git, it could be translated to git, worked
//!   on, and translated back
//!
//! ### Implementation Strategy
//!
//! This specification is provided for discussion. However, it is not envisiged
//! that there will be multiple implementations that interact with
//! spec-formatted data.
//!
//! This is because multiple implementations will be incompletely implemented,
//! have incomplete support for different features, and have errors. This
//! fragments the landscape and worsens the end user experience.
//!
//! Additionally, the spec is a relativley small part of an implementation. For
//! example, the spec contains infromation about the kind of signatures, where
//! they're laid out, what data they cover, etc. So, for implementations to
//! successfully interoperate re signatures, they must have complete and complex
//! cyrto libraries, etc.
//!
//! Additionally, if the canonical implementation has a bug, that bug can be
//! fixed, and worked around. If the bug has made its way into a lot of
//! archives, that bug can be "canonicalized", and future versions of the spec
//! can mandate compatibility. This is much harder if there are many parallel
//! implementations and many different bugs.
//!
//! Instead, it is envisiged that there will be a single implementation,
//! written in rust, that will have language bindings that allow it to be used
//! in as many different contexts is possible. The single implementation will be
//! maximally permissively licensed (CC0), and be sutible, with compile-time
//! configuration, for both highly resourced and embedded environments.
//!
//! Language bindings will likely take the form of a low-level C API, wrapped
//! by higher level bindings in target languages.
//!
//! Language bindings will be maintained in-tree, so that they are maintained in
//! lockstep with the reference implementation.
//!
//! Of course, those who choose to write alternative implementations are wished
//! the best in this endevor. However, the primary implementation will not
//! prioritize compatibility with alternative implementations. For example, if
//! the primary implementation has a bug that disagrees with the spec, but
//! fixing it would invalidate many extant archives, the spec will be updated to
//! agree with the primary implementation, not the other way around.
//!
//! ### Future Extensions
//!
//! - Structured Data / actual contents of metadata metadata that describes,
//!   semantically, what those files are, and their relationship. many content
//!   types are possible, but an example is a music album. would say "These are
//!   the tracks, this is the artist, this is the album, this file is track1,
//!   track2, this file is the cover. the file format is flac, etc."
//!
//!   - allow metadata to describe objects in universe, full wikidata model
//!
//!   This metadata is machine readable, so user agents may do useful things on
//!   behalf of users.
//!
//!   For example:
//!   - transcode all files to another format
//!   - translate the metadata to another format, for example to the format used
//!     by a media player
//!   - etc
//!
//!   metadata should reference items by numeric ID, not hash. a commit will
//!   contain hash<->id mappings.
//!
//!   this is so that metadata can be separated from hashes, and metadata can be
//!   improved and distributed separate from its contents
//!
//! - Allow multiple unrelated commits in a single repostiory
//!
//! - actual compression
//!
//! - add object headers and / or footers
//!
//! - work-tree mode
//!
//! - Add entry/member headers to before or after item in the body. These say:
//!   - length: so it can be extracted
//!   - type: signature, blob, commit, repository, tree, hash tree
//!   - compression: type of compression, if any
//!   - hash: so it can be verified
//!   - fragmentation information: so a file can be split into multiple entries
//!
//!   This can allow recovering items if the archive is truncated.
//!
//! - encryption at-rest
//!
//! - work-tree mode
//!
//! - mult-part archives (may not be necessary, since archives can already be
//!   thought of as being broken into 1kib blocks, and disks are very large,
//!   making splitting archives between multiple phsyical storage media less
//!   relevant.
//!
//! - fragmentation
//!
//! Specification
//! -------------
//!
//! Spec is overly flexible and complex for initial simple application. However,
//! it's important to leave flexibility so future features don't have to be
//! hacked in.
//!
//! There are many desirable features, most of which are out-of-scope for the
//! initial mvp. However, they must be kept in mind from the beginning, so that
//! appropriate flexibility is included in the archive format, so these features
//! can be added later.
//!
//! - handle large amounts of binary data well
//! - support signatures with different policies
//! - be useful for transmitting files in multiple contexts, like FTP, HTTP,
//!   BitTorrent, IPFS.
//! - extensible
//!
//! Explain importance of hashes committing or not committing to certain data.
//!
//! Mutiple modes:
//!
//! Support loose files, filename is `manifest.idml`, supports single-file
//! archive, filename is `NAME.imdl`. The loose file archive is the MVP, but you
//! can see how a single-file archive could be supported.
//!
//! `imdl-archive` is similar to a distributed version control system repository
//! in semantics, and adopts some terminology from DVCSs. In particular, the
//! contents of an archive are a repository, which contains one or more commits.
//!
//! Commits
//!
//! ### Modes
//!
//! - manifest + loose files: manifest.imdl
//! - single file: NAME.imdl
//!
//! If a three letter extension is required, use .imd
//!
//! ### Hash Types
//!
//! - archive hash
//! - repo hash
//! - tree hash
//! - metadata hash
//! - signature hash (may be used for distribution, just distribute signature,
//!   which contains hash of data, everything can be retrieved from signature)
//!
//! Objects that have hashes include:
//! - archive hash: can be used to sync an archive in its entirity, but will
//!   change if anything in the archive changes, including changes in encoding
//!   but not content (compression changes)
//!
//! - repository hash: itentifies the content of an archive. will change if any
//!   content changes (files/commits/signatures added or removed) but is
//!   insensitive to changes in encoding, so can identify the contents of an
//!   archive, independent from its encoding
//!
//! - commit hash: commits to a tree, which is a directory heirarchy snapshot,
//!   and metadata. So a commit hash identifies the contents of a commit, and
//!   the metadata that describes that content.
//!
//! - tree hash: A tree hash covers a snapshot of a directory, and all sub-files
//!   and directories.
//!
//! - signature: A signature made over some message.
//!
//! - message: what a signature is signing
//!
//!
//! ### Serialization and Deserialization
//!
//! Serialization format should:
//! - binary, so large amounts of binary data can be stored inline without
//!   escapin
//! - have a canonical representation of encoded data, so that hashes of data
//!   don't change unexpectedly
//! - efficient, for low power devices and efficient manipulation of large
//!   amounts of data. in particular, decoding should not require allocation to
//!   be efficient, and it should be possible mmap data.
//! - ideally, the serialization format should be usable for IPC and network
//!   messages, so that future protocols can use the same format.
//! - must be capable of generated fixed-size, known-layout types when needed
//!
//! I'm working on a custom format with derive macros, but in the mean time,
//! here are the salient features of the format:
//!
//! - encoders produce canonical output. I.e. for a given input, a decoder will
//!   produce a single, well specified output.
//! - enums and structs both supported
//! - integers are little endian
//! - all data is packed, with no alignment and padding, since unaligned
//!   accesses are fast on modern computers
//! - Very efficient, all data can be mmapped and quickly traversed in-place
//! - Small size penalty: variable length data is stored out-of-line, and
//!   referenced with offsets. these offsets take up additional space, relative
//!   to a format where all data is stored inline
//! - enums and structs are, by default, not extensible. however, it is possible
//!   to opt into flexible versions of both.
//!
//! - flexible enums and flexible structs are extensible in different ways.
//!
//! - flexible structs: can add new fields, and remove fields that are Option-al
//! - flexibile enums: can add and remove variants
//!
//! - Flexible structs and enums come with a small space and indirection
//!   penalty, which is why they aren't used everywhere, and why they aren't
//!   used where a fixed, predicatable format is required.
//!
//! ### Archive structure
//!
//! The archive consists of four segments. These segments
//!
//! - Header: Fixed length
//! - Body: Variable length binary data
//! - Footer: Variable length record. contains pointers to body data
//! - Footer length: little endian u64 containing the length of the footer
//!
//! ### Segmenting
//!
//! ```
//! use std::{convert::TryInto, fs, io, mem};
//!
//! use imdl_spec::archive::Header;
//!
//! // Normally, the archive would be read from a file:
//! // let archive = fs::read("manifest.imdl")?;
//!
//! // Instead, let's use dummy data:
//! let archive: &[u8] = &[0; 256];
//!
//! // The header is of a fixed length:
//! let header_end = mem::size_of::<Header>();
//!
//! // And so can be extracted immediately:
//! let header = &archive[..header_end];
//!
//! // The end of the footer is always 8 bytes, the size of a u64, before the
//! // end of the archive:
//! let footer_end = archive.len() - 8;
//!
//! // The length of the footer, in bytes, is stored as a little-endian u64
//! // integer at the end of the archive:
//! let footer_length_bytes: [u8; 8] = archive[footer_end..].try_into().unwrap();
//! let footer_length = u64::from_le_bytes(footer_length_bytes) as usize;
//!
//! // The start of the footer is footer length bytes before the end:
//! let footer_start = footer_end - footer_length;
//!
//! // Now that we have the footer start and end, we can extract the footer:
//! let footer = &archive[footer_start..footer_end];
//!
//! // The body is everything between the header and footer:
//! let body = &archive[header_end..footer_start];
//!
//! // The extracted header, body, footer, and final u64 are equal to the
//! // total size:
//! assert_eq!(archive.len(), header.len() + body.len() + footer.len() + 8);
//!
//! Ok::<(), io::Error>(())
//! ```
//!
//! ### Header, Body, and Footer
//!
//! After segmentation, the archive header, body, and footer segments are
//! deserialized.
//!
//! This rest of this spec is written as rust types. Flexible structs and enums
//! are annotated with a `flexible` annotation, and inflexible structs and
//! enums are annotated with a `fixed` tax. These tags, currently, are purely
//! informative.
//!
//! If a field or struct is annotated with the `later` tag, then its contents
//! are only an example of what it might contain. The future contents will
//! be decided on in future revisions of this specification.
//!
//! Some types are not fully implemented, for example Map and Slice. These are
//! types that will be exposde by the format, so aside from an explanation of
//! thier charactaristics, they have no internals.
//!
//! See the [Footer](Footer) and [Header](Header) documentation for information
//! about their contents.
//!
//! Since the footer is at the end of an archive, files can be added to an
//! archive in-place, by trucating the archive before the footer, writing new
//! body objects, and, finally, writing the new footer.
//!
//! Todo
//! ----
//!
//! - justify each `fixed` or `flexible`
//! - get feedback from RC
//! - cypherpunk bitstream ad
//! - get feedback from scene groups
//! - Should be composable and recursive. Should be possible to view a large
//!   collection of imdl archives as a single archive containing other archives.
//!   How is this best accomplished? Should it just be an archive will all
//!   sub-archives being files in a top-level tree? Or should there be some
//!   top-level thing that's like git refs that contains a bunch of sub-archives
//!   by link?

use std::marker::PhantomData;

/// `fixed`
pub struct Header {
  pub magic_number: MagicNumber<b"imdl">,
  pub version: Version,
  pub flags: Flags,
}

/// `fixed`
pub struct MagicNumber<const CONTENTS: &'static [u8]>;

/// `fixed`
pub struct Version {
  pub major: u64,
  pub minor: u64,
  pub patch: u64,
}

/// `fixed`
pub struct Flags {
  pub bits: u128,
}

/// `flexible`
pub struct Footer {
  pub repository: Link<Repository>,
  pub object_map: ObjectMap,
}

/// `flexible`
pub struct ObjectMap {
  pub objects: Map<Hash, Object>,
}

/// `fixed`
pub struct Object {
  pub location: Location,
  pub compression: Compression,
  pub length: u64,
  /// Should this be required?
  /// If interior hashes are always stored, then corruption in files
  /// can be identified, instead of just knowing that the hashes don't
  /// match.
  ///
  /// Additionally, data can be securely icrementally streamed from a
  /// signed archive.
  ///
  /// One possibility is to make interior hash storage optional. However,
  /// if interior hash storage is optional, then archives without interior
  /// hashes would be degraded.
  pub hash_tree: Option<Link<HashTree>>,
}

/// discuss hash tree overhead. Overhead at 1KiB blocks is ?,
/// at 16KIB blocks is 0.002%, or 2mib per gibibyte.
///
/// Tiers in order, so any number can be omitted.
///
/// `flexible`
pub struct HashTree {
  pub hashes: Slice<Hash>,
}

/// `flexible`
pub enum Location {
  Body { offset: u64 },
  Path { path: Path },
}

/// `flexible`
pub enum Compression {
  Uncompressed,
}

/// Should this support inline data?
/// `fixed`
pub struct Link<T> {
  pub hash: Hash,
  pub value: PhantomData<T>,
}

/// `fixed`
pub struct Hash {
  pub bytes: [u8; 32],
}

/// `flexible`
pub struct Repository {
  pub head: Link<Commit>,
  pub certificates: Set<Link<Certificate>>,
}

/// `flexible`
pub struct Commit {
  pub tree: Link<Tree>,
  pub parent: Option<Link<Commit>>,
  pub metadata: Link<Metadata>,
}

/// `flexible`
pub struct Metadata {}

/// `flexible`
pub struct Tree {
  pub root: Directory,
}

/// `flexible`
pub struct Directory {
  pub entries: Map<Filename, Entry>,
}

/// `flexible`
pub struct Entry {
  pub ty: EntryType,
}

/// `flexible`
pub enum EntryType {
  Directory(Directory),
  File(File),
}

/// `fixed`
pub struct Map<K, V> {
  pub key: PhantomData<K>,
  pub value: PhantomData<V>,
}

/// `fixed`
pub struct Set<T> {
  pub value: PhantomData<T>,
}

/// `fixed`
pub struct Slice<T> {
  pub value: PhantomData<T>,
}

/// `flexible`
pub struct File {
  pub hash: Hash,
}

/// `fixed`
pub struct Filename {
  /// Maximum length 255 bytes
  /// May not contain `\0`, or `\\`, or be `.`, or `..`
  pub data: String,
}

/// `fixed`
pub struct Path {
  /// may not contain `\0`, empty path components, `..`, or `.`, or a leading
  /// `\` or trailing `/`, filenames limited to 255 bytes
  pub data: String,
}

/// Must not be possible to trick someone into signing something they mean to.
///
/// Accomplished by having signature be over hash of signature message.
///
/// This sheme *must* be vetted by a crytographer first.
///
/// `flexible`
pub struct Certificate {
  pub message: Link<Message>,
  pub signature: Signature,
}

/// `fixed`
pub struct Message {
  /// for preventing tricking someone into signing something they didn't intend
  pub magic: MagicNumber<b"imdl.signature">,
  /// for versioning signatures separately from protocol
  pub version: SignatureVersion,
  /// for flags, unknown if this is useful
  pub flags: SignatureFlags,
  /// pubkey that signed
  pub pubkey: Pubkey,
  /// To prevent all bits of message being known by attacker
  /// Can nonce be derived deterministically?
  pub nonce: [u8; 32],
  // what this signature "means"
  pub policy: Policy,
}

/// `fixed`
pub struct SignatureVersion {
  pub major: u64,
  pub minor: u64,
  pub patch: u64,
}

/// `fixed`
pub struct SignatureFlags {
  pub flags: u128,
}

/// `flexible`
pub enum Policy {
  Author { hash: Hash },
  Packager { hash: Hash },
}

/// `fixed`
pub struct Signature {
  pub bytes: [u8; 64],
}

/// `fixed`
pub struct Pubkey {
  pub bytes: [u8; 32],
}
