import os
import sys
import platform
import ssl
import urllib.request
import tarfile
import zipfile
import shutil
import io

PDFIUM_RELEASES_URL_TEMPLATE = "https://github.com/bblanchon/pdfium-binaries/releases/latest/download/{archive_name}"

def get_platform_info():
    system = platform.system().lower()
    machine = platform.machine().lower()
    
    if system == "windows":
        archive_name = "pdfium-win-x64.tgz"
        binary_name = "pdfium.dll"
    elif system == "linux":
        archive_name = "pdfium-linux-x64.tgz"
        binary_name = "libpdfium.so"
    elif system == "darwin":
        if "arm" in machine or "aarch64" in machine:
            archive_name = "pdfium-mac-arm64.tgz"
        else:
            archive_name = "pdfium-mac-x64.tgz"
        binary_name = "libpdfium.dylib"
    else:
        raise RuntimeError(f"Unsupported operating system: {system}")
        
    return archive_name, binary_name

def ensure_pdfium(force: bool = False) -> bool:
    script_dir = os.path.dirname(os.path.abspath(__file__))
    root_dir = os.path.abspath(os.path.join(script_dir, ".."))
    resources_dir = os.path.join(root_dir, "src-tauri", "resources")
    os.makedirs(resources_dir, exist_ok=True)
    
    try:
        archive_name, binary_name = get_platform_info()
    except RuntimeError as e:
        print(f"[ensure_pdfium] Warning: {e}", file=sys.stderr)
        return False
        
    target_binary_path = os.path.join(resources_dir, binary_name)
    
    if not force and os.path.exists(target_binary_path) and os.path.getsize(target_binary_path) > 0:
        print(f"[ensure_pdfium] PDFium library '{binary_name}' is present at {target_binary_path}")
        return True
        
    url = PDFIUM_RELEASES_URL_TEMPLATE.format(archive_name=archive_name)
    print(f"[ensure_pdfium] Downloading PDFium shared library from {url}...")
    
    ctx = ssl.create_default_context()
    ctx.check_hostname = False
    ctx.verify_mode = ssl.CERT_NONE
    
    req = urllib.request.Request(
        url,
        headers={"User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64)"}
    )
    
    try:
        with urllib.request.urlopen(req, context=ctx) as resp:
            content = resp.read()
            
        print(f"[ensure_pdfium] Extracting '{binary_name}' from archive ({len(content)} bytes)...")
        extracted = False
        
        if archive_name.endswith(".tgz") or archive_name.endswith(".tar.gz"):
            with tarfile.open(fileobj=io.BytesIO(content), mode="r:gz") as tar:
                for member in tar.getmembers():
                    if os.path.basename(member.name) == binary_name:
                        f = tar.extractfile(member)
                        if f:
                            with open(target_binary_path, "wb") as out_f:
                                shutil.copyfileobj(f, out_f)
                            extracted = True
                            break
        elif archive_name.endswith(".zip"):
            with zipfile.ZipFile(io.BytesIO(content)) as z:
                for name in z.namelist():
                    if os.path.basename(name) == binary_name:
                        with z.open(name) as f, open(target_binary_path, "wb") as out_f:
                            shutil.copyfileobj(f, out_f)
                        extracted = True
                        break
                        
        if extracted:
            print(f"[ensure_pdfium] Successfully installed '{binary_name}' ({os.path.getsize(target_binary_path)} bytes) to {target_binary_path}")
            return True
        else:
            print(f"[ensure_pdfium] Error: Could not find '{binary_name}' inside archive {archive_name}", file=sys.stderr)
            return False
            
    except Exception as e:
        print(f"[ensure_pdfium] Error downloading or extracting PDFium: {e}", file=sys.stderr)
        return False

if __name__ == "__main__":
    force_download = "--force" in sys.argv
    ok = ensure_pdfium(force=force_download)
    sys.exit(0 if ok else 1)
