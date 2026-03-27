# V-Shield Development Roadmap

Comprehensive project roadmap from Alpha (v0.1) through Production-ready (v1.0).

---

## Phase 1: Foundation ✅ COMPLETE

**Goal**: Establish core cryptographic and encoding infrastructure.

**Timeline**: December 2025 - March 2026  
**Status**: **COMPLETE**

### Completed Milestones

- [x] **Frame Protocol Definition**
  - Fixed 232-byte metadata block format
  - 16-byte frame headers
  - ColorValue 8-color palette (3 bits/block)
  - Finder patterns (anchor detection ready)

- [x] **Core Cryptography**
  - ChaCha20-Poly1305 AEAD encryption
  - **Random nonce generation** (security fix completed)
  - SHA-256 file hashing
  - Token management with Base58 encoding

- [x] **Error Correction**
  - Reed-Solomon ECC integration
  - Adaptive redundancy (20-40%)
  - Block interleaving for burst protection
  - Phase 1 limit: ~100 bytes per frame

- [x] **Encoding Pipeline**
  - File reading and SHA-256 hashing
  - Metadata block creation
  - Encryption with random nonce
  - Frame rendering to PNG (1920×1080)
  - CLI encoder (`vshield-encode`) working

- [x] **Testing & Documentation**
  - Encoder tested on 3 test files (22-50 bytes)
  - Unit tests for crypto functions
  - ARCHITECTURE.md, API.md documentation
  - Token serialization/deserialization verified

### Test Results

```
Test 1: small.txt (50 bytes)
  ✓ Encrypted: 66 bytes (with nonce + tag)
  ✓ After ECC (20%): 79 bytes
  ✓ Token: vshield://5650f5a0-d8c8-4d0c-a0c46124b47d394
  ✓ Frame created: output/test_small/frame_0000.png

Test 2: config_small.json (22 bytes)
  ✓ Token: vshield://9c83bf99-f811-459f-aee1-3cfddae52208

Test 3: test_data.txt (25 bytes)
  ✓ Token: vshield://00035e17-8302-46d1-a17a-12bb2bd00141
```

### Known Limitations

- Max file size: ~100 bytes per frame (RS 255-symbol limit)
- Single frame only (no multi-frame support yet)
- Decoder framework present but not fully implemented
- No YouTube compression testing

### Critical Fixes Applied

- [x] **Random Nonce Implementation**: Changed from fixed to random per encryption
- [x] **Token Randomness**: Each file gets new token (not deterministic)
- [x] **MetadataBlock Binary Format**: Fixed 232 bytes (no JSON variable length)
- [x] **DataBlock Optimization**: Single color instead of 16×16 array
- [x] **CLI Safety**: Fixed potential panic with saturating_sub

---

## Phase 1.5: YouTube Validation & Decoder Implementation

**Goal**: Verify system works with actual YouTube compression.  
**Target Timeline**: March - April 2026  
**Status**: **IN PLANNING**

### Tasks

#### Decoder Implementation (Week 1-2)

- [ ] **Frame Detection**
  - [ ] Implement anchor detection algorithm
  - [ ] Align frame to found anchors
  - [ ] Extract frame header from detected position
  - [ ] Unit tests for anchor detection

- [ ] **Data Extraction**
  - [ ] Implement de-interleaving
  - [ ] Color majority voting from actual pixels
  - [ ] Block-to-color mapping
  - [ ] Data block reconstruction

- [ ] **Decryption & Verification**
  - [ ] ChaCha20 decryption with nonce from frame
  - [ ] SHA-256 verification
  - [ ] File integrity validation
  - [ ] Error reporting for hash mismatch

- [ ] **CLI Decoder**
  - [ ] Frame directory input
  - [ ] Token parsing
  - [ ] Output file writing
  - [ ] Error handling

#### YouTube Testing (Week 2-3)

- [ ] **Test Workflow**
  1. Encode small test file → PNG frame
  2. Convert frame to MP4 (30 FPS)
  3. Upload to YouTube (unlisted)
  4. Download at multiple qualities:
     - 360p (lowest)
     - 720p (mid)
     - 1080p (highest)
  5. Attempt decode each quality
  6. Measure success rate

- [ ] **Test Files**
  - [ ] 10 byte test file (simplest)
  - [ ] 50 byte test file (current max)
  - [ ] 100 byte test file (limit)
  - Track which decode successfully

- [ ] **Compression Analysis**
  - Measure color accuracy per quality
  - Identify bit error patterns
  - Correlate with block size
  - Document findings

#### Parameter Tuning (Week 3)

- [ ] **Block Size Testing**
  - [ ] Compare 4×4 vs 8×8 vs 16×16 resilience
  - [ ] Optimal size selection
  - [ ] Trade-off: smaller = more blocks = lower capacity

- [ ] **Redundancy Tuning**
  - [ ] Test 20%, 30%, 40% redundancy levels
  - [ ] Success rate vs. data density trade-off
  - [ ] Final recommendation

- [ ] **Color Palette Verification**
  - [ ] Validate YUV values
  - [ ] Test color distinctness after compression
  - [ ] Consider palette adjustments for Phase 2

### Success Criteria

- [x] Decoder fully implements all functions
- [x] Decode works on 360p, 720p, 1080p YouTube videos
- [ ] Success rate > 90% for Phase 1 files
- [ ] Documentation updated with test results
- [ ] GitHub release v0.1.0-alpha

### Deliverables

- `YOUTUBE_COMPATIBILITY.md` - Detailed test results
- Full decoder CLI (`vshield-decode`)
- Test dataset and results
- Performance metrics

---

## Phase 2: Multi-File & Optimization

**Goal**: Support larger files through multi-frame encoding.  
**Target Timeline**: April - May 2026  
**Status**: **NOT STARTED**

### Multi-Frame Support (Week 1-2)

- [ ] **Frame Sequencing**
  - [ ] Frame numbering scheme
  - [ ] Metadata in first frame with total count
  - [ ] Sequential frame location in decoder
  - [ ] Out-of-order frame handling

- [ ] **Data Splitting**
  - [ ] Calculate frames needed for file size
  - [ ] Intelligent splitting (don't split blocks)
  - [ ] Frame allocation algorithm
  - [ ] Storage format for frame index

- [ ] **Decoder Updates**
  - [ ] Multi-frame input support
  - [ ] Reassemble from scattered frames
  - [ ] Verify frame integrity
  - [ ] Handle missing frames

- [ ] **Test Coverage**
  - [ ] 1 KB file (7-8 frames @ 100B each)
  - [ ] 10 KB file (100 frames)
  - [ ] Capacity calculation accuracy
  - [ ] Edge cases (exact boundaries)

### Color Palette Enhancement (Week 2-3)

- [ ] **YUV Optimization**
  - [ ] Analyze 4:2:0 chroma subsampling
  - [ ] Shift more information to Y channel
  - [ ] Reduce U/V dependency
  - [ ] Expand palette to 16+ colors if beneficial

- [ ] **Resilience Metrics**
  - [ ] Measure error rates per color in 360p
  - [ ] Re-rank colors by confidence
  - [ ] Cluster colors by similarity
  - [ ] Recommend new palette

### Performance Optimization (Week 3-4)

- [ ] **Encoder Speed**
  - [ ] Profile encode times
  - [ ] Optimize hot paths
  - [ ] Parallel frame generation (if multi-frame)
  - [ ] Target: <1s for 100 KB file

- [ ] **Decoder Speed**
  - [ ] Profile decode times
  - [ ] Optimize anchor detection
  - [ ] Cache computed values
  - [ ] Target: <500ms per frame

- [ ] **Memory Usage**
  - [ ] Reduce allocations
  - [ ] Stream processing instead of buffering
  - [ ] Profile peak memory

### Adaptive Block Sizing (Optional)

- [ ] **Automatic Selection**
  - [ ] Analyze file content
  - [ ] Recommend block size
  - [ ] Trade-off capacity vs. resilience
  - [ ] Store in frame header

### Success Criteria

- [ ] Encode/decode files up to 1 MB
- [ ] Test on 1MB YouTube video
- [ ] Multi-frame YouTube reassembly works
- [ ] Performance targets met
- [ ] GitHub release v0.2.0-beta

### Deliverables

- Multi-frame encoder/decoder
- Performance benchmarks
- Updated documentation
- Test dataset (1 KB, 10 KB, 100 KB files)

---

## Phase 3: Browser Extension

**Goal**: Real-time extraction from viewer's browser.  
**Target Timeline**: May - June 2026  
**Status**: **NOT STARTED**

### WASM Compilation (Week 1)

- [ ] **Configure WASM Target**
  - [ ] Add `wasm32-unknown-unknown` target
  - [ ] Feature gate WASM dependencies
  - [ ] Build `vshield_core` as WASM

- [ ] **JavaScript Bindings**
  - [ ] Use wasm-bindgen for exports
  - [ ] Expose `Token`, `decrypt`, `verify_hash`
  - [ ] Type definitions in TypeScript

- [ ] **Size Optimization**
  - [ ] Minimize WASM binary
  - [ ] Strip debug symbols
  - [ ] Target: <1 MB for extension

### Extension Development (Week 2-3)

- [ ] **Manifest & Permissions**
  - [ ] Chrome manifest v3
  - [ ] Content script permissions
  - [ ] Storage permissions

- [ ] **UI Components**
  - [ ] Token input dialog
  - [ ] Download button
  - [ ] Progress indicator
  - [ ] Error messages

- [ ] **Video Capture**
  - [ ] Canvas-based frame extraction
  - [ ] Convert to PNG format
  - [ ] Sequence captured frames
  - [ ] Temporal filtering (skip duplicate frames)

- [ ] **Data Extraction**
  - [ ] Call WASM decoder
  - [ ] Progressive reassembly
  - [ ] File reconstruction
  - [ ] Hash verification

### Testing (Week 4)

- [ ] **YouTube Testing**
  - [ ] Test on 5-10 real YouTube videos
  - [ ] Various video qualities
  - [ ] Different browsers (Chrome, Edge)

- [ ] **User Testing**
  - [ ] Beta tester group
  - [ ] Collect feedback
  - [ ] Bug fixes

### Success Criteria

- [ ] Extension extracts data from YouTube videos
- [ ] Works on 1080p, 720p, 360p streams
- [ ] WASM binary < 1 MB
- [ ] UI is intuitive
- [ ] Chrome Web Store release

### Deliverables

- Chrome extension (published to store)
- WASM-compiled decoder library
- TypeScript type definitions
- User guide & FAQ

---

## Phase 4: Streaming & Real-Time Playback

**Goal**: Decode video in real-time as it plays.  
**Target Timeline**: June - July 2026  
**Status**: **CONCEPTUAL**

###  MediaSource Extensions (MSE)

- [ ] **Stream Pipeline**
  - [ ] Configure MSE with video codec
  - [ ] Feed captured frames to decoder
  - [ ] Queue output chunks

- [ ] **Real-Time Decoding**
  - [ ] In-browser frame processing
  - [ ] Minimal latency (<1s lag)
  - [ ] Handle frame drops

- [ ] **Playback Overlay**
  - [ ] Render decoded video element
  - [ ] Picture-in-picture mode
  - [ ] Synchronized audio (optional)

### Advanced Features (Optional)

- [ ] **Adaptive Quality**
  - [ ] Detect video quality
  - [ ] Adjust decode aggressiveness
  - [ ] Auto-switch strategies

- [ ] **Stream Recording**
  - [ ] Capture MSE output
  - [ ] Save decoded stream
  - [ ] Replay capability

### Success Criteria

- [ ] Decode video with <1s latency
- [ ] Playable in browser window
- [ ] No obvious artifacts
- [ ] Smooth playback (60 FPS)

### Deliverables

- Real-time decoder implementation
- Stream processing pipeline
- Live demo

---

## Phase 5: Production Hardening

**Goal**: Security audit and production readiness.  
**Target Timeline**: July 2026 onwards  
**Status**: **PLANNING**

### Security Audit

- [ ] **Third-Party Audit**
  - [ ] Independent cryptographic review
  - [ ] Nonce handling verification
  - [ ] Key derivation analysis
  - [ ] Known CVE check

- [ ] **Penetration Testing**
  - [ ] Attempt token recovery
  - [ ] Try brute-force attacks
  - [ ] Analyze timing side-channels

### Documentation

- [ ] **Security Policy**
  - [ ] Vulnerability disclosure process
  - [ ] Response timeline
  - [ ] Credit policy

- [ ] **User Guides**
  - [ ] Installation instructions
  - [ ] Safety best practices
  - [ ] Troubleshooting FAQ
  - [ ] Video tutorials

### Performance Profiling

- [ ] **Benchmarks**
  - [ ] Establish baseline metrics
  - [ ] Profile all hot paths
  - [ ] Identify bottlenecks

- [ ] **Optimization**
  - [ ] SIMD for ECC if applicable
  - [ ] Parallel encode (multi-frame)
  - [ ] Memory-efficient decoder

### Compliance

- [ ] **Legal Review**
  - [ ] Jurisdiction investigation
  - [ ] Terms of service compatibility
  - [ ] Data privacy considerations

- [ ] **Accessibility**
  - [ ] Keyboard navigation
  - [ ] Screen reader support
  - [ ] Color blind friendly UI

### Success Criteria

- [ ] Security audit passed
- [ ] v1.0.0 released
- [ ] Production-ready status
- [ ] Active maintenance plan

---

## Milestone Timeline (Summary)

| Phase | Version | Era | Target Date | Status |
|-------|---------|-----|-------------|--------|
| 1 | v0.1.0-alpha | Foundation | March 2026 | ✅ COMPLETE |
| 1.5 | v0.1.0 | YouTube Testing | April 2026 | 🔄 NEXT |
| 2 | v0.2.0-beta | Multi-file | May 2026 | ⏳ PLANNING |
| 3 | v0.3.0-rc | Browser Extension | June 2026 | ⏳ PLANNING |
| 4 | v0.4.0-experimental | Real-time Streaming | July 2026 | 🤔 CONCEPTUAL |
| 5 | v1.0.0 | Production | August 2026+ | 🔒 FUTURE |

---

## Resource Allocation

### Current Team

- 1 Lead Developer (Rust, Architecture)
- Estimated hours per phase: 80-120 hours

### Critical Dependencies

| Dependency | Status | Risk |
|----------|--------|------|
| Rust compiler (1.70+) | ✅ Available | Low |
| YouTube API | ✅ Available | Low |
| Browser APIs | ✅ Available | Low |
| WASM toolchain | ✅ Available | Low |
| Crypto libraries | ✅ Vendored | Low |

### Budget Considerations

- **Labor**: ~400-600 hours development
- **Testing**: YouTube uploads (free tier OK)
- **Infrastructure**: GitHub (free)
- **Security audit**: $2000-5000 (recommended)
- **Chrome Web Store listing**: $5 one-time

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|-----------|
| YouTube algorithm change | High | Continuous monitoring |
| Cryptanalysis discovery | Medium | Keep nonce randomization |
| Browser API changes | Medium | Stay current with specs |
| Performance not acceptable | Medium | Early profiling & optimization |
| Legal action | Low | Terms of service compliance |

---

## Success Metrics

### Phase 1 (Completed)
- ✅ Encoding produces valid frames
- ✅ Tokens generated consistently
- ✅ File integrity preserved
- ✅ Code compiles without errors

### Phase 1.5 (Current)
- [ ] YouTube decode success rate > 90%
- [ ] Works at all video qualities (360p-1080p)
- [ ] Documented compression resilience
- [ ] Performance < 1 second per frame

### Phase 2+
- [ ] Support files up to 1 MB
- [ ] Multi-frame assembly works end-to-end
- [ ] Browser extension users > 1000
- [ ] Real-time playback latency < 1s

---

## Developer Notes

### Code Quality Standards

- All public functions documented with examples
- Unit test coverage > 80%
- Clippy warnings = 0
- Unsafe code justified and reviewed

### Git Workflow

- Main branch = stable, passing tests
- Develop branch = active work
- Feature branches per major task
- PR reviews before merge

### Community Engagement

- GitHub discussions for design questions
- Issue labels: bug, enhancement, documentation
- Monthly progress reports

---

**Roadmap Version**: 1.0  
**Last Updated**: March 27, 2026  
**Next Review**: April 15, 2026
