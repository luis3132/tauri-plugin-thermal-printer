import { invoke } from '@tauri-apps/api/core'
import type { Encode } from './encode'
export { ENCODE } from './encode'
export type { Encode } from './encode'

// ─── Paper Size ───────────────────────────────────────────────────────────────

export type PaperSize =
  | 'Mm40'   // 40mm — ~21 chars/line
  | 'Mm44'   // 44mm — ~24 chars/line
  | 'Mm58'   // 58mm — ~32 chars/line (small format)
  | 'Mm72'   // 72mm — ~42 chars/line
  | 'Mm80'   // 80mm — ~48 chars/line (standard large format)
  | 'Mm104'  // 104mm — ~62 chars/line (wide format)

export const PAPER_SIZE_CHARS_PER_LINE: Record<PaperSize, number> = {
  Mm40: 21,
  Mm44: 24,
  Mm58: 32,
  Mm72: 42,
  Mm80: 48,
  Mm104: 62,
}

export const PAPER_SIZE_PIXELS_WIDTH: Record<PaperSize, number> = {
  Mm40: 256,
  Mm44: 288,
  Mm58: 384,
  Mm72: 512,
  Mm80: 576,
  Mm104: 752,
}

export const DEFAULT_PAPER_SIZE: PaperSize = 'Mm80'

export function getPaperSizeCharsPerLine(paperSize: PaperSize): number {
  return PAPER_SIZE_CHARS_PER_LINE[paperSize]
}

export function getPaperSizePixelsWidth(paperSize: PaperSize): number {
  return PAPER_SIZE_PIXELS_WIDTH[paperSize]
}

// ─── Code Page (language/encoding) ───────────────────────────────────────────

/**
 * ESC/POS page selection plus explicit host-side encoding behavior.
 *
 * - `code_page` controls the `ESC t n` command sent to the printer. Default is 0.
 * - `encode` selects the host-side encoding and defaults to
 *   `ENCODE.ACCENT_REMOVER`.
 * - `use_gbk` controls whether unmapped non-ASCII characters may fall back
 *   to GBK. It defaults to `false`.
 */
export interface CodePage {
  code_page: number
  encode?: Encode
  use_gbk?: boolean
}

// ─── Text style constants ─────────────────────────────────────────────────────

/** Text alignment options */
export type TextAlign = 'left' | 'center' | 'right'

/** Text size options */
export type TextSize = 'normal' | 'height' | 'width' | 'double'

/** Font options */
export type TextFont = 'A' | 'B' | 'C'

/** Barcode text position options */
export type BarcodeTextPosition = 'none' | 'above' | 'below' | 'both'

/**
 * Barcode types grouped by the character set (charset) each symbology accepts.
 *
 * Each family below is paired with a branded data type so the compiler forces
 * you to build the payload with the matching helper (`numericBarcodeData`,
 * `code39BarcodeData`, `codabarBarcodeData`, `gs1BarcodeData`) via the typed
 * factories (`numericBarcode`, `code39Barcode`, …). CODE93/CODE128 accept any
 * ASCII string.
 */

/** Numeric-only symbologies (digits `0-9`). Includes UPC/EAN/ITF/GS1 DataBar 1D. */
export type NumericBarcodeType =
  | 'UPC-A'
  | 'UPC-E'
  | 'EAN13'
  | 'EAN8'
  | 'ITF'
  | 'GS1-DATABAR-OMNI'
  | 'GS1-DATABAR-TRUNCATED'
  | 'GS1-DATABAR-LIMITED'

/** CODE39: digits, uppercase `A-Z`, space and `- . $ / + %`. */
export type Code39BarcodeType = 'CODE39'

/** CODABAR: digits, `- $ : / . +` and start/stop `A-D`. */
export type CodabarBarcodeType = 'CODABAR'

/** Full-ASCII symbologies (accept any ASCII character). */
export type AsciiBarcodeType = 'CODE93' | 'CODE128'

/** GS1 symbologies whose data uses GS1 AI format (printable ASCII). */
export type Gs1BarcodeType = 'GS1-128' | 'GS1-DATABAR-EXPANDED'

/** Barcode type options (union of every charset family). */
export type BarcodeType =
  | NumericBarcodeType
  | Code39BarcodeType
  | CodabarBarcodeType
  | AsciiBarcodeType
  | Gs1BarcodeType

// ─── Branded data types per charset family ────────────────────────────────────
// Force, at compile time, that the payload is built for the right symbology.
// The Rust side keeps `data: string`, so leading zeros are preserved.

/** Payload for a {@link NumericBarcodeType}. Build with {@link numericBarcodeData}. */
export type NumericBarcodeData = string & { readonly __barcodeCharset: 'numeric' }
/** Payload for a {@link Code39BarcodeType}. Build with {@link code39BarcodeData}. */
export type Code39BarcodeData = string & { readonly __barcodeCharset: 'code39' }
/** Payload for a {@link CodabarBarcodeType}. Build with {@link codabarBarcodeData}. */
export type CodabarBarcodeData = string & { readonly __barcodeCharset: 'codabar' }
/** Payload for a {@link Gs1BarcodeType}. Build with {@link gs1BarcodeData}. */
export type Gs1BarcodeData = string & { readonly __barcodeCharset: 'gs1' }

/**
 * Brand a string as numeric-barcode data (digits only, leading zeros kept).
 * Pure compile-time cast — no runtime validation.
 */
export const numericBarcodeData = (data: string): NumericBarcodeData => data as NumericBarcodeData
/** Brand a string as CODE39 data. Pure compile-time cast. */
export const code39BarcodeData = (data: string): Code39BarcodeData => data as Code39BarcodeData
/** Brand a string as CODABAR data. Pure compile-time cast. */
export const codabarBarcodeData = (data: string): CodabarBarcodeData => data as CodabarBarcodeData
/** Brand a string as GS1 (AI format) data. Pure compile-time cast. */
export const gs1BarcodeData = (data: string): Gs1BarcodeData => data as Gs1BarcodeData

/** QR error correction level */
export type QrErrorCorrection = 'L' | 'M' | 'Q' | 'H'

/** Image size mode */
export type ImageMode = 'normal' | 'double_width' | 'double_height' | 'quadruple'

/** Feed type */
export type FeedType = 'lines' | 'dots' | 'line_feed'

/** Cut mode */
export type CutMode = 'full' | 'partial' | 'partial_alt' | 'partial_alt2'

// ─── Convenience style presets ────────────────────────────────────────────────

export const TEXT_ALIGN = {
  LEFT: 'left' as TextAlign,
  CENTER: 'center' as TextAlign,
  RIGHT: 'right' as TextAlign,
} as const

export const TEXT_SIZE = {
  NORMAL: 'normal' as TextSize,
  HEIGHT: 'height' as TextSize,
  WIDTH: 'width' as TextSize,
  DOUBLE: 'double' as TextSize,
} as const

export const TEXT_FONT = {
  A: 'A' as TextFont,
  B: 'B' as TextFont,
  C: 'C' as TextFont,
} as const

export const BARCODE_TYPE = {
  UPC_A: 'UPC-A' as BarcodeType,
  UPC_E: 'UPC-E' as BarcodeType,
  EAN13: 'EAN13' as BarcodeType,
  EAN8: 'EAN8' as BarcodeType,
  CODE39: 'CODE39' as BarcodeType,
  ITF: 'ITF' as BarcodeType,
  CODABAR: 'CODABAR' as BarcodeType,
  CODE93: 'CODE93' as BarcodeType,
  CODE128: 'CODE128' as BarcodeType,
  /** GS1-128 (EAN-128). Data may use GS1 AI format with FNC1. */
  GS1_128: 'GS1-128' as BarcodeType,
  /** GS1 DataBar Omnidirectional — 13 numeric digits. */
  GS1_DATABAR_OMNI: 'GS1-DATABAR-OMNI' as BarcodeType,
  /** GS1 DataBar Truncated — 13 numeric digits. */
  GS1_DATABAR_TRUNCATED: 'GS1-DATABAR-TRUNCATED' as BarcodeType,
  /** GS1 DataBar Limited — 13 numeric digits (first digit 0 or 1). */
  GS1_DATABAR_LIMITED: 'GS1-DATABAR-LIMITED' as BarcodeType,
  /** GS1 DataBar Expanded — variable GS1 AI data. */
  GS1_DATABAR_EXPANDED: 'GS1-DATABAR-EXPANDED' as BarcodeType,
} as const

export const BARCODE_TEXT_POSITION = {
  NONE: 'none' as BarcodeTextPosition,
  ABOVE: 'above' as BarcodeTextPosition,
  BELOW: 'below' as BarcodeTextPosition,
  BOTH: 'both' as BarcodeTextPosition,
} as const

export const QR_ERROR_CORRECTION = {
  /** Low — 7% recovery */
  L: 'L' as QrErrorCorrection,
  /** Medium — 15% recovery (recommended default) */
  M: 'M' as QrErrorCorrection,
  /** Quartile — 25% recovery */
  Q: 'Q' as QrErrorCorrection,
  /** High — 30% recovery */
  H: 'H' as QrErrorCorrection,
} as const

export const IMAGE_MODE = {
  NORMAL: 'normal' as ImageMode,
  DOUBLE_WIDTH: 'double_width' as ImageMode,
  DOUBLE_HEIGHT: 'double_height' as ImageMode,
  QUADRUPLE: 'quadruple' as ImageMode,
} as const

export const CUT_MODE = {
  FULL: 'full' as CutMode,
  PARTIAL: 'partial' as CutMode,
} as const

// ─── Core interfaces ──────────────────────────────────────────────────────────



export interface GlobalStyles {
  bold?: boolean
  underline?: boolean
  align?: TextAlign
  italic?: boolean
  invert?: boolean
  font?: TextFont
  rotate?: boolean
  upside_down?: boolean
  size?: TextSize
  /** Double-strike / double-print (`ESC G`). Reinforces bold on generic printers. */
  double_strike?: boolean
  /**
   * When `true`, resets the printer (`ESC @`) and re-applies the code page.
   * Takes priority: all other style fields in the same section are ignored.
   */
  reset?: boolean
}

// ─── Print section interfaces ─────────────────────────────────────────────────

export interface Title {
  text: string
  styles?: GlobalStyles
}

export interface Subtitle {
  text: string
  styles?: GlobalStyles
}

export interface Text {
  text: string
  styles?: GlobalStyles
}

export interface Feed {
  feed_type: FeedType
  value: number
}

export interface Cut {
  mode: CutMode
  feed: number
}

export interface Beep {
  /** Number of beeps (1–9) */
  times: number
  /** Duration per beep in ms (1–255) */
  duration: number
}

export interface Drawer {
  pin: 2 | 5
  pulse_time: number
}

export interface Table {
  columns: number
  column_widths?: number[]
  header?: Text[]
  body: Text[][]
  truncate: boolean
}

export interface Qr {
  data: string
  /** Module size 1–16 (default 6) */
  size: number
  error_correction: QrErrorCorrection
  model: 1 | 2
  align?: TextAlign
}

export interface Barcode {
  data: string
  barcode_type: BarcodeType
  /** Bar width 2–6 */
  width: number
  /** Bar height in dots (1–255) */
  height: number
  text_position: BarcodeTextPosition
  align?: TextAlign
}

export interface DataMatrixModel {
  data: string
  /** Module size 1–16 */
  size: number
}

export interface Pdf417 {
  data: string
  /** Columns 0 (auto) or 1–30 */
  columns: number
  /** Rows 0 (auto) or 3–90 */
  rows: number
  /** Module width 2–8 */
  width: number
  /** Row height 2–8 */
  height: number
  /** Error correction level 0–8 */
  error_correction: number
}

/**
 * Aztec Code (2D). Requires printer firmware support (advanced Epson models).
 */
export interface Aztec {
  data: string
  /** 0 = full range, 1 = compact */
  mode: number
  /** Number of data layers, 0 = automatic (1–32) */
  layers: number
  /** Module size 2–16 */
  size: number
  /** Error correction as a percentage of capacity (5–95) */
  error_correction: number
  align?: TextAlign
}

/** GS1 DataBar 2D subtype. */
export type Gs1Databar2dType = 'STACKED' | 'STACKED-OMNI' | 'EXPANDED-STACKED'

/**
 * 2D GS1 DataBar (Stacked / Stacked Omnidirectional / Expanded Stacked).
 * Requires printer firmware support (advanced Epson models).
 */
export interface Gs1Databar2d {
  data: string
  databar_type: Gs1Databar2dType
  /** Module width 2–8 */
  width: number
  align?: TextAlign
}

/**
 * MaxiCode (2D). Fixed physical size. Requires printer firmware support
 * (advanced Epson models).
 */
export interface MaxiCode {
  data: string
  /** Mode 2–6 (4 is the general-purpose default) */
  mode: number
  align?: TextAlign
}

/**
 * Composite Symbology (GS1 Composite CC-A/CC-B/CC-C). Requires printer firmware
 * support (advanced Epson models).
 */
export interface Composite {
  data: string
  /**
   * The `m` parameter selecting the 1D host + 2D component. Its exact value is
   * printer/spec specific — check your device's ESC/POS reference.
   */
  symbol_type: number
  /** Module width 2–8 */
  width: number
  align?: TextAlign
}

export interface Image {
  /** Base64 encoded image (with or without data URI prefix) */
  data: string
  /** Max width in pixels. 0 = use full paper width. */
  max_width: number
  align: TextAlign
  dithering: boolean
  size: ImageMode
}

export interface Logo {
  /** NV memory key code (1–255) */
  key_code: number
  mode: ImageMode
}

export interface Line {
  /** Single character to repeat across the paper width (default '-') */
  character: string
}

export interface LineSpacing {
  /** Spacing in dots. Omit / `null` resets to the printer default (~1/6"). */
  value?: number | null
}

export interface CharSpacing {
  /** Extra spacing to the right of each character, in dots (0–255). */
  value: number
}

export interface Position {
  /** Absolute horizontal position for the next data, in dots from the left margin. */
  value: number
}

export interface TabStops {
  /** Tab stop columns (in characters), ascending. Up to 32. Emit `\t` in text to jump. */
  positions: number[]
}

export interface LeftMargin {
  /** Left margin in dots (standard mode). */
  value: number
}

export interface PrintAreaWidth {
  /** Printable area width in dots (standard mode). */
  value: number
}

// ─── Union type ───────────────────────────────────────────────────────────────

export type PrintSections =
  | { Title: Title }
  | { Subtitle: Subtitle }
  | { Text: Text }
  | { Feed: Feed }
  | { Cut: Cut }
  | { Beep: Beep }
  | { Beep2: Beep }
  | { Drawer: Drawer }
  | { GlobalStyles: GlobalStyles }
  | { Qr: Qr }
  | { Barcode: Barcode }
  | { Table: Table }
  | { DataMatrix: DataMatrixModel }
  | { Pdf417: Pdf417 }
  | { Aztec: Aztec }
  | { Gs1Databar2d: Gs1Databar2d }
  | { MaxiCode: MaxiCode }
  | { Composite: Composite }
  | { Image: Image }
  | { Logo: Logo }
  | { Line: Line }
  | { LineSpacing: LineSpacing }
  | { CharSpacing: CharSpacing }
  | { Position: Position }
  | { TabStops: TabStops }
  | { LeftMargin: LeftMargin }
  | { PrintAreaWidth: PrintAreaWidth }

// ─── Request interfaces ───────────────────────────────────────────────────────

export interface PrintJobRequest {
  printer: string
  sections: PrintSections[]
  options: CodePage
  paper_size: PaperSize
}

export interface PrinterInfo {
  name: string
  interface_type: string
  identifier: string
  status: string
}

export interface TestPrintRequest {
  printer_info: PrintJobRequest
  include_text?: boolean
  include_custom_text?: boolean
  custom_text?: string | null
  include_text_styles?: boolean
  include_alignment?: boolean
  include_columns?: boolean
  include_separators?: boolean
  include_barcode?: boolean
  include_barcode_types?: boolean
  include_qr?: boolean
  include_image?: boolean
  image_base64?: string | null
  include_beep?: boolean
  test_cash_drawer?: boolean
  cut_paper?: boolean
  test_feed?: boolean
  test_all_fonts?: boolean
  test_invert?: boolean
  test_rotate?: boolean
  /** Double-strike (`ESC G`) demo line in the text-styles section. */
  test_double_strike?: boolean
  /** Line spacing (`ESC 3`/`ESC 2`) + character spacing (`ESC SP`) demo. */
  test_spacing?: boolean
  /** Tab stops (`ESC D`), absolute position (`ESC $`) and margins (`GS L`/`GS W`) demo. */
  test_positioning?: boolean
  /** Generic buzzer (`ESC B`) — for printers that ignore the Epson beep. */
  test_beep2?: boolean
}

// ─── Helper builders ──────────────────────────────────────────────────────────

/** Creates a Title section */
export function title(text: string, styles?: GlobalStyles): PrintSections {
  return { Title: { text, styles } }
}

/** Creates a Subtitle section */
export function subtitle(text: string, styles?: GlobalStyles): PrintSections {
  return { Subtitle: { text, styles } }
}

/** Creates a Text section */
export function text(text: string, styles?: GlobalStyles): PrintSections {
  return { Text: { text, styles } }
}

/** Creates a Line separator section */
export function line(character: string = '-'): PrintSections {
  return { Line: { character } }
}

/** Creates a Feed section */
export function feed(value: number, feed_type: FeedType = 'lines'): PrintSections {
  return { Feed: { feed_type, value } }
}

/** Creates a Cut section */
export function cut(mode: CutMode = 'partial', feedLines: number = 4): PrintSections {
  return { Cut: { mode, feed: feedLines } }
}

/** Creates a GlobalStyles section — affects all subsequent sections until changed */
export function globalStyles(styles: GlobalStyles): PrintSections {
  return { GlobalStyles: styles }
}

/**
 * Creates a reset section: initializes the printer (`ESC @`) and re-applies the
 * code page. Shorthand for `globalStyles({ reset: true })`.
 */
export function reset(): PrintSections {
  return { GlobalStyles: { reset: true } }
}

/** Creates a Beep section (Epson `ESC ( A`) */
export function beep(times: number = 1, duration: number = 3): PrintSections {
  return { Beep: { times, duration } }
}

/** Creates a Beep2 section — generic buzzer (`ESC B n t`) for printers that ignore `beep()` */
export function beep2(times: number = 1, duration: number = 3): PrintSections {
  return { Beep2: { times, duration } }
}

/** Creates a LineSpacing section. Omit `value` to reset to the printer default (`ESC 2`). */
export function lineSpacing(value?: number): PrintSections {
  return { LineSpacing: { value: value ?? null } }
}

/** Creates a CharSpacing section (`ESC SP n`) */
export function charSpacing(value: number): PrintSections {
  return { CharSpacing: { value } }
}

/** Creates a Position section — absolute horizontal position in dots (`ESC $`) */
export function position(value: number): PrintSections {
  return { Position: { value } }
}

/** Creates a TabStops section (`ESC D`). Emit `\t` in text to jump to the next stop. */
export function tabStops(positions: number[]): PrintSections {
  return { TabStops: { positions } }
}

/** Creates a LeftMargin section — left margin in dots (`GS L`) */
export function leftMargin(value: number): PrintSections {
  return { LeftMargin: { value } }
}

/** Creates a PrintAreaWidth section — printable width in dots (`GS W`) */
export function printAreaWidth(value: number): PrintSections {
  return { PrintAreaWidth: { value } }
}

/** Creates a Drawer section */
export function drawer(pin: 2 | 5 = 2, pulse_time: number = 120): PrintSections {
  return { Drawer: { pin, pulse_time } }
}

/** Creates a Table section */
export function table(
  columns: number,
  body: Text[][],
  options?: {
    column_widths?: number[]
    header?: Text[]
    truncate?: boolean
  },
): PrintSections {
  return {
    Table: {
      columns,
      body,
      column_widths: options?.column_widths,
      header: options?.header,
      truncate: options?.truncate ?? true,
    },
  }
}

/** Creates a QR section */
export function qr(
  data: string,
  options?: {
    size?: number
    error_correction?: QrErrorCorrection
    model?: 1 | 2
    align?: TextAlign
  },
): PrintSections {
  return {
    Qr: {
      data,
      size: options?.size ?? 6,
      error_correction: options?.error_correction ?? 'M',
      model: options?.model ?? 2,
      align: options?.align,
    },
  }
}

/** Options shared by every barcode factory. */
export interface BarcodeOptions {
  width?: number
  height?: number
  text_position?: BarcodeTextPosition
  align?: TextAlign
}

function makeBarcodeSection(
  data: string,
  barcode_type: BarcodeType,
  options?: BarcodeOptions,
): PrintSections {
  return {
    Barcode: {
      data,
      barcode_type,
      width: options?.width ?? 3,
      height: options?.height ?? 80,
      text_position: options?.text_position ?? 'below',
      align: options?.align,
    },
  }
}

/**
 * Creates a Barcode section (untyped data).
 *
 * @deprecated Use the charset-typed factories instead: {@link numericBarcode},
 * {@link code39Barcode}, {@link codabarBarcode}, {@link asciiBarcode} or
 * {@link gs1Barcode}. They force, at compile time, that the payload matches the
 * character set the symbology accepts. This function stays for backwards
 * compatibility and accepts any `string`.
 */
export function barcode(
  data: string,
  barcode_type: BarcodeType = 'CODE128',
  options?: BarcodeOptions,
): PrintSections {
  return makeBarcodeSection(data, barcode_type, options)
}

/**
 * Creates a numeric barcode (UPC/EAN/ITF/GS1 DataBar 1D).
 * Build `data` with {@link numericBarcodeData} — digits only, leading zeros kept.
 */
export function numericBarcode(
  data: NumericBarcodeData,
  barcode_type: NumericBarcodeType,
  options?: BarcodeOptions,
): PrintSections {
  return makeBarcodeSection(data, barcode_type, options)
}

/**
 * Creates a CODE39 barcode.
 * Build `data` with {@link code39BarcodeData} — digits, uppercase A-Z and `- . $ / + %`.
 */
export function code39Barcode(data: Code39BarcodeData, options?: BarcodeOptions): PrintSections {
  return makeBarcodeSection(data, 'CODE39', options)
}

/**
 * Creates a CODABAR barcode.
 * Build `data` with {@link codabarBarcodeData} — digits, `- $ : / . +` and start/stop A-D.
 */
export function codabarBarcode(data: CodabarBarcodeData, options?: BarcodeOptions): PrintSections {
  return makeBarcodeSection(data, 'CODABAR', options)
}

/**
 * Creates a full-ASCII barcode (CODE93 / CODE128). Accepts any ASCII `string`.
 */
export function asciiBarcode(
  data: string,
  barcode_type: AsciiBarcodeType = 'CODE128',
  options?: BarcodeOptions,
): PrintSections {
  return makeBarcodeSection(data, barcode_type, options)
}

/**
 * Creates a GS1 barcode (GS1-128 / GS1 DataBar Expanded).
 * Build `data` with {@link gs1BarcodeData} — GS1 AI format (printable ASCII).
 */
export function gs1Barcode(
  data: Gs1BarcodeData,
  barcode_type: Gs1BarcodeType,
  options?: BarcodeOptions,
): PrintSections {
  return makeBarcodeSection(data, barcode_type, options)
}

/** Creates a DataMatrix section */
export function dataMatrix(data: string, size: number = 6): PrintSections {
  return {
    DataMatrix: {
      data,
      size,
    },
  }
}

/** Creates a PDF417 section */
export function pdf417(
  data: string,
  options?: {
    columns?: number
    rows?: number
    width?: number
    height?: number
    error_correction?: number
  },
): PrintSections {
  return {
    Pdf417: {
      data,
      columns: options?.columns ?? 0,
      rows: options?.rows ?? 0,
      width: options?.width ?? 2,
      height: options?.height ?? 3,
      error_correction: options?.error_correction ?? 2,
    },
  }
}

/**
 * Creates an Aztec Code section.
 * Requires printer firmware support (advanced Epson models).
 */
export function aztec(
  data: string,
  options?: {
    mode?: number
    layers?: number
    size?: number
    error_correction?: number
    align?: TextAlign
  },
): PrintSections {
  return {
    Aztec: {
      data,
      mode: options?.mode ?? 0,
      layers: options?.layers ?? 0,
      size: options?.size ?? 3,
      error_correction: options?.error_correction ?? 23,
      align: options?.align,
    },
  }
}

/**
 * Creates a 2D GS1 DataBar section.
 * Requires printer firmware support (advanced Epson models).
 */
export function gs1Databar2d(
  data: string,
  databar_type: Gs1Databar2dType = 'STACKED-OMNI',
  options?: {
    width?: number
    align?: TextAlign
  },
): PrintSections {
  return {
    Gs1Databar2d: {
      data,
      databar_type,
      width: options?.width ?? 2,
      align: options?.align,
    },
  }
}

/**
 * Creates a MaxiCode section (mode 2–6, default 4).
 * Requires printer firmware support (advanced Epson models).
 */
export function maxicode(
  data: string,
  options?: {
    mode?: number
    align?: TextAlign
  },
): PrintSections {
  return {
    MaxiCode: {
      data,
      mode: options?.mode ?? 4,
      align: options?.align,
    },
  }
}

/**
 * Creates a Composite Symbology section.
 * Requires printer firmware support (advanced Epson models). The `symbol_type`
 * (`m`) value is printer/spec specific — check your device's ESC/POS reference.
 */
export function composite(
  data: string,
  options?: {
    symbol_type?: number
    width?: number
    align?: TextAlign
  },
): PrintSections {
  return {
    Composite: {
      data,
      symbol_type: options?.symbol_type ?? 48,
      width: options?.width ?? 2,
      align: options?.align,
    },
  }
}

/** Creates an Image section */
export function image(
  data: string,
  options?: {
    max_width?: number
    align?: TextAlign
    dithering?: boolean
    size?: ImageMode
  },
): PrintSections {
  return {
    Image: {
      data,
      max_width: options?.max_width ?? 0,
      align: options?.align ?? 'center',
      dithering: options?.dithering ?? true,
      size: options?.size ?? 'normal',
    },
  }
}

/** Creates a Logo section */
export function logo(key_code: number, mode: ImageMode = 'normal'): PrintSections {
  return {
    Logo: {
      key_code,
      mode,
    },
  }
}

// ─── Commands ─────────────────────────────────────────────────────────────────

/**
 * Sends a print job to the specified thermal printer.
 * @throws {string} Error message from the printer or document generation if the job fails.
 */
export async function print_thermal_printer(printJobRequest: PrintJobRequest): Promise<void> {
  await invoke('plugin:thermal-printer|print_thermal_printer', {
    printJobRequest,
  })
}

/**
 * Returns the list of available thermal printers on the current system.
 * @throws {string} Error message if printer enumeration fails.
 */
export async function list_thermal_printers(): Promise<PrinterInfo[]> {
  return await invoke<PrinterInfo[]>('plugin:thermal-printer|list_thermal_printers')
}

/**
 * Sends a test print job to verify the printer is working correctly.
 * @throws {string} Error message from the printer or document generation if the job fails.
 */
export async function test_thermal_printer(testPrintRequest: TestPrintRequest): Promise<void> {
  await invoke('plugin:thermal-printer|test_thermal_printer', {
    printTestRequest: testPrintRequest,
  })
}
