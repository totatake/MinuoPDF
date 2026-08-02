import io
import fitz
from PIL import Image, ImageDraw

def create_test_pdf(filename):
    print(f"Creating test PDF: {filename}...")
    doc = fitz.open()
    
    # Page 1: Text and color gradient image
    page1 = doc.new_page(width=595, height=842) # A4
    page1.insert_text((50, 50), "PDF Compression Test - Page 1 (Color)", fontsize=20, color=(0.1, 0.2, 0.6))
    page1.insert_text((50, 90), "This text should remain searchable and selectable after compression.", fontsize=11, color=(0.2, 0.2, 0.2))
    page1.insert_text((50, 110), "The image below is a high-resolution color pattern (PNG).", fontsize=11, color=(0.2, 0.2, 0.2))
    
    # Create high-resolution color gradient image (assuming 300dpi, 1200x800)
    img1 = Image.new("RGB", (1200, 800), color=(240, 240, 245))
    draw1 = ImageDraw.Draw(img1)
    # Draw colorful pattern
    for i in range(0, 1200, 15):
        draw1.line([(i, 0), (1200 - i, 800)], fill=(i % 256, 128, 255 - (i % 256)), width=4)
    for i in range(0, 800, 15):
        draw1.line([(0, i), (1200, 800 - i)], fill=(255 - (i % 256), i % 256, 128), width=4)
        
    img_bytes1 = io.BytesIO()
    img1.save(img_bytes1, format="PNG")
    page1.insert_image(fitz.Rect(50, 140, 545, 440), stream=img_bytes1.getvalue())
    
    # Page 2: Text and grayscale shapes
    page2 = doc.new_page(width=595, height=842)
    page2.insert_text((50, 50), "PDF Compression Test - Page 2 (Gray)", fontsize=20, color=(0.3, 0.3, 0.3))
    page2.insert_text((50, 90), "This is page 2. It contains grayscale text and vector-like lines.", fontsize=11, color=(0.3, 0.3, 0.3))
    
    img2 = Image.new("L", (1000, 700), color=255)
    draw2 = ImageDraw.Draw(img2)
    for i in range(10, 350, 15):
        draw2.rectangle([i, i, 1000 - i, 700 - i], outline=100, width=2)
    draw2.ellipse([300, 200, 700, 500], outline=50, width=5)
    
    img_bytes2 = io.BytesIO()
    img2.save(img_bytes2, format="PNG")
    page2.insert_image(fitz.Rect(50, 140, 545, 490), stream=img_bytes2.getvalue())
    
    # Page 3: Page with high density of text (for scanned PDF simulation)
    page3 = doc.new_page(width=595, height=842)
    page3.insert_text((50, 50), "PDF Compression Test - Page 3 (Text Heavy)", fontsize=20, color=(0, 0, 0))
    for i in range(1, 25):
        page3.insert_text((50, 80 + i * 28), f"This is line {i} of sample text to simulate document scanning and rendering features. Mono conversion will affect this page.", fontsize=10, color=(0.1 * (i % 3), 0.1 * (i % 3), 0.1 * (i % 3)))
        
    doc.save(filename)
    doc.close()
    print(f"Successfully generated {filename}")

if __name__ == "__main__":
    create_test_pdf("test_input.pdf")
