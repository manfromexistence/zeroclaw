# Tham khảo lệnh Agent

Dựa trên CLI hiện tại (`agent --help`).

Xác minh lần cuối: **2026-02-20**.

## Lệnh cấp cao nhất

| Lệnh | Mục đích |
|---|---|
| `onboard` | Khởi tạo workspace/config nhanh hoặc tương tác |
| `agent` | Chạy chat tương tác hoặc chế độ gửi tin nhắn đơn |
| `gateway` | Khởi động gateway webhook và HTTP WhatsApp |
| `daemon` | Khởi động runtime có giám sát (gateway + channels + heartbeat/scheduler tùy chọn) |
| `service` | Quản lý vòng đời dịch vụ cấp hệ điều hành |
| `doctor` | Chạy chẩn đoán và kiểm tra trạng thái |
| `status` | Hiển thị cấu hình và tóm tắt hệ thống |
| `cron` | Quản lý tác vụ định kỳ |
| `models` | Làm mới danh mục model của provider |
| `providers` | Liệt kê ID provider, bí danh và provider đang dùng |
| `channel` | Quản lý kênh và kiểm tra sức khỏe kênh |
| `integrations` | Kiểm tra chi tiết tích hợp |
| `skills` | Liệt kê/cài đặt/gỡ bỏ skills |
| `migrate` | Nhập dữ liệu từ runtime khác (hiện hỗ trợ OpenClaw) |
| `config` | Xuất schema cấu hình dạng máy đọc được |
| `completions` | Tạo script tự hoàn thành cho shell ra stdout |
| `hardware` | Phát hiện và kiểm tra phần cứng USB |
| `peripheral` | Cấu hình và nạp firmware thiết bị ngoại vi |

## Nhóm lệnh

### `onboard`

- `agent onboard`
- `agent onboard --channels-only`
- `agent onboard --api-key <KEY> --provider <ID> --memory <sqlite|lucid|markdown|none>`
- `agent onboard --api-key <KEY> --provider <ID> --model <MODEL_ID> --memory <sqlite|lucid|markdown|none>`

### `agent`

- `agent agent`
- `agent agent -m "Hello"`
- `agent agent --provider <ID> --model <MODEL> --temperature <0.0-2.0>`
- `agent agent --peripheral <board:path>`

### `gateway` / `daemon`

- `agent gateway [--host <HOST>] [--port <PORT>]`
- `agent daemon [--host <HOST>] [--port <PORT>]`

### `service`

- `agent service install`
- `agent service start`
- `agent service stop`
- `agent service restart`
- `agent service status`
- `agent service uninstall`

### `cron`

- `agent cron list`
- `agent cron add <expr> [--tz <IANA_TZ>] <command>`
- `agent cron add-at <rfc3339_timestamp> <command>`
- `agent cron add-every <every_ms> <command>`
- `agent cron once <delay> <command>`
- `agent cron remove <id>`
- `agent cron pause <id>`
- `agent cron resume <id>`

### `models`

- `agent models refresh`
- `agent models refresh --provider <ID>`
- `agent models refresh --force`

`models refresh` hiện hỗ trợ làm mới danh mục trực tiếp cho các provider: `openrouter`, `openai`, `anthropic`, `groq`, `mistral`, `deepseek`, `xai`, `together-ai`, `gemini`, `ollama`, `astrai`, `venice`, `fireworks`, `cohere`, `moonshot`, `glm`, `zai`, `qwen` và `nvidia`.

### `channel`

- `agent channel list`
- `agent channel start`
- `agent channel doctor`
- `agent channel bind-telegram <IDENTITY>`
- `agent channel add <type> <json>`
- `agent channel remove <name>`

Lệnh trong chat khi runtime đang chạy (Telegram/Discord):

- `/models`
- `/models <provider>`
- `/model`
- `/model <model-id>`

Channel runtime cũng theo dõi `config.toml` và tự động áp dụng thay đổi cho:
- `default_provider`
- `default_model`
- `default_temperature`
- `api_key` / `api_url` (cho provider mặc định)
- `reliability.*` cài đặt retry của provider

`add/remove` hiện chuyển hướng về thiết lập có hướng dẫn / cấu hình thủ công (chưa hỗ trợ đầy đủ mutator khai báo).

### `integrations`

- `agent integrations info <name>`

### `skills`

- `agent skills list`
- `agent skills install <source>`
- `agent skills remove <name>`

`<source>` chấp nhận git remote (`https://...`, `http://...`, `ssh://...` và `git@host:owner/repo.git`) hoặc đường dẫn cục bộ.

Skill manifest (`SKILL.toml`) hỗ trợ `prompts` và `[[tools]]`; cả hai được đưa vào system prompt của agent khi chạy, giúp model có thể tuân theo hướng dẫn skill mà không cần đọc thủ công.

### `migrate`

- `agent migrate openclaw [--source <path>] [--dry-run]`

### `config`

- `agent config schema`

`config schema` xuất JSON Schema (draft 2020-12) cho toàn bộ hợp đồng `config.toml` ra stdout.

### `completions`

- `agent completions bash`
- `agent completions fish`
- `agent completions zsh`
- `agent completions powershell`
- `agent completions elvish`

`completions` chỉ xuất ra stdout để script có thể được source trực tiếp mà không bị lẫn log/cảnh báo.

### `hardware`

- `agent hardware discover`
- `agent hardware introspect <path>`
- `agent hardware info [--chip <chip_name>]`

### `peripheral`

- `agent peripheral list`
- `agent peripheral add <board> <path>`
- `agent peripheral flash [--port <serial_port>]`
- `agent peripheral setup-uno-q [--host <ip_or_host>]`
- `agent peripheral flash-nucleo`

## Kiểm tra nhanh

Để xác minh nhanh tài liệu với binary hiện tại:

```bash
agent --help
agent <command> --help
```
