🎯 Mục tiêu cốt lõi

Xây dựng một ứng dụng web tải video (TikTok, YouTube) có hiệu năng tối thượng. Thay vì lưu trữ file cồng kềnh như các hệ thống cũ, dự án này hoạt động như một "đường ống tàng hình" (Stream Proxy) ở tầng Kernel, cho phép hàng chục nghìn người dùng tải video ngay lập tức với độ trễ gần như bằng 0 và không tiêu tốn dung lượng ổ cứng máy chủ.

🏗️ Kiến trúc Tổng thể (Architecture)

Dự án được chia làm 2 phần tách biệt hoàn toàn:

1. Frontend: "Lớp Vỏ Tối Giản"

Công nghệ: HTML/JS thuần, React hoặc Vue (dạng Static Site).

Giao diện: Cực kỳ đơn giản với 1 ô nhập Link và nút "Download".

Hoạt động: Không xử lý logic nặng. Chỉ gọi API đến Backend và nhận luồng dữ liệu (stream) trả về để ép trình duyệt bật hộp thoại tải file ngay lập tức.

Chi phí: Có thể host miễn phí trên Cloudflare Pages, Vercel hoặc S3.

2. Backend: "Động Cơ Quái Vật" (The Rust Engine)

Đây là trái tim của hệ thống, được viết hoàn toàn bằng Rust kết hợp với các System Call cấp thấp nhất của Linux.

Extraction Layer (Tách Link): Tích hợp engine JavaScript siêu nhẹ (như rquickjs) ngay trong Rust. Khi thuật toán của TikTok/YouTube thay đổi, chỉ cần update file .js mà không cần biên dịch lại toàn bộ server.

Networking Layer (Xử lý Mạng): Sử dụng kiến trúc Thread-per-Core (với Glommio/Monoio) kết hợp io_uring. Không dùng luồng ảo, mỗi kết nối của người dùng được gắn chặt vào một nhân CPU vật lý độc lập để xử lý I/O không khóa (lock-free).

Proxy Layer (Đường Ống Zero-Copy): Sử dụng tuyệt kỹ splice() và kTLS. Dữ liệu tải từ máy chủ nền tảng gốc sẽ chạy thẳng qua Card mạng -> Kernel -> Card mạng -> Trình duyệt người dùng. Hoàn toàn bypass (bỏ qua) RAM của ứng dụng.

⚡ Xử lý Media Tốc độ cao (On-the-fly Processing)

Đối với các ca khó yêu cầu can thiệp vào file (như ghép Audio + Video 4K của YouTube, hoặc xóa Watermark TikTok):

Luồng dữ liệu sẽ được đẩy trực tiếp vào bộ đệm RAM (Ring Buffers).

Hệ thống không dùng FFmpeg gọi từ bên ngoài, mà gọi trực tiếp các thư viện C/C++ cấp thấp qua Rust FFI.

Tối ưu Phần cứng (Hardware Offloading): Kiến trúc này được sinh ra để tận dụng tối đa sức mạnh của các hệ thống Home Server bare-metal. Phần Networking xử lý hàng nghìn kết nối sẽ vắt kiệt sức mạnh đa luồng của các dòng CPU cấp máy chủ (như Threadripper), trong khi các tác vụ nặng như muxing/encoding video ngay trên bộ nhớ đệm sẽ được đẩy thẳng sang VRAM để xử lý bằng NVENC/NVDEC pipeline trên các GPU mạnh mẽ (như RTX 3090).

💰 Lợi thế Cạnh tranh (Selling Points)

Zero-Storage: Tốn 0 đồng chi phí mua ổ cứng lưu trữ video.

Instant Download: Người dùng dán link là trình duyệt tải ngay, không có thanh tiến trình "Đang xử lý trên server...".

Privacy: Hoạt động như một Proxy bảo vệ IP người dùng cuối, đồng thời vượt qua các cơ chế chặn tải trực tiếp của nền tảng.

Self-Hosted Ready: Không cần phụ thuộc vào Cloud VPS đắt đỏ, hệ thống có thể gói gọn thành 1 file binary duy nhất để deploy trực tiếp trên máy chủ cá nhân tại nhà.