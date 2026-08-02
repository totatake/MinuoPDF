import os
import sys
import ssl
import urllib.request

FIXTURE_URLS = {
    # Font embedding samples from https://note.cman.jp/pdf/create/font/
    "sample_font_yes.pdf": "https://note.cman.jp/pdf/create/sample/font/sample_font_yes.pdf",
    "sample_font_no.pdf": "https://note.cman.jp/pdf/create/sample/font/sample_font_no.pdf",
    "sample_font_img.pdf": "https://note.cman.jp/pdf/create/sample/font/sample_font_img.pdf",
    "sample_font_cube_yes.pdf": "https://note.cman.jp/pdf/create/sample/font/sample_font_cube_yes.pdf",
    "sample_font_just_yes.pdf": "https://note.cman.jp/pdf/create/sample/font/sample_font_just_yes.pdf",
    "sample_font_word_yes.pdf": "https://note.cman.jp/pdf/create/sample/font/sample_font_word_yes.pdf",
    "sample_font_ikinari_yes.pdf": "https://note.cman.jp/pdf/create/sample/font/sample_font_ikinari_yes.pdf",
    "sample_font_ikinari_no.pdf": "https://note.cman.jp/pdf/create/sample/font/sample_font_ikinari_no.pdf",
    
    # Version samples from https://note.cman.jp/pdf/create/version/
    "sample_ver13.pdf": "https://note.cman.jp/pdf/create/sample/version/sample_ver13.pdf",
    "sample_ver14.pdf": "https://note.cman.jp/pdf/create/sample/version/sample_ver14.pdf",
    "sample_ver15.pdf": "https://note.cman.jp/pdf/create/sample/version/sample_ver15.pdf",
    "sample_ver16.pdf": "https://note.cman.jp/pdf/create/sample/version/sample_ver16.pdf",
    "sample_ver17.pdf": "https://note.cman.jp/pdf/create/sample/version/sample_ver17.pdf",
    "sample_ver20.pdf": "https://note.cman.jp/pdf/create/sample/version/sample_ver20.pdf",
    "sample_X-1a2003.pdf": "https://note.cman.jp/pdf/create/sample/version/sample_X-1a2003.pdf",
    "sample_A-1b.pdf": "https://note.cman.jp/pdf/create/sample/version/sample_A-1b.pdf",
}

def get_url_for_filename(filename: str) -> str:
    if filename in FIXTURE_URLS:
        return FIXTURE_URLS[filename]
    if filename.startswith("sample_font_"):
        return f"https://note.cman.jp/pdf/create/sample/font/{filename}"
    if filename.startswith("sample_ver") or filename.startswith("sample_X-") or filename.startswith("sample_A-"):
        return f"https://note.cman.jp/pdf/create/sample/version/{filename}"
    # Default fallback to sample/font/
    return f"https://note.cman.jp/pdf/create/sample/font/{filename}"

def download_fixture(filename: str, fixtures_dir: str) -> bool:
    target_path = os.path.join(fixtures_dir, filename)
    if os.path.exists(target_path) and os.path.getsize(target_path) > 0:
        return True

    url = get_url_for_filename(filename)
    print(f"Downloading missing test fixture '{filename}' from {url}...")

    os.makedirs(fixtures_dir, exist_ok=True)

    # SSL context creation for platforms with missing root cert stores
    ctx = ssl.create_default_context()
    ctx.check_hostname = False
    ctx.verify_mode = ssl.CERT_NONE

    req = urllib.request.Request(
        url,
        headers={"User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64)"}
    )

    try:
        with urllib.request.urlopen(req, context=ctx) as resp:
            data = resp.read()
            with open(target_path, "wb") as f:
                f.write(data)
        print(f"Successfully downloaded '{filename}' ({len(data)} bytes) to {target_path}")
        return True
    except Exception as e:
        print(f"Failed to download '{filename}' from {url}: {e}", file=sys.stderr)
        return False

def ensure_fixtures(target_file: str = None) -> bool:
    # Find project root directory
    script_dir = os.path.dirname(os.path.abspath(__file__))
    root_dir = os.path.abspath(os.path.join(script_dir, ".."))
    fixtures_dir = os.path.join(root_dir, "tests", "fixtures")

    os.makedirs(fixtures_dir, exist_ok=True)

    if target_file:
        filename = os.path.basename(target_file)
        return download_fixture(filename, fixtures_dir)

    success = True
    for filename in FIXTURE_URLS:
        if filename.startswith("sample_font_"):
            if not download_fixture(filename, fixtures_dir):
                success = False
    return success

if __name__ == "__main__":
    requested_file = None
    if len(sys.argv) > 1:
        if sys.argv[1] == "--file" and len(sys.argv) > 2:
            requested_file = sys.argv[2]
        else:
            requested_file = sys.argv[1]

    ok = ensure_fixtures(requested_file)
    sys.exit(0 if ok else 1)
