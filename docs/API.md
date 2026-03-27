# V-Shield API Reference

Complete API documentation for all public types and functions in V-Shield.

---

## `vshield-core` Library

### Module: `token`

Cryptographic token generation and management.

#### Struct: `Token`

```rust
pub struct Token {
    pub key: [u8; 32],        // Encryption key
    pub video_id: [u8; 16],   // Video association
}
```

##### Methods

##### `Token::generate() -> Self`

Generate a new random token. Called every encoding, always unique.

```rust
let token = Token::generate();
println!("{}", token.to_string()); // vshield://abc123...
```

**Returns**: Random `Token`  
**Panics**: Never  
**Use Case**: Always use this for new encodings

---

##### `Token::to_string() -> String`

Serialize token to user-displayable format.

```rust
let token = Token::generate();
let s = token.to_string();
// Output: "vshield://5650f5a0d8c840d0a0c46124b47d394..."
```

**Format**: `vshield://<base58(key || video_id)>`  
**Length**: ~64 characters  
**Use**: Display to user, save for later recovery

---

##### `Token::from_str(s: &str) -> Result<Self, String>`

Parse token from string representation.

```rust
match Token::from_str("vshield://abc...") {
    Ok(token) => println!("Valid token"),
    Err(e) => eprintln!("Parse failed: {}", e),
}
```

**Arguments**:
- `s` - Token string starting with "vshield://"

**Returns**: 
- `Ok(Token)` on success
- `Err(String)` with error message on failure

**Possible Errors**:
- Missing "vshield://" prefix
- Invalid Base58 encoding
- Wrong length (!=  48 bytes decoded)

**Use**: Recovery, CLI argument parsing

---

##### `Token::key_bytes() -> &[u8; 32]`

Get encryption key as byte reference.

```rust
let token = Token::generate();
let key: &[u8; 32] = token.key_bytes();
```

---

##### `Token::video_id_bytes() -> &[u8; 16]`

Get video association bytes as reference.

```rust
let token = Token::generate();
let vid: &[u8; 16] = token.video_id_bytes();
```

---

##### `Token::set_video_id(&mut self, video_id: [u8; 16])`

Update video association (called during first frame encoding).

```rust
let mut token = Token::generate();
token.set_video_id([0u8; 16]); // Can update
```

---

### Module: `crypto`

Authenticated encryption and hashing.

#### Constants

```rust
pub const TAG_SIZE: usize = 16;              // Poly1305 auth tag
pub const NONCE_SIZE: usize = 12;            // ChaCha20 nonce
pub const ENCRYPTION_OVERHEAD: usize = 28;   // nonce + tag
```

---

#### Function: `encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, String>`

Encrypt data with authenticated encryption (ChaCha20-Poly1305).

```rust
let key = [42u8; 32];
let data = b"Secret message";

let encrypted = encrypt(&key, data)?;
println!("Encrypted: {} bytes", encrypted.len());
// Output: Encrypted: 42 bytes (14 original + 28 overhead)
```

**Arguments**:
- `key` - 32-byte encryption key
- `plaintext` - Data to encrypt (any length)

**Returns**:
- `Ok(Vec<u8>)` with format: `[12-byte nonce][ciphertext][16-byte tag]`
- `Err(String)` on encryption error (rare)

**Properties**:
- **Random nonce** generated fresh each call (CRITICAL SECURITY PROPERTY)
- **Authenticated** - Poly1305 validates integrity
- **No nonce reuse** - Different each encryption
- **Overhead** - Always adds 28 bytes

**Use Case**: Primary encryption function for data in encoder

---

#### Function: `decrypt(key: &[u8; 32], encrypted_data: &[u8])  -> Result<Vec<u8>, String>`

Decrypt data encrypted with `encrypt()`.

```rust
let key = [42u8; 32];
let encrypted = encrypt(&key, b"Secret")?;
let decrypted = decrypt(&key, &encrypted)?;

assert_eq!(decrypted, b"Secret".to_vec());
```

**Arguments**:
- `key` - 32-byte decryption key (must match encrypt key)
- `encrypted_data` - Output from `encrypt()`

**Returns**:
- `Ok(Vec<u8>)` - Original plaintext
- `Err(String)` on decryption failure

**Possible Errors**:
- Data too short (< 12 bytes for nonce)
- Wrong key
- Corrupted ciphertext  
- Invalid authentication tag

**Use Case**: Primary decryption in decoder

---

#### Function: `hash_sha256(data: &[u8]) -> [u8; 32]`

Compute SHA-256 hash of data.

```rust
let data = b"File content";
let hash = hash_sha256(data);
println!("Hash: {:?}", hash); // [u8; 32]
```

**Arguments**: Any byte slice

**Returns**: 32-byte hash array

**Use Case**: File integrity verification

---

#### Function: `verify_sha256(data: &[u8], expected_hash: &[u8; 32]) -> bool`

Verify SHA-256 hash matches expected value.

```rust
let original = b"File";
let hash = hash_sha256(original);
assert!(verify_sha256(original, &hash)); // true
assert!(!verify_sha256(b"Modified", &hash)); // false
```

**Returns**: 
- `true` if hash matches
- `false` otherwise (no error raised)

**Use Case**: Safe integrity checking (constant-time would be better for production)

---

### Module: `protocol`

Frame structures and encoding protocol.

#### Enum: `ColorValue`

8-color palette for encoding.

```rust
pub enum ColorValue {
    Black = 0,      // (0, 0, 0)
    DarkGray = 1,   // (64, 64, 64)
    Gray = 2,       // (128, 128, 128)
    LightGray = 3,  // (192, 192, 192)
    White = 4,      // (255, 255, 255)
    DarkRed = 5,    // (128, 0, 0)
    DarkBlue = 6,   // (0, 0, 128)
    DarkGreen = 7,  // (0, 128, 0)
}
```

Each color value = 3 bits of data (2^3 = 8 colors).

##### Methods

###### `ColorValue::to_rgb() -> (u8, u8, u8)`

Convert to RGB bytes.

---

###### `ColorValue::to_yuv() -> (u8, u8, u8)`

Convert to YUV values (better for video compression).

---

###### `ColorValue::from_rgb(r: u8, g: u8, b: u8) -> Self`

Find nearest color from RGB values using Euclidean distance.

```rust
let color = ColorValue::from_rgb(255, 0, 0); // Red
// Returns ColorValue::DarkRed (closest match)
```

---

#### Struct: `FrameHeader`

Metadata about frame structure (16 bytes fixed).

```rust
pub struct FrameHeader {
    pub frame_id: u32,              // Unique frame identifier
    pub block_size: u8,             // 4, 8, or 16 pixels
    pub data_blocks_count: u16,     // Number of data blocks
    pub is_first_frame: bool,       // Contains metadata?
    pub protocol_version: u8,       // Protocol version (1)
    pub flags: u8,                  // Reserved
}
```

##### Methods

###### `FrameHeader::new(frame_id, block_size, data_blocks_count, is_first_frame) -> Self`

Create new frame header.

---

###### `FrameHeader::to_bytes(&self) -> [u8; 16]`

Serialize to 16-byte array.

```rust
let header = FrameHeader::new(0, 8, 1000, true);
let bytes: [u8; 16] = header.to_bytes();
```

---

###### `FrameHeader::from_bytes(bytes: &[u8; 16]) -> Result<Self, String>`

Deserialize from 16-byte array.

```rust
let bytes: [u8; 16] = [/* ... */];
let header = FrameHeader::from_bytes(&bytes)?;
```

---

#### Struct: `MetadataBlock`

File metadata stored in first frame (232 bytes fixed).

```rust
pub struct MetadataBlock {
    pub filename: String,         // Original filename
    pub file_size: u64,           // Original file size in bytes
    pub file_hash: [u8; 32],      // SHA-256 of original file
    pub token_id: String,         // Token for decryption
}
```

**Fixed Layout** (232 bytes total):
- bytes 0-127: filename (null-terminated UTF-8)
- bytes 128-135: file_size (little-endian u64)
- bytes 136-167: file_hash ([u8; 32])
- bytes 168-231: token_id (null-terminated vshield://)

##### Constants

###### `MetadataBlock::SIZE: usize = 232`

Always 232 bytes, enables fixed frame layout.

---

##### Methods

###### `MetadataBlock::to_bytes(&self) -> [u8; 232]`

Serialize to fixed-size binary buffer.

```rust
let meta = MetadataBlock {
    filename: "file.bin".to_string(),
    file_size: 1024,
    file_hash: [0u8; 32],
    token_id: "vshield://abc...".to_string(),
};
let bytes: [u8; 232] = meta.to_bytes();
```

**Properties**:
- Null-terminates strings (truncates if too long)
- Fills unused bytes with zeros
- **Never fails** - always returns [u8; 232]

---

###### `MetadataBlock::from_bytes(buf: &[u8]) -> Result<Self, String>`

Deserialize from binary buffer.

```rust
let buf: &[u8] = &[/* ...232 bytes... */];
let meta = MetadataBlock::from_bytes(buf)?;
```

**Requirements**: buf.len() >= 232

**Errors**:
- Buf too short
- UTF-8 decoding fails (used lossy, doesn't error)

---

#### Struct: `DataBlock`

Single encoded data block (3 bits).

```rust
pub struct DataBlock {
    pub color: ColorValue,  // Encoded 3-bit value
    pub size: u8,          // Block size (4, 8, or 16)
}
```

##### Methods

###### `DataBlock::new(size: u8) -> Self`

Create block filled with black.

---

###### `DataBlock::encode(bits: u8, size: u8) -> Self`

Encode 3 bits into a color value.

```rust
let block = DataBlock::encode(0b101, 8); // 5 = DarkRed
assert_eq!(block.color, ColorValue::DarkRed);
```

**Argument**: Only lower 3 bits used, range 0-7

---

###### `DataBlock::decode_from_pixels(pixels: &[u8], stride: usize, x: u32, y: u32, size: u8) -> u8`

Recover color from actual pixel buffer using majority vote.

```rust
let pixels: &[u8] = /* RGB pixel data */;
let color = DataBlock::decode_from_pixels(
    pixels,
    1920,      // stride (width)
    100,       // x coordinate
    50,        //y coordinate
    8,         // block size
);
// Returns 0-7 (color index)
```

**Use**: Decoding from actual PNG pixels

---

###### `DataBlock::decode(&self) -> u8`

Get color value directly (0-7).

```rust
let block = DataBlock::encode(3, 8);
assert_eq!(block.decode(), 3);
```

---

#### Struct: `Frame`

Complete frame with all components.

```rust
pub struct Frame {
    pub header: FrameHeader,
    pub metadata: Option<MetadataBlock>,
    pub data_blocks: Vec<DataBlock>,
    pub frame_width: u32,
    pub frame_height: u32,
    pub pixel_data: Option<Vec<u8>>,  // RGB: 3 bytes/pixel
}
```

##### Methods

###### `Frame::new(frame_id, block_size, width, height, is_first_frame) -> Self`

Create new frame.

```rust
let frame = Frame::new(0, 8, 1920, 1080, true);
```

---

###### `Frame::capacity_bytes(&self) -> usize`

Calculate maximum data capacity in bytes.

```rust
let frame = Frame::new(0, 8, 1920, 1080, true);
let cap = frame.capacity_bytes();
// Result: ~19,200 bytes for 1920×1080 @ 8px blocks
```

**Calculation**: (data_blocks_count * 3 bits) / 8

---

### Module: `anchor`

Pattern generation and detection (finder patterns).

#### Constants

```rust
pub const ANCHOR_BLOCK_SIZE: u8 = 10;  // 10×10 blocks = 80×80 px @ 8px
pub const MAX_ANCHOR_REFINEMENT: u32 = 10;
```

#### Function: `generate_anchor_pattern() -> Vec<Vec<ColorValue>>`

Generate 1:1:3:1:1 pattern (like QR code).

---

### Module: `ecc`

Error correction code (Reed-Solomon).

#### Struct: `ECCConfig`

```rust
pub struct ECCConfig {
    pub redundancy_percent: u8,  // 20-40 recommended
}
```

#### Function: `apply_ecc(data: &[u8], config: &ECCConfig) -> Result<Vec<u8>, String>`

Apply Reed-Solomon error correction.

```rust
let data = b"File info";
let config = ECCConfig { redundancy_percent: 20 };
let coded = apply_ecc(data, &config)?;
// coded.len() > data.len() (added parity)
```

---

#### Function: `recover_from_ecc(coded: &[u8], redundancy_percent: u8) -> Result<Vec<u8>, String>`

Recover original data using ECC.

```rust
// Assume coded has some bit errors
let recovered = recover_from_ecc(&coded, 20)?;
```

---

### Module: `interleave`

Data scattering across frame.

#### Function: `interleave(blocks: &[DataBlock], frame_width: u32, frame_height: u32) -> Vec<(u32, u32)>`

Return pixel coordinates for each block (scattered pattern).

---

#### Function: `de_interleave(coordinates: &[(u32, u32)], frame_width: u32, frame_height: u32) -> Vec<(u32, u32)>`

Reverse interleaving.

---

## CLI Binaries

### `vshield-encode`

```bash
vshield-encode --input FILE --output DIR [--block-size SIZE] [--redundancy PCT]
```

**Arguments**:
- `--input FILE` - Path to file to encode
- `--output DIR` - Output directory for PNG frames
- `--block-size SIZE` - Block size: 4, 8, or 16 (default: 8)
- `--redundancy PCT` - ECC redundancy: 20-40 percent (default: 20)

**Output**:
- `{output}/frame_0000.png` - Encoded frame
- `{output}/metadata.json` - Token and file info

**Example**:
```bash
./vshield-encode --input secret.txt --output /encoded --block-size 8 --redundancy 25
```

### `vshield-decode`

```bash
vshield-decode --input DIR --output FILE --token TOKEN
```

**Arguments**:
- `--input DIR` - Directory containing frame PNG files
- `--output FILE` - Output file path
- `--token TOKEN` - Decryption token (vshield://...)

**Example**:
```bash
./vshield-decode --input /frames --output recovered.txt --token "vshield://abc..."
```

---

## Error Types

### `vshield_core::Error` (Future)

```rust
pub enum Error {
    CryptoError(String),
    ECCError(String),
    FrameError(String),
    TokenError(String),
    IOError(String),
}
```

*Currently using `Result<T, String>` for simplicity*

---

## Constants Reference

| Constant | Value | Purpose |
|----------|-------|---------|
| `DEFAULT_FRAME_WIDTH` | 1920 | Frame width in pixels |
| `DEFAULT_FRAME_HEIGHT` | 1080 | Frame height in pixels |
| `DEFAULT_BLOCK_SIZE` | 8 | Default block size |
| `COLOR_PALETTE_SIZE` | 8 | Color options |
| `ANCHOR_BLOCK_SIZE` | 10 | Finder pattern size |
| `ECC_REDUNDANCY_PERCENT` | 25 | Default redundancy |
| `NONCE_SIZE` | 12 | ChaCha20 nonce bytes |
| `TAG_SIZE` | 16 | Poly1305 auth tag |
| `ENCRYPTION_OVERHEAD` | 28 | nonce + tag |

---

## Testing

All modules include `#[cfg(test)]` sections with unit tests.

Run tests:
```bash
cargo test --lib
```

---

## Versioning

- **API Version**: 1.0
- **Protocol Version**: 1
- **Minimum Rust**: 1.70

---

**Last Updated**: March 2026
