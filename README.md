# dlt698_rs

DLT698(电力用户用电信息采集系统)通信协议族(DLT/645 系列扩展,DES 相关可参考 DLT 698.45)的 Rust 实现。以库(crate)形式提供,供上层采集服务(如 `hplc_manage`)复用,负责协议帧、APDU 的编解码与分片重组。

## 功能特性

- **帧层(Frame)**:`68 ... 16` 帧格式的构造与解析,含长度域、控制域、地址域、FCS16 校验和、帧尾。
- **APDU 编解码**:支持链路(Link)、客户端(Client)、服务端(Server)、安全(Security)四类 APDU。
- **应用服务**:Connect / Release / Get / Set / Action / Report / Proxy / Link / Security 等服务的请求与响应编解码。
- **数据单元**:OAD(对象属性描述符)、TI、DAR、Data、DateTime、MS、Region、安全数据单元(MAC/RN/SID/SIDMAC)、COMDCB、描述符(attribute/selector)等。
- **AXDR 编码**:通过 `asn1_type` 实现 DLT698 AXDR 基本类型,通过 `axdr_macro` 过程宏自动派生结构体的编解码。
- **长帧分片**:接收方向分片合并(`combine_fragment`)、发送方向分片确认(`fragment_response`),帧解析支持字节流中跨包/流式解析。
- **校验与健壮性**:头部与整帧双重 FCS16 校验,解析失败返回可读错误;错误类型经 `thiserror` 定义。

## 架构设计

### Workspace 结构

本项目为 Cargo workspace,含 3 个成员:

```
dlt698_rs/
├── Cargo.toml          # workspace 定义
├── src/                # 主 crate:dlt698_rs
│   ├── lib.rs          # 顶层导出(Apdu / Frame / axdr 宏)
│   ├── frame.rs        # 帧层:Header / CtrlField / AddressField / UserData / Fragment
│   ├── checksum.rs     # FCS16 校验表与计算
│   └── apdu/
│       ├── mod.rs      # Apdu 枚举 + 标签分发解析
│       ├── protocol.rs # Client/Server/Link/Security APDU 与应用服务枚举
│       ├── data_unit.rs # 数据单元(PIID、Float32/64、TSA 等)
│       └── protocol/   # 各服务实现:get / set / action / report / proxy / connect / link / release / security
│       └── data_unit/  # 数据单元实现:OAD/descriptor、date_time、dar、data、ms、region、com、security
├── asn1_type/          # AXDR 基本类型:Boolean/Integer/UnsignedInteger/OctetString/BitString/
│                       #   Enumerated/Null/字符串/SequenceOf/Optional + traits(ToAxdr/FromAxdr)
└── axdr_macro/         # 过程宏:AxdrSequence / ToAxdrSequence / IntoData
```

### 帧格式

```
+--------+------+------+-------------+--------+-----------------+--------+------+
| 0x68   | 长度域(2B) | 控制域(1B) | 地址域(2+len) | 头部校验(2B) | 用户数据 | 尾部校验(2B) | 0x16 |
+--------+------+------+-------------+--------+-----------------+--------+------+
```

- **控制域**:`modular_bitfield` 位域实现,含功能码(链路管理/用户数据)、方向(Dir)、请求标志(PRM)、加扰、分片标志。
- **地址域**:首字节高 2 位地址类型、次 2 位逻辑地址、低 4 位地址长度,地址字节逆序存放,末字节为客户端地址。
- **用户数据**:非分片时为 AXDR 编码的 APDU;分片时携带格式域(标签 Start/Middle/End/Confirm + 序号)与分片内容。

### APDU 类型与标签

| APDU 类别 | 标签范围 | 说明 |
|-----------|---------|------|
| Link | 1 / 129 | 链路请求/响应(含心跳) |
| Client | 2~9 | Connect/Release/Get/Set/Action/Report/Proxy 请求 |
| Server | 130~137 | 各服务响应 + ReportNotification |
| Security | 16 / 144 | 安全请求/响应 |

`Apdu::from_axdr` 按 Link→Client→Server→Security 顺序尝试解析,由标签值分派。

## 技术栈与技术点

| 技术点 | 说明 |
|--------|------|
| Rust 2021 / Cargo workspace | 主库 + 两个子 crate 拆分协议类型与宏 |
| 过程宏(`axdr_macro`) | 基于 `synstructure` 的 `AxdrSequence`/`ToAxdrSequence`/`IntoData` 派生宏,支持 `#[tag]`、`#[optional]`、`#[default]` 属性,自动生成 AXDR 编解码代码 |
| `asn1_type` | 实现 DLT698 的 AXDR 基本类型;错误/解析结果类型复用 `asn1-rs`(asn1_rs::Error 等) |
| `modular-bitfield` | 位域级结构(长度域、控制域、PIID、PIID_ACD)的声明与序列化 |
| 流式解析 | `Frame::parse` 基于 `Cursor` 跳过不匹配起始字节、按长度域判断是否收够一帧,适配 TCP/串口字节流 |
| 分片机制 | FormatDomain(标签 + 12 位序号),接收侧校验序号连续性并合并,发送侧生成 Confirm 分片 |
| 双 FCS16 | 头部(不含起始符)与整帧(去起始符与帧尾)各一次查表校验 |
| 并发安全的序号分配 | PIID 序号用 `AtomicU8` 自增生成 |
| `num_enum` / `strum` / `thiserror` / `anyhow` / `chrono` / `hex` | 枚举转换、错误、时间等基础工具 |

## 关键流程

1. **组帧发送**:构造 `CtrlField + AddressField + UserData(Apdu)` → `Frame::new` 自动计算长度域、头部/整帧校验和 → `into_vec()` 输出字节流。
2. **接收解析**:`Frame::parse` 按 `0x68` 起始符定位 → 依长度域判断完整帧 → `TryFrom<&[u8]>` 校验头部与整帧 FCS16 → 依分片标志解析为 APDU 或分片。
3. **分片重组**:`is_first_fragment` → `combine_fragment` 合并,序号不连续或标签非法时报错;末片到达后 `fragment_transfer` 还原为 APDU。

## 构建与测试

```bash
cargo build          # 构建 workspace
cargo test           # 运行单元测试(含帧/校验和/APDU 编解码用例)
```

`src/frame.rs` 内置了针对真实抓包字节(`68 17 00 ... 16`)的编解码回归测试。

## 依赖关系

- 被 `hplc_manage` 等上层服务以 `path = "../dlt698_rs"` 方式引用。
- 内部依赖:`asn1_type`(AXDR 类型)、`axdr_macro`(派生宏)。
