# Shared-dialog source-description schema

A shared, cross-repository schema for **replayable descriptions of shared AI
dialogs** captured from providers such as ChatGPT and Google AI Mode.

It exists so that web-capture (which emits captures), formal-ai (which stores
them in `demo_memory`), and future tools all agree on how a source is described.
The schema lives here, in meta-language, because the meta-language is a
self-describing links network: the same data can be expressed as a links
network (LiNo) and as JSON, and the two are lossless equivalents.

- **Canonical meta-language form:** the LiNo definition and examples below
  (`*.lino` files in [`examples/`](examples)).
- **Interchange form:** [`shared-dialog.schema.json`](shared-dialog.schema.json)
  (JSON Schema, draft 2020-12) with `*.json` instances in
  [`examples/`](examples). This is what web-capture emits and formal-ai
  consumes.

Both forms are validated in CI: the Rust reference implementation proves every
`*.lino` example round-trips losslessly through `LinkNetwork::parse` /
`reconstruct_text`, and both the Rust and JavaScript test suites validate every
`*.json` instance against this schema.

## Concepts

### `shared_dialog_source`

The capture container. One per shared dialog.

| Field | Required | Description |
| --- | --- | --- |
| `provider` | yes | Originating provider, e.g. `chatgpt`, `google_ai_mode`, `markdown`. |
| `source_url` | yes (nullable) | Canonical URL of the shared dialog. `null` when the source has no URL (e.g. a pasted Markdown transcript). |
| `capture_method` | yes | How the transcript was obtained — see [capture methods](#capture-methods). |
| `capture_status` | yes | Outcome of the capture — see [status values](#status-values). |
| `captured_at` | no | ISO 8601 timestamp of the capture. |
| `conversation_id` | no | Provider-side conversation identifier, when known. |
| `conversation_title` | no | Human-readable conversation title, when known. |
| `turns` | when captured | Ordered `shared_dialog_turn` list. Present and non-empty when `capture_status` is `captured`. |
| `diagnostics` | when not captured | `shared_dialog_capture_diagnostic` list explaining a non-captured or partial result. |

### `shared_dialog_turn`

A single message in the dialog.

| Field | Required | Description |
| --- | --- | --- |
| `turn_id` | yes | Stable identifier of the turn within this capture. |
| `role` | yes | Speaker — `user`, `assistant`, `system`, or `tool`. |
| `content` | yes | Plain-text (or Markdown) message content. |
| `order` | yes | Zero-based position of the turn in the transcript. |
| `visibility` | no | `visible` (shown to the user) or `hidden` (internal reasoning/tool traffic). |
| `source_fragment` | no (nullable) | Raw source slice or selector the turn was extracted from, for replay/debugging. |

### `shared_dialog_capture_diagnostic`

Explanation for an unsuccessful or partial capture.

| Field | Required | Description |
| --- | --- | --- |
| `diagnostic_code` | yes | Machine-readable reason; mirrors a non-captured [status value](#status-values) or a finer-grained provider-specific code. |
| `message` | yes | Human-readable explanation. |
| `evidence` | no (nullable) | Optional raw evidence (HTML snippet, screenshot reference, log excerpt). |

## Status values

`capture_status` is one of:

| Value | Meaning |
| --- | --- |
| `captured` | A transcript was obtained; `turns` is populated. |
| `unsupported_provider_format` | The provider responded but its layout is not (yet) parseable. |
| `provider_challenge` | An anti-automation interstitial / CAPTCHA blocked the capture. |
| `login_required` | The dialog is private and requires authentication. |
| `expired_or_deleted` | The share link no longer resolves to a dialog. |
| `no_transcript_found` | The page loaded but contained no recoverable transcript. |

When `capture_status` is not `captured`, the same value is the natural
`diagnostic_code` for the accompanying diagnostic (providers may add finer codes).

## Capture methods

`capture_method` is one of `static_html` (parse server-rendered HTML),
`browser_automation` (drive a real browser, e.g. Google AI Mode),
`markdown` (a pasted/transcribed Markdown dialog), or `api` (a provider API).

## Examples

Each scenario ships as a JSON instance and an equivalent LiNo file.

| Scenario | JSON | LiNo |
| --- | --- | --- |
| ChatGPT static HTML capture | [`chatgpt-static-html.json`](examples/chatgpt-static-html.json) | [`chatgpt-static-html.lino`](examples/chatgpt-static-html.lino) |
| Google AI Mode browser capture | [`google-ai-mode-capture.json`](examples/google-ai-mode-capture.json) | [`google-ai-mode-capture.lino`](examples/google-ai-mode-capture.lino) |
| Google AI Mode challenge (diagnostic) | [`google-ai-mode-challenge.json`](examples/google-ai-mode-challenge.json) | [`google-ai-mode-challenge.lino`](examples/google-ai-mode-challenge.lino) |
| Plain Markdown transcript | [`markdown-transcript.json`](examples/markdown-transcript.json) | [`markdown-transcript.lino`](examples/markdown-transcript.lino) |
| demo_memory mapping | [`demo-memory-mapping.json`](examples/demo-memory-mapping.json) | — |

### Meta-language (LiNo) form

The schema is a small set of relation links. Concept definitions:

```lino
(shared_dialog_source: provider source_url capture_method capture_status captured_at conversation_id conversation_title turns diagnostics)
(shared_dialog_turn: turn_id role content order visibility source_fragment)
(shared_dialog_capture_diagnostic: diagnostic_code message evidence)
```

Enumerations:

```lino
(capture_status: captured unsupported_provider_format provider_challenge login_required expired_or_deleted no_transcript_found)
(capture_method: static_html browser_automation markdown api)
(role: user assistant system tool)
(visibility: visible hidden)
```

An instance nests field links under the source link; see
[`chatgpt-static-html.lino`](examples/chatgpt-static-html.lino) for a complete
captured dialog and
[`google-ai-mode-challenge.lino`](examples/google-ai-mode-challenge.lino) for a
diagnostic.

## Mapping to formal-ai `demo_memory`

formal-ai stores a captured dialog as a sequence of `demo_memory` events. The
mapping is lossless for the four fields the
[formal-ai issue](https://github.com/link-assistant/formal-ai/issues/552)
requires — source URL, provider, turn role, and turn content:

- One event per `shared_dialog_turn`, in `order`.
- Each event carries the source-level `provider`, `source_url`, and (when
  present) `conversation_id`, plus the turn's `order`, `role`, and `content`.
- `capture_status` other than `captured` produces no turn events; the
  `diagnostics` are recorded so the failure is replayable rather than silent.

[`demo-memory-mapping.json`](examples/demo-memory-mapping.json) is a worked
example pairing a `source` with its derived `events`; the Rust and JavaScript
test suites assert the projection preserves provider, source URL, role, and
content for every turn.

## Emitting the schema from web-capture

web-capture can emit `shared_dialog_source` JSON directly (it matches
[`shared-dialog.schema.json`](shared-dialog.schema.json)) or produce the
equivalent LiNo network. Because the meta-language round-trips LiNo losslessly,
a capturer that emits either form can be converted to the other without loss.
