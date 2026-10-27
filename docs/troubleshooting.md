# Troubleshooting and platform notes

Carried over from the Voice Workspace README this app started from, renamed for this
app. Upstream issue links are kept as background references.

## Shortcuts stopped working on macOS (Secure Input)

macOS Secure Input blocks other apps from seeing key presses while it is on.
Password fields, some terminals and password managers turn it on, and an app
that crashes can leave it on. While it is on, the dictation shortcut cannot
fire and shortcut changes cannot be recorded; the app shows a warning naming
the app that holds it when macOS reports one.

1. Close the password prompt, or quit the app named in the warning.
2. In Terminal, `ioreg -l -w 0 | grep SecureInput` shows the process ID
   (`kCGSSessionSecureInputPID`) holding it; quitting that process releases it.
3. If nothing obvious holds it, lock the screen and unlock it, or log out and
   back in.

## Known issues and limitations

### Bluetooth Headset Microphones (macOS)

Using a Bluetooth headset microphone on macOS may temporarily reduce playback quality or volume while recording because Bluetooth switches to bidirectional audio. Keep your headphones as the output device and select your Mac's built-in or an external microphone in Voice Workspace to avoid this.

### fn and Globe Key Shortcuts (macOS)

Shortcuts that include the `fn` (Globe) key **only work on Apple keyboards** — your Mac's built-in keyboard or an Apple external keyboard. They will never trigger on a third-party keyboard, even while it is connected to the same Mac.

This is a hardware limitation rather than a Voice Workspace bug. `fn` is not part of the standard USB HID keyboard specification: Apple reports it through a vendor-specific usage that macOS honors only from Apple devices, while third-party keyboards handle their `Fn` key entirely in firmware and send nothing to the computer. There is no event for Voice Workspace to listen for.

If you switch between a MacBook keyboard and an external one, pick a shortcut built from standard modifiers (`ctrl`, `option`, `shift`, `command`) or a regular key instead.

### Linux Notes

**Text Input Tools:**

For reliable text input on Linux, install the appropriate tool for your display server:

| Display Server | Recommended Tool | Install Command                                    |
| -------------- | ---------------- | -------------------------------------------------- |
| X11            | `xdotool`        | `sudo apt install xdotool`                         |
| Wayland        | `wtype`          | `sudo apt install wtype`                           |
| Both           | `dotool`         | `sudo apt install dotool` (requires `input` group) |

- **X11**: Install `xdotool` for both direct typing and clipboard paste shortcuts
- **Ubuntu 26.04**: See [the GNOME Wayland guide](troubleshooting/ubuntu-26-04-gnome-wayland/README.md). Has Wayland display server by default. `wtype` does not work, you need to install `ydotool` and configure systemd as described [here](https://github.com/cjpais/Handy/pull/557#issuecomment-3781249267).
- **Wayland**: Install `wtype` (preferred) or `dotool` for text input to work correctly
- **dotool setup**: Requires adding your user to the `input` group: `sudo usermod -aG input $USER` (then log out and back in)

Without these tools, Voice Workspace falls back to enigo which may have limited compatibility, especially on Wayland.

**Wayland Support (Linux):**

- Limited support for Wayland display server
- Requires [`wtype`](https://github.com/atx/wtype) or [`dotool`](https://sr.ht/~geb/dotool/) for text input to work correctly (see [Linux notes](#linux-notes) for installation)

**Other Notes:**

- **Runtime library dependency (`libgtk-layer-shell.so.0`)**:
  - Voice Workspace links `gtk-layer-shell` on Linux. If startup fails with `error while loading shared libraries: libgtk-layer-shell.so.0`, install the runtime package for your distro:

    | Distro        | Package to install    | Example command                        |
    | ------------- | --------------------- | -------------------------------------- |
    | Ubuntu/Debian | `libgtk-layer-shell0` | `sudo apt install libgtk-layer-shell0` |
    | Fedora/RHEL   | `gtk-layer-shell`     | `sudo dnf install gtk-layer-shell`     |
    | Arch Linux    | `gtk-layer-shell`     | `sudo pacman -S gtk-layer-shell`       |

  - For building from source on Ubuntu/Debian, you may also need `libgtk-layer-shell-dev`.

- The recording overlay is disabled by default on Linux (`Overlay Position: None`) because certain compositors treat it as the active window. When the overlay is visible it can steal focus, which prevents Voice Workspace from pasting back into the application that triggered transcription. If you enable the overlay anyway, be aware that clipboard-based pasting might fail or end up in the wrong window.
- Voice Workspace disables the WebKit DMA-BUF renderer by default on Linux, but preserves explicit `WEBKIT_DISABLE_DMABUF_RENDERER` values. For rendering workarounds, see [Troubleshooting → Linux Startup Crashes or Instability](#linux-startup-crashes-or-instability).
- If Voice Workspace fails to start reliably on Linux, see [Troubleshooting → Linux Startup Crashes or Instability](#linux-startup-crashes-or-instability).
- **Global keyboard shortcuts (Wayland):** On Wayland, system-level shortcuts must be configured through your desktop environment or window manager. Use the [CLI flags](#cli-parameters) as the command for your custom shortcut.

  **GNOME:**
  1. Open **Settings > Keyboard > Keyboard Shortcuts > Custom Shortcuts**
  2. Click the **+** button to add a new shortcut
  3. Set the **Name** to `Toggle Voice Workspace Transcription`
  4. Set the **Command** to `voice-workspace --toggle-transcription`
  5. Click **Set Shortcut** and press your desired key combination (e.g., `Super+O`)

  **KDE Plasma:**
  1. Open **System Settings > Shortcuts > Custom Shortcuts**
  2. Click **Edit > New > Global Shortcut > Command/URL**
  3. Name it `Toggle Voice Workspace Transcription`
  4. In the **Trigger** tab, set your desired key combination
  5. In the **Action** tab, set the command to `voice-workspace --toggle-transcription`

  **Sway / i3:**

  Add to your config file (`~/.config/sway/config` or `~/.config/i3/config`):

  ```ini
  bindsym $mod+o exec voice-workspace --toggle-transcription
  ```

  **Hyprland:**

  Add to your config file (`~/.config/hypr/hyprland.conf`):

  ```ini
  bind = $mainMod, O, exec, voice-workspace --toggle-transcription
  ```

- You can also trigger Voice Workspace externally via Unix signals or the CLI flags, which lets Wayland window managers or other hotkey daemons keep ownership of keybindings:

  | Action                                    | Trigger                                                                      |
  | ----------------------------------------- | ---------------------------------------------------------------------------- |
  | Toggle transcription                      | `pkill -USR2 -n voice-workspace` or `voice-workspace --toggle-transcription` |
  | Toggle transcription with post-processing | `voice-workspace --toggle-post-process`                                      |

  Example Sway config:

  ```ini
  bindsym $mod+o exec pkill -USR2 -n voice-workspace
  bindsym $mod+p exec voice-workspace --toggle-post-process
  ```

  `pkill` here simply delivers the signal—it does not terminate the process.

  > **Behavior change:** older releases also accepted `SIGUSR1` for toggling transcription with post-processing. WebKitGTK — the webview engine embedded in Voice Workspace on Linux — uses SIGUSR1 internally to coordinate JavaScript garbage collection, so listening for it caused phantom recordings and interrupted dictations every few minutes ([#1660](https://github.com/cjpais/Handy/issues/1660)). Voice Workspace no longer listens for SIGUSR1 on Linux; the post-processing toggle is still available via `voice-workspace --toggle-post-process`. **Remove any `pkill -USR1` bindings**: the signal is now delivered straight to WebKit's internal handler and can crash the app.

**Overlay & Pasting Issues (Linux):**

- The recording overlay window can interfere with pasting transcribed text into target applications on Linux (X11)
- **Solution:** Open **Settings > Advanced** and set **"Overlay Position"** to **"None"** to disable the overlay
- Enable **"Audio Feedback"** (also in Advanced) if you still want audible confirmation of recording state
- Users who upgrade from older versions or import settings from other platforms may need to manually apply this change

## Troubleshooting

### Previous Clipboard Content Is Pasted Instead of the Transcription

If the transcription is correct in **History** but Voice Workspace inserts text you copied earlier, see [issue #502](https://github.com/cjpais/Handy/issues/502). With the standard clipboard paste method, Voice Workspace restores your previous clipboard after a fixed delay. Under load, the receiving application may read the clipboard only after that restoration.

1. Open Voice Workspace's settings window and press `Cmd+Shift+D` (macOS) or `Ctrl+Shift+D` (Windows/Linux) to reveal **Debug**.
2. On **macOS and Windows**, try **Reliable Paste (Beta)** in Debug with a clipboard paste method selected. It uses clipboard read notifications to delay restoration instead of relying on the standard fixed delay. Test it in the application where the problem occurs; it is still experimental.
3. If Reliable Paste is disabled or unavailable, increase **Paste Delay (After)** in Debug and test again. This controls the wait before restoring your previous clipboard. **Paste Delay (Before)** controls the wait before sending the paste keystroke and addresses a different part of the operation. These delay settings apply to the standard paste path, not Reliable Paste.

If the problem persists, add your Voice Workspace version, operating system, receiving application, paste method, Reliable Paste setting, and before/after delays to the existing issue. Redact private dictated text before sharing logs.

### Manual Model Installation (For Proxy Users or Network Restrictions)

If you're behind a proxy, firewall, or in a restricted network environment where Voice Workspace cannot download models automatically, you can manually download and install them. The URLs are publicly accessible from any browser.

#### Step 1: Find Your App Data Directory

1. Open Voice Workspace settings
2. Navigate to the **About** section
3. Copy the "App Data Directory" path shown there, or use the shortcuts:
   - **macOS**: `Cmd+Shift+D` to open debug menu
   - **Windows/Linux**: `Ctrl+Shift+D` to open debug menu

The typical paths are:

- **macOS**: `~/Library/Application Support/com.akshatksingh.voiceworkspace/`
- **Windows**: `C:\Users\{username}\AppData\Roaming\com.akshatksingh.voiceworkspace\`
- **Linux**: `~/.config/com.akshatksingh.voiceworkspace/`

#### Step 2: Create Models Directory

Inside your app data directory, create a `models` folder if it doesn't already exist:

```bash
# macOS/Linux
mkdir -p ~/Library/Application\ Support/com.akshatksingh.voiceworkspace/models

# Windows (PowerShell)
New-Item -ItemType Directory -Force -Path "$env:APPDATA\com.akshatksingh.voiceworkspace\models"
```

#### Step 3: Download Model Files

Download the models you want from below

**Whisper Models (single .bin files):**

- Small (487 MB): `https://huggingface.co/ggerganov/whisper.cpp/resolve/5359861c739e955e79d9a303bcbc70fb988958b1/ggml-small.bin`
- Turbo (1600 MB): `https://huggingface.co/ggerganov/whisper.cpp/resolve/5359861c739e955e79d9a303bcbc70fb988958b1/ggml-large-v3-turbo.bin`
- Large (1100 MB): `https://huggingface.co/ggerganov/whisper.cpp/resolve/5359861c739e955e79d9a303bcbc70fb988958b1/ggml-large-v3-q5_0.bin`

**Parakeet Unified EN 0.6B (single `.gguf` file, recommended):**

- Q8_0 (731 MB): `https://huggingface.co/handy-computer/parakeet-unified-en-0.6b-gguf/resolve/main/parakeet-unified-en-0.6b-Q8_0.gguf`

#### Step 4: Install Models

**For Whisper Models (.bin files):**

Simply place the `.bin` file directly into the `models` directory:

```
{app_data_dir}/models/
├── ggml-small.bin
├── ggml-large-v3-turbo.bin
└── ggml-large-v3-q5_0.bin
```

**For GGUF Models (.gguf files):**

Place the `.gguf` file directly into the `models` directory, exactly like the Whisper `.bin` files above. Voice Workspace also picks up models already present in the shared Hugging Face cache (`~/.cache/huggingface/hub`), so a copy downloaded by another tool works without being moved.

**Important Notes:**

- Do not rename the `.bin` or `.gguf` files—use the exact filenames from the download URLs
- After placing the files, restart Voice Workspace to detect the new models

#### Step 5: Verify Installation

1. Restart Voice Workspace
2. Open Settings → Models
3. Your manually installed models should now appear as "Downloaded"
4. Select the model you want to use and test transcription

### Custom Whisper Models

Voice Workspace can auto-discover custom Whisper GGML models placed in the `models` directory. This is useful for users who want to use fine-tuned or community models not included in the default model list.

**How to use:**

1. Obtain a Whisper model in GGML `.bin` format (e.g., from [Hugging Face](https://huggingface.co/models?search=whisper%20ggml))
2. Place the `.bin` file in your `models` directory (see paths above)
3. Restart Voice Workspace to discover the new model
4. The model will appear in the "Custom Models" section of the Models settings page

**Important:**

- Community models are user-provided and may not receive troubleshooting assistance
- The model must be a valid Whisper GGML format (`.bin` file)
- Model name is derived from the filename (e.g., `my-custom-model.bin` → "My Custom Model")

### Linux Startup Crashes or Instability

If Voice Workspace fails to start reliably on Linux — for example, it crashes shortly after launch, never shows its window, or reports a Wayland protocol error — try the steps below in order.

**1. Install (or reinstall) `gtk-layer-shell`**

Voice Workspace uses `gtk-layer-shell` for its recording overlay and links against it at runtime. A missing or broken installation is the most common cause of startup failures and can manifest as a crash or a hang well before any window is shown. Make sure the runtime package is installed for your distro:

| Distro        | Package to install    | Example command                        |
| ------------- | --------------------- | -------------------------------------- |
| Ubuntu/Debian | `libgtk-layer-shell0` | `sudo apt install libgtk-layer-shell0` |
| Fedora/RHEL   | `gtk-layer-shell`     | `sudo dnf install gtk-layer-shell`     |
| Arch Linux    | `gtk-layer-shell`     | `sudo pacman -S gtk-layer-shell`       |

If it is already installed and you still see startup problems, try reinstalling it (e.g. `sudo pacman -S gtk-layer-shell` again) in case the library files were corrupted by a partial upgrade.

**2. Disable the GTK layer shell overlay (`HANDY_NO_GTK_LAYER_SHELL`)**

If installing the library does not help, you can skip `gtk-layer-shell` initialization entirely as a workaround. On some compositors (notably KDE Plasma under Wayland) it has been reported to interact poorly with the recording overlay. With this variable set, the overlay falls back to a regular always-on-top window:

```bash
HANDY_NO_GTK_LAYER_SHELL=1 voice-workspace
```

**3. Configure WebKit DMA-BUF rendering (`WEBKIT_DISABLE_DMABUF_RENDERER`)**

Voice Workspace disables the WebKitGTK DMA-BUF renderer by default on Linux because it can cause blank windows or crashes on some GPU/driver combinations. Explicit environment-variable values are preserved, including in the Nix package. To explicitly keep the renderer disabled:

```bash
WEBKIT_DISABLE_DMABUF_RENDERER=1 voice-workspace
```

Some users on WebKitGTK 2.54 report a flickering or partially drawn recording overlay with the renderer disabled ([#2166](https://github.com/cjpais/Handy/issues/2166)). If affected, try opting out of the default workaround:

```bash
WEBKIT_DISABLE_DMABUF_RENDERER=0 voice-workspace
```

This allows WebKit to use the renderer when supported, but can reintroduce the GPU/driver-specific crashes described above. Remove the override or use `=1` if that happens.

Fully quit Voice Workspace, including its tray instance, before testing either value. Running a second instance with `--toggle-transcription` does not change the running process's environment.

**Making a workaround permanent**

Set the variable in the environment of the launcher that actually starts Voice Workspace. Shell profile exports may not apply to desktop-menu or autostart launches. If you launch Voice Workspace from a `.desktop` file, you can prefix the `Exec=` line, e.g.:

```ini
Exec=env HANDY_NO_GTK_LAYER_SHELL=1 voice-workspace
```

Voice Workspace's built-in autostart rewrites its generated desktop entry, so editing that entry is not a durable workaround. For autostart, use an appropriate session/service environment, or disable Voice Workspace's built-in autostart and manage your own startup entry with the required variable.

### Empty recording overlay on Hyprland / Omarchy

If the recording overlay is empty or bordered, fully quit Voice Workspace and launch a
[native installation](BUILD.md#linux-install-from-source) with Wayland enabled:

```bash
env -u HANDY_NO_GTK_LAYER_SHELL GDK_BACKEND=wayland voice-workspace
```

If this works, apply `GDK_BACKEND=wayland` only to Voice Workspace's launcher. This
workaround does not work with the 0.9.6 AppImage, which forces X11.

### Vulkan Overlays and Capture Tools on Windows (`HANDY_KEEP_VULKAN_IMPLICIT_LAYERS`)

On Windows, Voice Workspace asks the Vulkan loader to skip implicit layers to avoid crashes caused by overlay and capture hooks ([#2049](https://github.com/cjpais/Handy/issues/2049)). GPU acceleration remains enabled; this does not change system-wide settings.

To opt out for GPU selection or debugging tools, fully quit Voice Workspace (including the tray icon), then run both commands in the same PowerShell window:

```powershell
$env:HANDY_KEEP_VULKAN_IMPLICIT_LAYERS = "1"
& "$env:ProgramFiles\Voice Workspace\voice-workspace.exe"
```

Adjust the executable path if needed. This override only applies to apps launched from that PowerShell session, not the Start menu. Voice Workspace also preserves any existing `VK_LOADER_LAYERS_DISABLE` value.

## Debug mode

Voice Workspace includes an advanced debug mode for development and troubleshooting. Access it by pressing:

- **macOS**: `Cmd+Shift+D`
- **Windows/Linux**: `Ctrl+Shift+D`

## Command-line flags

Voice Workspace supports command-line flags for controlling a running instance and customizing startup behavior. These work on all platforms (macOS, Windows, Linux). Largely this is a beta feature.

**Remote control flags** (sent to an already-running instance via the single-instance plugin):

```bash
voice-workspace --toggle-transcription    # Toggle recording on/off
voice-workspace --toggle-post-process     # Toggle recording with post-processing on/off
voice-workspace --cancel                  # Cancel the current operation
```

**Startup flags:**

```bash
voice-workspace --start-hidden            # Start without showing the main window
voice-workspace --no-tray                 # Start without the system tray icon
voice-workspace --debug                   # Enable debug mode with verbose logging
voice-workspace --help                    # Show all available flags
```

Flags can be combined for autostart scenarios:

```bash
voice-workspace --start-hidden --no-tray
```

> **macOS tip:** When Voice Workspace is installed as an app bundle, invoke the binary directly:
>
> ```bash
> "/Applications/Voice Workspace.app/Contents/MacOS/voice-workspace" --toggle-transcription
> ```
