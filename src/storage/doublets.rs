use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use ::doublets::{Doublets, DoubletsExt, Links};
use platform_mem::FileMapped;

use crate::link_flags::LinkFlags;
use crate::link_network::{Link, LinkId, LinkMetadata, LinkNetwork, LinkType};
use crate::source::{ByteRange, Point, SourceSpan};

use super::{LinkStore, LinkStoreQuery, StorageError};

impl From<::doublets::Error<u64>> for StorageError {
    fn from(error: ::doublets::Error<u64>) -> Self {
        Self::Doublets(error.to_string())
    }
}

#[cfg(feature = "doublets")]
type FileMappedDoubletsStore =
    ::doublets::unit::Store<u64, FileMapped<::doublets::parts::LinkPart<u64>>>;

#[cfg(feature = "doublets")]
const TAG_HEADER: u64 = u64::MAX - 1_024;
#[cfg(feature = "doublets")]
const TAG_REFERENCE: u64 = u64::MAX - 1_025;
#[cfg(feature = "doublets")]
const TAG_METADATA_BYTE: u64 = u64::MAX - 1_026;
#[cfg(feature = "doublets")]
const METADATA_VERSION: u8 = 1;
#[cfg(feature = "doublets")]
const SNAPSHOT_MAGIC: &[u8; 8] = b"MLDSNP01";

#[cfg(feature = "doublets")]
#[derive(Debug)]
struct StoredLinkRecord {
    sequence: u64,
    link: Link,
    registered_term: bool,
    deleted: bool,
}

/// File-mapped binary storage backed by `doublets-rs`.
#[cfg(feature = "doublets")]
pub struct DoubletsLinkStore {
    path: PathBuf,
    store: FileMappedDoubletsStore,
}

#[cfg(feature = "doublets")]
impl DoubletsLinkStore {
    /// Creates a new empty file-mapped doublets store at `path`.
    ///
    /// Any existing file at `path` is removed first.
    ///
    /// # Errors
    ///
    /// Returns [`StorageError`] when the file cannot be created or initialized.
    pub fn create_file(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let path = path.as_ref();
        if path.exists() {
            fs::remove_file(path)?;
        }
        let snapshot_path = snapshot_path(path);
        if snapshot_path.exists() {
            fs::remove_file(snapshot_path)?;
        }
        Self::open_file(path)
    }

    /// Opens an existing file-mapped doublets store, creating the file when it
    /// does not yet exist.
    ///
    /// # Errors
    ///
    /// Returns [`StorageError`] when the file cannot be opened or the doublets
    /// store cannot be initialized.
    pub fn open_file(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let path = path.as_ref().to_path_buf();
        let mem = FileMapped::from_path(&path)?;
        let store = ::doublets::unit::Store::<u64, _>::new(mem)?;
        let mut this = Self { path, store };
        for (link, registered_term) in read_snapshot(&this.path)? {
            this.append_record(
                link.id(),
                link.references(),
                link.metadata(),
                registered_term,
                false,
            )?;
        }
        Ok(this)
    }

    /// Replaces the logical binary contents with a lossless encoding of
    /// `network`.
    ///
    /// # Errors
    ///
    /// Returns [`StorageError`] when the file-mapped doublets store cannot be
    /// updated.
    pub fn replace_with_network(&mut self, network: &LinkNetwork) -> Result<(), StorageError> {
        for record in self.active_records()? {
            self.append_record(record.link.id(), &[], &LinkMetadata::new(), false, true)?;
        }
        for link in network.links() {
            let registered_term = link
                .metadata()
                .term()
                .is_some_and(|term| network.find_term(term) == Some(link.id()));
            self.append_record(
                link.id(),
                link.references(),
                link.metadata(),
                registered_term,
                false,
            )?;
        }
        self.persist_snapshot()
    }

    /// Returns the durable companion snapshot path for a doublets store path.
    #[must_use]
    pub fn snapshot_path(path: impl AsRef<Path>) -> PathBuf {
        snapshot_path(path.as_ref())
    }

    /// Reconstructs a [`LinkNetwork`] from this binary store.
    ///
    /// # Errors
    ///
    /// Returns [`StorageError`] when the binary data is malformed.
    pub fn to_network(&self) -> Result<LinkNetwork, StorageError> {
        let mut links = self
            .active_records()?
            .into_iter()
            .map(|record| (record.link, record.registered_term))
            .collect::<Vec<_>>();
        links.sort_by_key(|(link, _registered)| link.id());
        Ok(super::network_from_stored_links(links))
    }

    /// Creates a logical link using an explicit id.
    ///
    /// This is used by network import paths to preserve text and binary id
    /// equivalence. Normal callers can use [`LinkStore::create`].
    ///
    /// # Errors
    ///
    /// Returns [`StorageError`] when the id already exists or the backend
    /// cannot write the physical records.
    pub fn create_with_id(
        &mut self,
        id: LinkId,
        references: &[LinkId],
        metadata: LinkMetadata,
        registered_term: bool,
    ) -> Result<(), StorageError> {
        if self.latest_record(id)?.is_some() {
            return Err(StorageError::Corrupt(format!(
                "link id {id} already exists in doublets store"
            )));
        }
        self.append_record(id, references, &metadata, registered_term, false)?;
        self.persist_snapshot()
    }

    fn persist_snapshot(&self) -> Result<(), StorageError> {
        write_snapshot(&self.path, &self.active_records()?)
    }

    fn active_records(&self) -> Result<Vec<StoredLinkRecord>, StorageError> {
        let mut latest = BTreeMap::<LinkId, StoredLinkRecord>::new();
        for record in self.decode_all_records()? {
            let replace = latest
                .get(&record.link.id())
                .map_or(true, |existing| existing.sequence < record.sequence);
            if replace {
                latest.insert(record.link.id(), record);
            }
        }
        Ok(latest
            .into_values()
            .filter(|record| !record.deleted)
            .collect())
    }

    fn latest_record(&self, id: LinkId) -> Result<Option<StoredLinkRecord>, StorageError> {
        let latest = self
            .decode_records_for_id(id)?
            .into_iter()
            .max_by_key(|record| record.sequence);
        Ok(latest.filter(|record| !record.deleted))
    }

    fn decode_records_for_id(&self, id: LinkId) -> Result<Vec<StoredLinkRecord>, StorageError> {
        Ok(self
            .decode_all_records()?
            .into_iter()
            .filter(|record| record.link.id() == id)
            .collect())
    }

    fn append_record(
        &mut self,
        id: LinkId,
        references: &[LinkId],
        metadata: &LinkMetadata,
        registered_term: bool,
        deleted: bool,
    ) -> Result<(), StorageError> {
        self.encode_record(id, references, metadata, registered_term, deleted)
    }

    fn next_logical_id(&self) -> Result<LinkId, StorageError> {
        let max_id = self
            .decode_all_records()?
            .into_iter()
            .map(|record| record.link.id().as_u64())
            .max()
            .unwrap_or(0);
        let next = max_id
            .checked_add(1)
            .ok_or_else(|| StorageError::Corrupt("link id space is exhausted".to_string()))?;
        Ok(LinkId::from_u64(next))
    }

    fn headers(&self) -> Vec<::doublets::Link<u64>> {
        let any = self.store.constants().any;
        self.store
            .each_iter([any, TAG_HEADER, any])
            .collect::<Vec<_>>()
    }

    fn encode_record(
        &mut self,
        id: LinkId,
        references: &[LinkId],
        metadata: &LinkMetadata,
        registered_term: bool,
        deleted: bool,
    ) -> Result<(), StorageError> {
        let nonce = self.store.create_point()?;
        let id_link = self.store.create_link(nonce, id.as_u64())?;
        let header = self.store.create_link(TAG_HEADER, id_link)?;
        for (position, reference) in references.iter().enumerate() {
            let position = stored_position(position)?;
            let entry = self.store.create_link(TAG_REFERENCE, position)?;
            let value = self.store.create_link(entry, reference.as_u64())?;
            self.store.create_link(header, value)?;
        }

        let metadata = encode_metadata(metadata, registered_term, deleted)?;
        for (position, byte) in metadata.iter().enumerate() {
            let position = stored_position(position)?;
            let entry = self.store.create_link(TAG_METADATA_BYTE, position)?;
            let value = self.store.create_link(entry, u64::from(*byte) + 1)?;
            self.store.create_link(header, value)?;
        }
        Ok(())
    }

    fn decode_all_records(&self) -> Result<Vec<StoredLinkRecord>, StorageError> {
        self.headers()
            .into_iter()
            .map(|header| self.decode_header(header))
            .collect()
    }

    fn decode_header(
        &self,
        header: ::doublets::Link<u64>,
    ) -> Result<StoredLinkRecord, StorageError> {
        let mut references = BTreeMap::new();
        let mut metadata_bytes = BTreeMap::new();
        let any = self.store.constants().any;
        let id_link = self.store.get_link(header.target).ok_or_else(|| {
            StorageError::Corrupt(format!(
                "record header {} references missing id link {}",
                header.index, header.target
            ))
        })?;
        let logical_id = LinkId::from_u64(id_link.target);

        for association in self.store.each_iter([any, header.index, any]) {
            let value = self.store.get_link(association.target).ok_or_else(|| {
                StorageError::Corrupt(format!(
                    "record {logical_id} references missing value link {}",
                    association.target
                ))
            })?;
            let entry = self.store.get_link(value.source).ok_or_else(|| {
                StorageError::Corrupt(format!(
                    "record {logical_id} references missing entry link {}",
                    value.source
                ))
            })?;
            match entry.source {
                TAG_REFERENCE => {
                    references.insert(entry.target, LinkId::from_u64(value.target));
                }
                TAG_METADATA_BYTE => {
                    let byte = value.target.checked_sub(1).ok_or_else(|| {
                        StorageError::Corrupt("metadata byte cannot be zero".to_string())
                    })?;
                    let byte = u8::try_from(byte).map_err(|_| {
                        StorageError::Corrupt(format!("metadata byte out of range: {byte}"))
                    })?;
                    metadata_bytes.insert(entry.target, byte);
                }
                other => {
                    return Err(StorageError::Corrupt(format!(
                        "unknown doublets field tag {other}"
                    )));
                }
            }
        }

        let references = ordered_values(references, "reference")?;
        let metadata_bytes = ordered_values(metadata_bytes, "metadata byte")?;
        let (metadata, registered_term, deleted) = decode_metadata(&metadata_bytes)?;
        let link = Link {
            id: logical_id,
            references: std::sync::Arc::from(references),
            metadata,
        };
        Ok(StoredLinkRecord {
            sequence: header.index,
            link,
            registered_term,
            deleted,
        })
    }
}

#[cfg(feature = "doublets")]
impl LinkStore for DoubletsLinkStore {
    fn create(
        &mut self,
        references: &[LinkId],
        metadata: LinkMetadata,
    ) -> Result<LinkId, StorageError> {
        let id = self.next_logical_id()?;
        self.append_record(id, references, &metadata, true, false)?;
        self.persist_snapshot()?;
        Ok(id)
    }

    fn read(&self, id: LinkId) -> Result<Option<Link>, StorageError> {
        Ok(self.latest_record(id)?.map(|record| record.link))
    }

    fn update(
        &mut self,
        id: LinkId,
        references: &[LinkId],
        metadata: LinkMetadata,
    ) -> Result<bool, StorageError> {
        if self.latest_record(id)?.is_none() {
            return Ok(false);
        }
        self.append_record(id, references, &metadata, true, false)?;
        self.persist_snapshot()?;
        Ok(true)
    }

    fn delete(&mut self, id: LinkId) -> Result<bool, StorageError> {
        if self.latest_record(id)?.is_none() {
            return Ok(false);
        }
        self.append_record(id, &[], &LinkMetadata::new(), false, true)?;
        self.persist_snapshot()?;
        Ok(true)
    }

    fn search(&self, query: &LinkStoreQuery) -> Result<Vec<Link>, StorageError> {
        Ok(self
            .active_records()?
            .into_iter()
            .map(|record| record.link)
            .filter(|link| query.matches(link))
            .collect())
    }
}

#[cfg(feature = "doublets")]
fn snapshot_path(path: &Path) -> PathBuf {
    let mut snapshot = path.as_os_str().to_os_string();
    snapshot.push(".snapshot");
    PathBuf::from(snapshot)
}

#[cfg(feature = "doublets")]
fn write_snapshot(path: &Path, records: &[StoredLinkRecord]) -> Result<(), StorageError> {
    let mut output = Vec::new();
    output.extend_from_slice(SNAPSHOT_MAGIC);
    write_len(&mut output, records.len())?;
    for record in records {
        write_u64(&mut output, record.link.id().as_u64());
        write_len(&mut output, record.link.references().len())?;
        for reference in record.link.references() {
            write_u64(&mut output, reference.as_u64());
        }
        let metadata = encode_metadata(record.link.metadata(), record.registered_term, false)?;
        write_len(&mut output, metadata.len())?;
        output.extend_from_slice(&metadata);
    }
    fs::write(snapshot_path(path), output)?;
    Ok(())
}

#[cfg(feature = "doublets")]
fn read_snapshot(path: &Path) -> Result<Vec<(Link, bool)>, StorageError> {
    let snapshot_path = snapshot_path(path);
    if !snapshot_path.exists() {
        return Ok(Vec::new());
    }

    let bytes = fs::read(snapshot_path)?;
    let mut cursor = 0;
    if read_bytes(&bytes, &mut cursor, SNAPSHOT_MAGIC.len())? != SNAPSHOT_MAGIC {
        return Err(StorageError::Corrupt(
            "doublets snapshot has an invalid header".to_string(),
        ));
    }

    let record_count = read_len(&bytes, &mut cursor)?;
    let mut records = Vec::with_capacity(record_count);
    for _ in 0..record_count {
        let id = LinkId::from_u64(read_u64(&bytes, &mut cursor)?);
        let reference_count = read_len(&bytes, &mut cursor)?;
        let mut references = Vec::with_capacity(reference_count);
        for _ in 0..reference_count {
            references.push(LinkId::from_u64(read_u64(&bytes, &mut cursor)?));
        }
        let metadata_len = read_len(&bytes, &mut cursor)?;
        let metadata_bytes = read_bytes(&bytes, &mut cursor, metadata_len)?;
        let (metadata, registered_term, deleted) = decode_metadata(metadata_bytes)?;
        if deleted {
            return Err(StorageError::Corrupt(
                "doublets snapshot cannot contain tombstones".to_string(),
            ));
        }
        let link = Link {
            id,
            references: std::sync::Arc::from(references),
            metadata,
        };
        records.push((link, registered_term));
    }

    if cursor != bytes.len() {
        return Err(StorageError::Corrupt(
            "doublets snapshot has trailing bytes".to_string(),
        ));
    }
    Ok(records)
}

#[cfg(feature = "doublets")]
fn stored_position(position: usize) -> Result<u64, StorageError> {
    u64::try_from(position)
        .ok()
        .and_then(|position| position.checked_add(1))
        .ok_or_else(|| StorageError::Corrupt("position does not fit in u64".to_string()))
}

#[cfg(feature = "doublets")]
fn ordered_values<T: Copy>(
    values: BTreeMap<u64, T>,
    label: &'static str,
) -> Result<Vec<T>, StorageError> {
    let mut ordered = Vec::with_capacity(values.len());
    for (expected, (position, value)) in (1_u64..).zip(values) {
        if position != expected {
            return Err(StorageError::Corrupt(format!(
                "{label} positions must be contiguous; expected {expected}, found {position}"
            )));
        }
        ordered.push(value);
    }
    Ok(ordered)
}

#[cfg(feature = "doublets")]
fn encode_metadata(
    metadata: &LinkMetadata,
    registered_term: bool,
    deleted: bool,
) -> Result<Vec<u8>, StorageError> {
    let mut output = vec![
        METADATA_VERSION,
        link_type_code(metadata.link_type()),
        u8::from(metadata.is_named()),
        flag_bits(metadata.flags()),
        u8::from(registered_term),
        u8::from(deleted),
    ];
    write_optional_string(&mut output, metadata.term())?;
    write_optional_string(&mut output, metadata.definition())?;
    write_optional_string(&mut output, metadata.language())?;
    write_optional_span(&mut output, metadata.span())?;
    Ok(output)
}

#[cfg(feature = "doublets")]
fn decode_metadata(bytes: &[u8]) -> Result<(LinkMetadata, bool, bool), StorageError> {
    let mut cursor = 0;
    let version = read_u8(bytes, &mut cursor)?;
    if version != METADATA_VERSION {
        return Err(StorageError::Corrupt(format!(
            "unsupported metadata version {version}"
        )));
    }
    let mut metadata = LinkMetadata::new();
    if let Some(link_type) = parse_link_type_code(read_u8(bytes, &mut cursor)?)? {
        metadata = metadata.with_link_type(link_type);
    }
    metadata = metadata.with_named(read_u8(bytes, &mut cursor)? != 0);
    let flags = read_u8(bytes, &mut cursor)?;
    let registered_term = read_u8(bytes, &mut cursor)? != 0;
    let deleted = read_u8(bytes, &mut cursor)? != 0;

    if let Some(term) = read_optional_string(bytes, &mut cursor)? {
        metadata = metadata.with_term(term);
    }
    if let Some(definition) = read_optional_string(bytes, &mut cursor)? {
        metadata = metadata.with_definition(definition);
    }
    if let Some(language) = read_optional_string(bytes, &mut cursor)? {
        metadata = metadata.with_language(language);
    }
    if let Some(span) = read_optional_span(bytes, &mut cursor)? {
        metadata = metadata.with_span(span);
    }
    if flags != 0 {
        metadata = metadata.with_flags(parse_flags(flags));
    }
    if cursor != bytes.len() {
        return Err(StorageError::Corrupt(
            "metadata has trailing bytes".to_string(),
        ));
    }
    Ok((metadata, registered_term, deleted))
}

#[cfg(feature = "doublets")]
fn write_optional_string(output: &mut Vec<u8>, value: Option<&str>) -> Result<(), StorageError> {
    let Some(value) = value else {
        output.push(0);
        return Ok(());
    };
    output.push(1);
    write_len(output, value.len())?;
    output.extend_from_slice(value.as_bytes());
    Ok(())
}

#[cfg(feature = "doublets")]
fn read_optional_string(bytes: &[u8], cursor: &mut usize) -> Result<Option<String>, StorageError> {
    if read_u8(bytes, cursor)? == 0 {
        return Ok(None);
    }
    let len = read_len(bytes, cursor)?;
    let value = read_bytes(bytes, cursor, len)?;
    let value = String::from_utf8(value.to_vec())
        .map_err(|_| StorageError::Corrupt("metadata string is not UTF-8".to_string()))?;
    Ok(Some(value))
}

#[cfg(feature = "doublets")]
fn write_optional_span(output: &mut Vec<u8>, span: Option<SourceSpan>) -> Result<(), StorageError> {
    let Some(span) = span else {
        output.push(0);
        return Ok(());
    };
    output.push(1);
    let byte_range = span.byte_range();
    let start = span.start_point();
    let end = span.end_point();
    for value in [
        byte_range.start(),
        byte_range.end(),
        start.row(),
        start.column(),
        end.row(),
        end.column(),
    ] {
        write_usize(output, value)?;
    }
    Ok(())
}

#[cfg(feature = "doublets")]
fn read_optional_span(
    bytes: &[u8],
    cursor: &mut usize,
) -> Result<Option<SourceSpan>, StorageError> {
    if read_u8(bytes, cursor)? == 0 {
        return Ok(None);
    }
    let values = [
        read_usize(bytes, cursor)?,
        read_usize(bytes, cursor)?,
        read_usize(bytes, cursor)?,
        read_usize(bytes, cursor)?,
        read_usize(bytes, cursor)?,
        read_usize(bytes, cursor)?,
    ];
    Ok(Some(SourceSpan::new(
        ByteRange::new(values[0], values[1]),
        Point::new(values[2], values[3]),
        Point::new(values[4], values[5]),
    )))
}

#[cfg(feature = "doublets")]
fn write_len(output: &mut Vec<u8>, len: usize) -> Result<(), StorageError> {
    write_usize(output, len)
}

#[cfg(feature = "doublets")]
fn read_len(bytes: &[u8], cursor: &mut usize) -> Result<usize, StorageError> {
    read_usize(bytes, cursor)
}

#[cfg(feature = "doublets")]
fn write_usize(output: &mut Vec<u8>, value: usize) -> Result<(), StorageError> {
    let value = u64::try_from(value)
        .map_err(|_| StorageError::Corrupt("usize does not fit in u64".to_string()))?;
    write_u64(output, value);
    Ok(())
}

#[cfg(feature = "doublets")]
fn read_usize(bytes: &[u8], cursor: &mut usize) -> Result<usize, StorageError> {
    let value = read_u64(bytes, cursor)?;
    usize::try_from(value)
        .map_err(|_| StorageError::Corrupt("u64 does not fit in usize".to_string()))
}

#[cfg(feature = "doublets")]
fn write_u64(output: &mut Vec<u8>, value: u64) {
    output.extend_from_slice(&value.to_le_bytes());
}

#[cfg(feature = "doublets")]
fn read_u64(bytes: &[u8], cursor: &mut usize) -> Result<u64, StorageError> {
    let mut value = [0_u8; 8];
    value.copy_from_slice(read_bytes(bytes, cursor, 8)?);
    Ok(u64::from_le_bytes(value))
}

#[cfg(feature = "doublets")]
fn read_u8(bytes: &[u8], cursor: &mut usize) -> Result<u8, StorageError> {
    let value = *read_bytes(bytes, cursor, 1)?
        .first()
        .expect("one byte was requested");
    Ok(value)
}

#[cfg(feature = "doublets")]
fn read_bytes<'a>(
    bytes: &'a [u8],
    cursor: &mut usize,
    len: usize,
) -> Result<&'a [u8], StorageError> {
    let end = cursor
        .checked_add(len)
        .ok_or_else(|| StorageError::Corrupt("metadata cursor overflow".to_string()))?;
    let value = bytes.get(*cursor..end).ok_or_else(|| {
        StorageError::Corrupt("metadata ended before expected length".to_string())
    })?;
    *cursor = end;
    Ok(value)
}

#[cfg(feature = "doublets")]
fn flag_bits(flags: LinkFlags) -> u8 {
    u8::from(flags.is_error())
        | (u8::from(flags.has_error()) << 1)
        | (u8::from(flags.is_missing()) << 2)
        | (u8::from(flags.is_extra()) << 3)
}

#[cfg(feature = "doublets")]
fn parse_flags(bits: u8) -> LinkFlags {
    let mut flags = LinkFlags::clean();
    if bits & 0b0001 != 0 {
        flags = flags.with_error();
    }
    if bits & 0b0010 != 0 {
        flags = flags.with_containing_error();
    }
    if bits & 0b0100 != 0 {
        flags = flags.with_missing();
    }
    if bits & 0b1000 != 0 {
        flags = flags.with_extra();
    }
    flags
}

#[cfg(feature = "doublets")]
fn link_type_code(link_type: Option<LinkType>) -> u8 {
    match link_type {
        None => 0,
        Some(LinkType::Link) => 1,
        Some(LinkType::Reference) => 2,
        Some(LinkType::Relation) => 3,
        Some(LinkType::Language) => 4,
        Some(LinkType::Grammar) => 5,
        Some(LinkType::Type) => 6,
        Some(LinkType::Concept) => 7,
        Some(LinkType::Syntax) => 8,
        Some(LinkType::Field) => 9,
        Some(LinkType::Trivia) => 10,
        Some(LinkType::Token) => 11,
        Some(LinkType::Document) => 12,
        Some(LinkType::Semantic) => 13,
        Some(LinkType::Region) => 14,
        Some(LinkType::Object) => 15,
    }
}

#[cfg(feature = "doublets")]
fn parse_link_type_code(code: u8) -> Result<Option<LinkType>, StorageError> {
    Ok(Some(match code {
        0 => return Ok(None),
        1 => LinkType::Link,
        2 => LinkType::Reference,
        3 => LinkType::Relation,
        4 => LinkType::Language,
        5 => LinkType::Grammar,
        6 => LinkType::Type,
        7 => LinkType::Concept,
        8 => LinkType::Syntax,
        9 => LinkType::Field,
        10 => LinkType::Trivia,
        11 => LinkType::Token,
        12 => LinkType::Document,
        13 => LinkType::Semantic,
        14 => LinkType::Region,
        15 => LinkType::Object,
        other => {
            return Err(StorageError::Corrupt(format!(
                "unknown link type code {other}"
            )))
        }
    }))
}
