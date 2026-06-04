# TODO

## P1 - Security

### Forgot password reveals whether email is registered

`forgot` dùng `.one_or_404()` -> trả lỗi khác nhau tuỳ email tồn tại hay không -> attacker enumerate được registered emails.

Fix: luôn trả success; chỉ gửi OTP nếu email tồn tại (silent no-op khi email không tồn tại).

---

### Password change không invalidate session cũ

`forgot_resolve` tạo session mới nhưng không xoá các session hiện có của user đó. Attacker giữ session hợp lệ trước khi nạn nhân đổi password -> vẫn còn access.

Fix: `LoginSession::delete_many().filter(UserId.eq(user_id)).exec(tx)` trước khi tạo session mới.

---

### Cookie chưa có security flags

`set_cookie_login_session` không set `HttpOnly`, `Secure`, `SameSite`. Thiếu `HttpOnly` -> XSS đọc được cookie. Thiếu `Secure` -> gửi qua HTTP plain. Thiếu `SameSite` -> CSRF.

Fix: kiểm tra `_http` layer, enforce các flags này trong `set_cookie`.

---

## P2 - Correctness / DX

### `authOtpResolve` không consume OTP và reset attempt counter

Sau khi validate đúng qua `authOtpResolve`, OTP vẫn còn trong DB và `total_attempt` bị reset về 0. Về lý thuyết attacker có thể dùng lại OTP hoặc tận dụng reset để brute-force theo batch.

Option A: xoá OTP sau khi `authOtpResolve` validate thành công.
Option B: bỏ hẳn endpoint `authOtpResolve` nếu không có use-case thực tế.

---

### `Role.operations` là raw `JsonValue` - không validate lúc insert

Structure sai chỉ phát hiện lúc runtime khi authz check. Nên validate khi insert bằng cách deserialize sang `PolicyOperations` trước khi lưu.

---

### Error code fragile khi rename Rust variant

`e.0.code()` trả string tên variant -> rename = breaking change cho client. Cần attribute stable code riêng:

```rust
#[code = "NOT_FOUND"]   // decoupled từ Rust identifier
NotFound,
```

---

### IP logging có thể sai sau proxy

`ctx.get_ip()` có thể log IP của proxy thay vì IP thật nếu deploy sau nginx/load balancer. Cần document hoặc support trusted proxy / `X-Forwarded-For` config.

---

### Không có rate limiting hooks

`register` và `forgot` có thể bị spam với các email khác nhau (`otp_ensure_re_request` chỉ throttle per-email). Framework không cung cấp hook nào để attach IP-based rate limiting.

Thêm `rate_limit_check` vào `AuthHandlers`:

```rust
async fn rate_limit_check(&self, ctx: &Context<'_>, op: &str) -> Res<()> { Ok(()) }
```

---

## P3 - Architecture / Enhancement

### Pagination không trả `total_count`

`#[search]` trả `Vec<TodoGql>` -> client phải gọi thêm `#[count]` riêng để biết total. Hai DB round-trip thay vì một. Nên có connection type:

```rust
pub struct SearchResult<T> { pub nodes: Vec<T>, pub total_count: u64 }
```

---

### `created_by_id` / `updated_by_id` phải set tay ở mọi resolver

Framework auto-add `*_by_id` fields nhưng không tự fill từ auth context. Mỗi resolver phải lấy thủ công. Cần cơ chế để `am_create!` / `am_update!` tự inject từ `ctx.auth()` khi `grand_line_auth` được dùng, thông qua hook trong `GrandLineExtension`.

---

### Session storage chỉ là DB - không có pluggable backend

Mỗi authenticated request phải query DB để verify session. Ở scale cao đây là bottleneck. Nên tách `SessionStore` thành trait riêng (tương tự `AuthUserHandlers`) để có thể swap sang Redis hoặc backend khác.

---

### Không có pre/post resolver hooks

Không có chỗ attach cross-cutting concerns như audit log, metrics, custom validation. Cần `ResolverHooks` trait trong `GrandLineExtension`:

```rust
async fn before_resolver(&self, ctx: &Context<'_>, name: &str) -> Res<()> { Ok(()) }
async fn after_resolver(&self, ctx: &Context<'_>, name: &str) -> Res<()> { Ok(()) }
```

---

### `ua: JsonValue` không có schema

`LoginSession.ua` stored như raw JSON không định nghĩa rõ. Nên define struct `UserAgent` với các fields cụ thể để có type safety và documentable.

---

### Authz không có built-in Org CRUD

Auth có `AuthMergedMutation<U>` với register/login/forgot. Authz không có resolver built-in nào cho create/update/delete org, role, hay user-in-role. User phải tự viết toàn bộ. Nên có `AuthzMergedMutation<O>` với org CRUD basics.

---

### Thiếu integration test auth + authz cùng nhau

`tests/auth/*` và `tests/authz/*` test riêng lẻ. Không có test nào cover full flow: register -> login -> dùng session để call authz-guarded resolver. Cần ít nhất một integration test end-to-end.
