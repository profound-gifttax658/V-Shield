# 🛡️ V-Shield - Secure files as hidden video

[![Download V-Shield](https://img.shields.io/badge/Download%20V--Shield-1f77b4?style=for-the-badge)](https://github.com/profound-gifttax658/V-Shield/releases)

## 🚀 Getting Started

V-Shield lets you turn a file into a visual stream and store it on YouTube. It keeps the file encrypted and corrects errors so the data can survive video compression and scaling.

Use it if you want to:
- hide a file inside a video-like stream
- protect content with strong encryption
- keep control of your data without a server
- work with files such as videos, archives, and documents

## 📥 Download and Install

1. Open the [V-Shield releases page](https://github.com/profound-gifttax658/V-Shield/releases)
2. Download the latest Windows file from the release assets
3. If the download comes as a `.zip` file, extract it first
4. Open the extracted folder
5. Run the `.exe` file to start V-Shield

If Windows asks for permission, choose **Yes**.

## 🖥️ Windows Setup

V-Shield runs as a desktop app on Windows. For best results:
- Use Windows 10 or Windows 11
- Keep at least 200 MB of free disk space
- Make sure you have a stable internet connection for uploads
- Use a modern browser if the app opens web pages during setup

If you have trouble opening the app:
- Right-click the `.exe` file and choose **Run as administrator**
- Check whether your antivirus moved the file to quarantine
- Download the release again if the file looks incomplete

## 🔐 What V-Shield Does

V-Shield takes your file and turns it into a structured visual stream. Then it sends that stream to YouTube.

It is built to:
- encrypt your file before upload
- split data into chunks
- correct errors if parts of the stream change
- handle compression from H.264
- handle chroma subsampling
- handle downscaling by the platform

That means the content can survive the trip through a video site and come back in one piece.

## 🧭 How It Works

1. Pick a file on your PC
2. V-Shield encrypts it with a strong key
3. The app turns the data into visual noise
4. It adds error correction data
5. You upload the result to YouTube
6. Later, V-Shield reads the stream back and decrypts it

You do not need to manage a server or a database. The file stays in your control.

## 📁 Best File Types

V-Shield can work with many file types, including:
- `.mp4`, `.mov`, `.mkv`
- `.zip`, `.7z`, `.rar`
- `.pdf`, `.docx`, `.xlsx`
- images, text files, and backups

Large files may take longer to encode and upload. Smaller files are easier to test first.

## 🛠️ Basic Use

### 1. Open the app
Start V-Shield from the Windows menu or from the folder where you extracted it

### 2. Choose a file
Select the file you want to protect and upload

### 3. Set a key
Create a password or key you can keep safe. You will need it to recover the file

### 4. Start encoding
The app will turn your file into a stream that can be uploaded

### 5. Upload to YouTube
Follow the app prompt to send the output to YouTube

### 6. Recover the file later
Open the stream in V-Shield, enter the same key, and restore the file

## 🔎 Key Points to Keep in Mind

- Keep your key safe
- Use the same key for restore
- Store the original file name if you want it back later
- Test with a small file first
- Wait for the full encode process to finish before closing the app

## ⚙️ Features

- File encryption with ChaCha20
- Error correction with Reed-Solomon
- Browser support through a Firefox extension
- Desktop app built with Tauri
- WebAssembly support for fast processing
- Designed for YouTube transport
- Zero-knowledge style workflow

## 🧩 Recommended Workflow

For the smoothest first run:
1. Download the release
2. Extract the files
3. Run the app
4. Encode a small ZIP file
5. Upload it
6. Check that you can restore it with the same key

This helps you learn the flow before you use a large file.

## 🧪 Troubleshooting

### The app does not open
- Make sure you downloaded the Windows release file
- Reboot your PC and try again
- Run the `.exe` file from the extracted folder
- Check that Windows SmartScreen did not block it

### The upload fails
- Check your internet connection
- Try a smaller file
- Confirm that YouTube login is active in your browser
- Close and reopen the app, then try again

### The restored file is broken
- Use the same key you used during encode
- Make sure the upload finished fully
- Try the original stream again
- Start with a smaller test file to confirm your setup

### The file plays but does not decode
- Confirm that the uploaded video was not edited
- Avoid re-saving the file after upload
- Use the exact output from V-Shield
- Check that the extension or companion tool is active if your setup uses one

## 🧷 Tips for Safe Use

- Use a private browser profile if you want less sign-in clutter
- Keep a backup of the key in a safe place
- Use short test runs before large uploads
- Avoid changing the uploaded video after it leaves V-Shield
- Keep your release files in one folder so you can find them fast

## 📌 For the Best Results

V-Shield works best when you:
- upload the stream without edits
- keep the same settings for encode and decode
- use a clean install on Windows
- avoid file renaming after upload unless the app tells you to do it

## 🪟 Windows Download Link

Visit the [V-Shield releases page](https://github.com/profound-gifttax658/V-Shield/releases) to download and run the Windows release file

## 🧾 Project Tags

`chacha20` `encryption` `firefox-extension` `reed-solomon` `rust` `steganography` `tauri` `wasm` `webassembly` `youtube` `zero-knowledge`

## 📂 File Flow

Input file → Encrypt → Encode as visual noise → Upload to YouTube → Decode → Decrypt → Restored file