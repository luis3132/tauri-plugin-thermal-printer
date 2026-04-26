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
 * - `codepage` controls the `ESC t n` command sent to the printer.
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

/** Barcode type options */
export type BarcodeType =
  | 'UPC-A'
  | 'UPC-E'
  | 'EAN13'
  | 'EAN8'
  | 'CODE39'
  | 'ITF'
  | 'CODABAR'
  | 'CODE93'
  | 'CODE128'

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

export interface PrinterOptions {
  /** Append a tail `Cut` section with mode `"partial"` and feed `0`. */
  cut_paper: boolean
  /** Append a tail `Beep` section with times `1` and duration `3`. */
  beep: boolean
  /** Append a tail `Drawer` section with pin `2` and pulse time `100`. */
  open_cash_drawer: boolean
  /** Required ESC/POS page plus host-side encoding strategy. */
  code_page: CodePage
}

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

// ─── Union type ───────────────────────────────────────────────────────────────

export type PrintSections =
  | { Title: Title }
  | { Subtitle: Subtitle }
  | { Text: Text }
  | { Feed: Feed }
  | { Cut: Cut }
  | { Beep: Beep }
  | { Drawer: Drawer }
  | { GlobalStyles: GlobalStyles }
  | { Qr: Qr }
  | { Barcode: Barcode }
  | { Table: Table }
  | { DataMatrix: DataMatrixModel }
  | { Pdf417: Pdf417 }
  | { Image: Image }
  | { Logo: Logo }
  | { Line: Line }

// ─── Request interfaces ───────────────────────────────────────────────────────

export interface PrintJobRequest {
  printer: string
  sections: PrintSections[]
  options: PrinterOptions
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

/** Creates a Beep section */
export function beep(times: number = 1, duration: number = 3): PrintSections {
  return { Beep: { times, duration } }
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

/** Creates a Barcode section */
export function barcode(
  data: string,
  barcode_type: BarcodeType = 'CODE128',
  options?: {
    width?: number
    height?: number
    text_position?: BarcodeTextPosition
    align?: TextAlign
  },
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
