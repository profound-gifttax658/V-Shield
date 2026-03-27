# V-Shield Architecture

## System Overview

V-Shield is a multi-layer cryptographic steganography system designed to hide arbitrary data in video frames in a way that survives YouTube's compression algorithms.

### Core Layers

```
┌─────────────────────────────────────────────────────────┐
│ Application Layer (Encoder/Decoder CLIs)                │
├─────────────────────────────────────────────────────────┤
│ Protocol Layer (Frame/Metadata structures)              │
├─────────────────────────────────────────────────────────┤
│ Cryptography Layer (ChaCha20-Poly1305, SHA-256)         │
├─────────────────────────────────────────────────────────┤
│ Encoding Layer (ECC, Interleaving, Color  Mapping)      │
├─────────────────────────────────────────────────────────┤
│ Rendering Layer (PNG generation)                        │
└─────────────────────────────────────────────────────────┘
```

---

## Module Breakdown

### `vshield-core` - Shared Library

The heart of V-Shield, containing all cryptographic and encoding logic.

#### `token.rs` - Token Management
- **Purpose**: Generate and manage cryptographic tokens
- **Key Struct**: `Token { key: [u8; 32], video_id: [u8; 16] }`
- **Behavior**: 
  - `Token::generate()` creates a NEW random token EVERY time
  - Different files get different tokens (randomness, not determinism)
  - Tokens encode as Base58 for user display: `vshield://...`
  - Parsing: `Token::from_str()` for recovery
- **Security Notes**:
  - 32-byte key is suitable for ChaCha20
  - 16-byte video_id for frame association
  - NO derivation from file content (different from old code)

#### `crypto.rs` - AEAD Encryption
- **Algorithm**: ChaCha20-Poly1305 (authenticated encryption)
- **Nonce Strategy**: **RANDOM and prepended** to ciphertext (CRITICAL SECURITY FIX)
  - Old: Fixed nonce (two-time pad vulnerability)
  - New: Random nonce generated per encryption
  - Format: `[12-byte random nonce][ciphertext][16-byte auth tag]`
- **Functions**:
  - `encrypt(key, plaintext) -> Vec<u8>` returns nonce + ciphertext
  - `decrypt(key, encrypted_data) -> Result<Vec<u8>>`
  - `hash_sha256(data) -> [u8; 32]`
  - `verify_sha256(data, hash) -> bool`
- **Overhead**: 12 (nonce) + 16 (auth tag) = 28 bytes per encryption

#### `protocol.rs` - Frame Structures
- **Frame Structure**:
  ```
  ┌──────────────────────────────────────────────┐
  │ Finder Patterns (4 corners)                  │
  ├──────────────────────────────────────────────┤
  │ Frame Header (16 bytes)                      │
  ├──────────────────────────────────────────────┤
  │ Metadata Block (232 bytes, first frame only) │
  ├──────────────────────────────────────────────┤
  │ Data Blocks (3 bits each, 8-color palette)   │
  ├──────────────────────────────────────────────┤
  │ Error Correction (Reed-Solomon)              │
  └──────────────────────────────────────────────┘
  ```

- **Key Structs**:
  - `ColorValue` (enum): 8 colors → 3 bits per block
  - `MetadataBlock` (232 bytes fixed):
    - filename: 128 bytes (null-terminated)
    - file_size: 8 bytes (LE u64)
    - file_hash: 32 bytes (SHA-256)
    - token_id: 64 bytes (null-terminated vshield://)
  - `DataBlock`: Single color per block (optimized from 16×16 array)
  - `FrameHeader` (16 bytes):
    - frame_id: u32
    - block_size: u8 (4, 8, or 16 pixels)
    - data_blocks_count: u16
    - is_first_frame: bool
    - protocol_version: u8
    - flags: u8
  - `Frame`: Complete frame with header, metadata, blocks, pixels

- **Data Density**:
  - 1920×1080 frame with 8px blocks = 51,200 blocks
  - 3 bits per block = 153,600 bits = 19,200 bytes
  - With ~20% ECC redundancy: ~15,400 bytes per frame

#### `anchor.rs` - Pattern Detection
- **Purpose**: Generate and detect finder patterns (like QR codes)
- **Pattern**: 1:1:3:1:1 ratio (21×21 pixels at 4:4:12:4:4)
- **Locations**: 4 corners of frame
- **Use**: Frame alignment and orientation detection

#### `ecc.rs` - Error Correction
- **Algorithm**: Reed-Solomon codes (from `reed-solomon-erasure` crate)
- **Redundancy**: 20-40% (configurable, auto-reduced for large data)
- **Buffer Limit**: Max 255 total symbols (data + parity)
- **Phase 1 Limit**: ~100 bytes per frame (due to RS constraint)

#### `interleave.rs` - Data Scattering
- **Purpose**: Spread data across the frame to resist burst errors
- **Strategy**: Distribute blocks non-sequentially across frame
- **Benefit**: YouTube compression artifacts less likely to destroy contiguous data

#### `lib.rs` - Public API
- Re-exports key types and functions
- Version and constants
- Test utilities

### `vshield-enc` - Encoder Binary

Converts files into encoded PNG frames.

**Encoding Pipeline**:
```
Input File
   ↓
[Read & Hash]
   ↓ SHA-256
[Generate Token]
   ↓ Random key + video_id
[Encrypt with ChaCha20]
   ↓ Random nonce + ciphertext
[Apply Reed-Solomon ECC]
   ↓ Adaptive redundancy
[Interleave Blocks]
   ↓ Spread across frame
[Render to PNG]
   ↓ Create 1920×1080 image
Output: frame_0000.png + metadata.json
```

**Key Functions**:
- `Encoder::new(input_file, config)`
- `encode() -> Result<Token, Error>` - Returns unique token for decryption

### `vshield-dec` - Decoder Binary

Extracts files from encoded PNG frames.

**Decoding Pipeline** (Phase 1.5):
```
Input Frame (PNG)
   ↓
[Load Image]
   ↓
[Detect Finder Patterns]
   ↓ Corner detection
[Extract Frame Header]
   ↓ Read 16-byte header
[Read Metadata Block]
   ↓ 232 bytes fixed format
[De-interleave]
   ↓ Restore block order
[Majority Vote]
   ↓ Recover color per block
[Apply Reed-Solomon Decode]
   ↓ Error correction
[Decrypt with Token]
   ↓ ChaCha20 with stored nonce
[Verify SHA-256 Hash]
   ↓ Check file integrity
Output: recovered.bin
```

**Current Status**: Framework only (Phase 1.5)

---

## Cryptographic Security Model

### What's Protected

- **Confidentiality**: ChaCha20 encryption with random nonce
- **Integrity**: Poly1305 authentication + SHA-256 file hash
- **Uniqueness**: Each file gets random token (not derived from content)
- **Nonce Safety**: Random 12-byte nonce per encryption (no reuse)

### What's NOT Protected

- **The token itself**: Must be kept safe by user
- **Video platform moderation**: YouTube can still remove videos
- **Strategic reverse engineering**: Only protects against compression

### Key Sizes

- **Encryption Key**: 256 bits (32 bytes) ✓ Suitable for ChaCha20
- **Token**: 48 bytes total (32 key + 16 video_id)
- **Hash**: 256 bits (32 bytes) SHA-256 blocks
- **Nonce**: 96 bits (12 bytes) ChaCha20 standard

### Threat Model

| Threat | Protected? | Mechanism |
|--------|-----------|-----------|
| YouTube compression | ✓ | ECC + color redundancy |
| Chroma subsampling | ✓ | Y-channel focused colors |
| Eavesdropping | ✓ | ChaCha20 encryption |
| MITM attacks | ✗ | Use HTTPS for token distribution |
| Brute force (token) | ✓* | 256-bit key = 2^256 combinations |
| Token loss | ✗ | No recovery mechanism |
| Nonce reuse | ✓ | Random nonce per encryption |

*Depends on token distribution method

---

## Data Flow: Encoding

```
File.bin (50 bytes)
    │
    ├─→ SHA256 hash
    │   └─→ [32 bytes]
    │
    ├─→ Generate Token
    │   ├─→ Key: [32 random bytes]
    │   ├─→ Video ID: [16 random bytes]
    │   └─→ String: "vshield://abc123..."
    │
    ├─→ Encrypt(file, key, random_nonce)
    │   └─→ [12-byte nonce][encrypted file][16-byte tag] = 66 bytes
    │
    ├─→ Create MetadataBlock
    │   ├─→ filename: "file.bin" (128 bytes)
    │   ├─→ file_size: 50 (8 bytes LE)
    │   ├─→ file_hash: [32 bytes]
    │   └─→ token_id: "vshield://..." (64 bytes)
    │   └─→ Total: 232 bytes fixed
    │
    ├─→ Apply Reed-Solomon ECC (20% redundancy)
    │   ├─→ Input: 232 + 66 = 298 bytes
    │   ├─→ Output: 357 bytes (20% redundancy)
    │   └─→ Blocks: 357 / 3 bits = 119 blocks
    │
    ├─→ Interleave across 1920×1080 frame
    │   └─→ 51,200 available blocks
    │
    └─→ Render to PNG
        ├─→ 8 colors per pixel (8-color palette)
        ├─→ 1920×1080 @8px blocks
        └─→ frame_0000.png

Output: frame_0000.png + metadata.json
Token: vshield://5650f5a0-d8c8-4d0c-a00c-46124b47d394
```

---

## Data Flow: Decoding (Phase 1.5)

```
frame_0000.png
    │
    ├─→Load image
    │
    ├─→ Detect anchors
    │   └─→ Align frame
    │
    ├─→ Extract header
    │   └─→ Determine block sizing
    │
    ├─→ Read metadata (232 bytes)
    │   ├─→ Get original filename
    │   ├─→ Get file size
    │   ├─→ Get expected hash
    │   └─→ Get token (for user reference)
    │
    ├─→ De-interleave blocks
    │
    ├─→ Majority vote per block
    │   └─→ Recover 3-bit color value
    │
    ├─→ Reed-Solomon decode
    │   └─→ Recover original 298 bytes
    │
    ├─→ Split: metadata (232) | encrypted data (66)
    │
    ├─→ User provides token via CLI
    │   └─→ Calls Token::from_str()
    │
    ├─→ Decrypt(66 bytes, key, nonce)
    │   ├─→ Extract nonce (first 12 bytes)
    │   ├─→ Decrypt ciphertext
    │   └─→ Verify auth tag
    │   └─→ Output: 50 bytes original file
    │
    └─→ Verify SHA-256
        └─→ Confirm file integrity

Output: recovered_file.bin
```

---

## Performance Characteristics

### Encoding (1920×1080 frame @ 100 bytes data)

| Operation | Time | Notes |
|-----------|------|-------|
| Read file | <1ms | Depends on disk |
| SHA-256 | <1ms | CPU-bound |
| ChaCha20 encrypt | <1ms | 100 bytes |
| Reed-Solomon encode | 5-10ms | Algorithm complexity |
| PNG rendering | 50-100ms | Image library |
| **Total** | **60-120ms** | Per frame |

### Decoding (Pending Phase 1.5)

Expected similar or slightly faster (no PNG writing).

### Scalability

- **Phase 1**: 1 frame per file (~20 KB payload)
- **Phase 2**: Multiple frames per file (streaming)
- **Bottleneck**: Reed-Solomon 255-symbol limit
  - Workaround: Split data across multiple frames
  - Frame sequencing: frame_0000, frame_0001, etc.

---

## Error Handling

### Encoding Errors

| Scenario | Handling |
|----------|----------|
| File not found | Return Err, exit CLI |
| Unreadable file | Return Err, exit CLI |
| File > 100 bytes | Reduce redundancy 20% → 10% → 5% |
| File > 150 bytes | Automatic adaptation |
| Token generation | Never fails (random) |
| PNG write failure | Return Err |

### Decoding Errors

| Scenario | Handling |
|----------|----------|
| Frame not found | Return Err |
| Invalid PNG | Return Err |
| Anchor detection | Return Err (alignment needed) |
| Corrupted metadata | Use defaults if possible |
| Decryption failure | Unknown token or corrupted frame |
| Hash mismatch | File integrity error |

---

## Future Roadmap

### Phase 1.5 - YouTube Testing
- [ ] Full decoder implementation
- [ ] Real YouTube upload test
- [ ] Compression resilience analysis
- [ ] Optimal block size determination

### Phase 2 - Multi-frame Support
- [ ] Frame sequencing (multiple frames per file)
- [ ] Adaptive block sizing
- [ ] Performance optimization

### Phase 3 - Browser Extension
- [ ] WASM compilation
- [ ] YouTube video capture
- [ ] Real-time decoder
- [ ] Chrome extension manifest

### Phase 4 - Streaming
- [ ] MediaSource Extensions
- [ ] Live video playback
- [ ] Network optimization

---

## Code Quality & Testing

### Current Test Coverage

- Token generation and serialization ✓
- Encryption round-trip (encrypt → decrypt) ✓
- Random nonce uniqueness ✓
- Wrong key rejection ✓
- Corrupted data detection ✓
- SHA-256 hashing ✓

### Planned Tests

- Frame rendering accuracy
- Anchor detection robustness
- Interleaving correctness
- Reed-Solomon round-trip
- Full encoding → frame creation → decoding

---

## Security Audit Checklist

- [x] Nonce randomization (critical fix)
- [x] Token randomness (not file-derived)
- [x] 256-bit encryption key
- [x] Authenticated encryption (ChaCha20-Poly1305)
- [x] File integrity (SHA-256)
- [x] Fixed metadata size (no variable-length encoding)
- [ ] Third-party audit (needed for production)

---

## Dependencies

| Crate | Purpose | Version |
|-------|---------|---------|
| `chacha20poly1305` | AEAD encryption | 0.10 |
| `sha2` | Hashing | 0.10 |
| `reed-solomon-erasure` | Error correction | 6.0 |
| `image` | PNG I/O | 0.24 |
| `rand` | Random generation | 0.8 |
| `bs58` | Base58 encoding | 0.5 |
| `serde` | Serialization | 1.0 |

---

## Glossary

| Term | Definition |
|------|-----------|
| **AEAD** | Authenticated Encryption with Associated Data |
| **ECC** | Error Correction Code (Reed-Solomon) |
| **Nonce** | Number used once (randomness for encryption) |
| **Token** | Cryptographic key in user-readable format |
| **Frame** | Single 1920×1080 image with encoded data |
| **Block** | 4×4/8×8/16×16 pixel region (3 bits of data) |
| **Payload** | Encrypted user data + metadata |
| **Interleaving** | Spreading data across frame (burst protection) |
| **Majority Vote** | Decoding: choose most common color |
| **Two-Time Pad** | Attack that breaks encryption when nonce repeats |

---

**Architecture Version**: 1.0  
**Last Updated**: March 2026  
**Status**: Phase 1 Complete, Phase 1.5 In Design
